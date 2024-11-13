//! Farmer-specific piece getter

use crate::cluster::cache::{ClusterCacheReadPieceRequest, ClusterCacheReadPiecesRequest};
use crate::cluster::controller::{
    ClusterControllerFindPieceInCacheRequest, ClusterControllerFindPiecesInCacheRequest,
    ClusterControllerPieceRequest, ClusterControllerPiecesRequest,
};
use crate::cluster::nats_client::NatsClient;
use crate::farm::plotted_pieces::PlottedPieces;
use crate::farm::{PieceCacheId, PieceCacheOffset};
use crate::farmer_cache::FarmerCache;
use crate::node_client::NodeClient;
use async_lock::RwLock as AsyncRwLock;
use async_trait::async_trait;
use futures::channel::mpsc;
use futures::future::FusedFuture;
use futures::stream::FuturesUnordered;
use futures::{stream, FutureExt, Stream, StreamExt};
use parking_lot::Mutex;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::pin::Pin;
use std::sync::{Arc, Weak};
use std::task::{Context, Poll};
use subspace_core_primitives::pieces::{Piece, PieceIndex};
use subspace_farmer_components::PieceGetter;
use subspace_networking::utils::multihash::ToMultihash;
use subspace_networking::utils::piece_provider::{PieceProvider, PieceValidator};
use tracing::{debug, error, info, trace, warn};

pub mod piece_validator;

const MAX_RANDOM_WALK_ROUNDS: usize = 15;

struct Inner<FarmIndex, PV, NC> {
    nats_client: NatsClient,
    piece_provider: PieceProvider<PV>,
    farmer_cache: FarmerCache,
    node_client: NC,
    plotted_pieces: Arc<AsyncRwLock<PlottedPieces<FarmIndex>>>,
}

/// Farmer-specific piece getter.
///
/// Implements [`PieceGetter`] for plotting purposes, but useful outside of that as well.
pub struct RemotePieceGetter<FarmIndex, PV, NC> {
    inner: Arc<Inner<FarmIndex, PV, NC>>,
}

impl<FarmIndex, PV, NC> fmt::Debug for RemotePieceGetter<FarmIndex, PV, NC> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FarmerPieceGetter").finish_non_exhaustive()
    }
}

impl<FarmIndex, PV, NC> Clone for RemotePieceGetter<FarmIndex, PV, NC> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<FarmIndex, PV, NC> RemotePieceGetter<FarmIndex, PV, NC>
where
    FarmIndex: Hash + Eq + Copy + fmt::Debug + Send + Sync + 'static,
    usize: From<FarmIndex>,
    PV: PieceValidator + Send + 'static,
    NC: NodeClient,
{
    /// Create new instance
    pub fn new(
        nats_client: NatsClient,
        piece_provider: PieceProvider<PV>,
        farmer_cache: FarmerCache,
        node_client: NC,
        plotted_pieces: Arc<AsyncRwLock<PlottedPieces<FarmIndex>>>,
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                nats_client,
                piece_provider,
                farmer_cache,
                node_client,
                plotted_pieces,
            }),
        }
    }

    /// Fast way to get piece using various caches
    pub async fn get_piece_fast(&self, piece_index: PieceIndex) -> Option<Piece> {
        self.get_piece_fast_internal(piece_index).await
    }

    async fn get_piece_fast_internal(&self, piece_index: PieceIndex) -> Option<Piece> {
        let inner = &self.inner;

        trace!(%piece_index, "Getting piece from farmer cache");
        if let Some(piece) = inner
            .farmer_cache
            .get_piece(piece_index.to_multihash())
            .await
        {
            trace!(%piece_index, "Got piece from farmer cache successfully");
            return Some(piece);
        }

        // L2 piece acquisition
        trace!(%piece_index, "Getting piece from DSN L2 cache");
        if let Some(piece) = inner.piece_provider.get_piece_from_cache(piece_index).await {
            trace!(%piece_index, "Got piece from DSN L2 cache");
            inner
                .farmer_cache
                .maybe_store_additional_piece(piece_index, &piece)
                .await;
            return Some(piece);
        }

        // Try node's RPC before reaching to L1 (archival storage on DSN)
        trace!(%piece_index, "Getting piece from node");
        match inner.node_client.piece(piece_index).await {
            Ok(Some(piece)) => {
                trace!(%piece_index, "Got piece from node successfully");
                inner
                    .farmer_cache
                    .maybe_store_additional_piece(piece_index, &piece)
                    .await;
                return Some(piece);
            }
            Ok(None) => {
                // Nothing to do
            }
            Err(error) => {
                error!(
                    %error,
                    %piece_index,
                    "Failed to retrieve first segment piece from node"
                );
            }
        }

        None
    }

    /// Slow way to get piece using archival storage
    pub async fn get_piece_slow(&self, piece_index: PieceIndex) -> Option<Piece> {
        self.get_piece_slow_internal(piece_index).await
    }

    /// Slow way to get piece using archival storage
    async fn get_piece_slow_internal(&self, piece_index: PieceIndex) -> Option<Piece> {
        let inner = &self.inner;

        trace!(%piece_index, "Getting piece from local plot");
        let maybe_read_piece_fut = inner
            .plotted_pieces
            .try_read()
            .and_then(|plotted_pieces| plotted_pieces.read_piece(piece_index));

        if let Some(read_piece_fut) = maybe_read_piece_fut {
            if let Some(piece) = read_piece_fut.await {
                trace!(%piece_index, "Got piece from local plot successfully");
                inner
                    .farmer_cache
                    .maybe_store_additional_piece(piece_index, &piece)
                    .await;
                return Some(piece);
            }
        }

        // L1 piece acquisition
        trace!(%piece_index, "Getting piece from DSN L1.");

        let archival_storage_search_result = inner
            .piece_provider
            .get_piece_from_archival_storage(piece_index, MAX_RANDOM_WALK_ROUNDS)
            .await;

        if let Some(piece) = archival_storage_search_result {
            trace!(%piece_index, "DSN L1 lookup succeeded");
            inner
                .farmer_cache
                .maybe_store_additional_piece(piece_index, &piece)
                .await;
            return Some(piece);
        }

        None
    }

    /// Downgrade to [`WeakFarmerPieceGetter`] in order to break reference cycles with internally
    /// used [`Arc`]
    pub fn downgrade(&self) -> WeakFarmerPieceGetter<FarmIndex, PV, NC> {
        WeakFarmerPieceGetter {
            inner: Arc::downgrade(&self.inner),
        }
    }
}

#[async_trait]
impl<FarmIndex, PV, NC> PieceGetter for RemotePieceGetter<FarmIndex, PV, NC>
where
    FarmIndex: Hash + Eq + Copy + fmt::Debug + Send + Sync + 'static,
    usize: From<FarmIndex>,
    PV: PieceValidator + Send + 'static,
    NC: NodeClient,
{
    async fn get_piece(&self, piece_index: PieceIndex) -> anyhow::Result<Option<Piece>> {
        if let Some((piece_cache_id, piece_cache_offset)) = self
            .inner
            .nats_client
            .request(
                &ClusterControllerFindPieceInCacheRequest { piece_index },
                None,
            )
            .await?
        {
            trace!(
                %piece_index,
                %piece_cache_id,
                %piece_cache_offset,
                "Found piece in cache, retrieving"
            );

            match self
                .inner
                .nats_client
                .request(
                    &ClusterCacheReadPieceRequest {
                        offset: piece_cache_offset,
                    },
                    Some(&piece_cache_id.to_string()),
                )
                .await
                .map_err(|error| error.to_string())
                .flatten()
            {
                Ok(Some((retrieved_piece_index, piece))) => {
                    if retrieved_piece_index == piece_index {
                        trace!(
                            %piece_index,
                            %piece_cache_id,
                            %piece_cache_offset,
                            "Retrieved piece from cache successfully"
                        );

                        self.inner
                            .farmer_cache
                            .maybe_store_additional_piece(piece_index, &piece)
                            .await;

                        return Ok(Some(piece));
                    } else {
                        trace!(
                            %piece_index,
                            %piece_cache_id,
                            %piece_cache_offset,
                            "Retrieving piece was replaced in cache during retrieval"
                        );
                    }
                }
                Ok(None) => {
                    trace!(
                        %piece_index,
                        %piece_cache_id,
                        %piece_cache_offset,
                        "Piece cache didn't have piece at offset"
                    );
                }
                Err(error) => {
                    debug!(
                        %piece_index,
                        %piece_cache_id,
                        %piece_cache_offset,
                        %error,
                        "Retrieving piece from cache failed"
                    );
                }
            }
        } else {
            trace!(%piece_index, "Piece not found in cache");
        }

        Ok(self
            .inner
            .nats_client
            .request(&ClusterControllerPieceRequest { piece_index }, None)
            .await?)
    }

    async fn get_pieces<'a, PieceIndices>(
        &'a self,
        piece_indices: PieceIndices,
    ) -> anyhow::Result<
        Box<dyn Stream<Item = (PieceIndex, anyhow::Result<Option<Piece>>)> + Send + Unpin + 'a>,
    >
    where
        PieceIndices: IntoIterator<Item = PieceIndex, IntoIter: Send> + Send + 'a,
    {
        let (tx, mut rx) = mpsc::unbounded();
        let mut pieces_from_cache = 0;
        let mut pieces_not_found_in_farmer_cache = Vec::new();
        {
            let tx = &tx;
            let mut pieces_in_farmer_cache =
                self.inner.farmer_cache.get_pieces(piece_indices).await;
            while let Some((piece_index, maybe_piece)) = pieces_in_farmer_cache.next().await {
                let Some(piece) = maybe_piece else {
                    pieces_not_found_in_farmer_cache.push(piece_index);
                    continue;
                };
                pieces_from_cache += 1;
                tx.unbounded_send((piece_index, Ok(Some(piece))))
                    .expect("This future isn't polled after receiver is dropped; qed");
            }
        }

        info!(
            pieces_found_in_farmer_cache = %pieces_from_cache,
            "get pieces from farmer cache"
        );

        info!(
            async_nets = %pieces_not_found_in_farmer_cache.len(),
            "getting pieces from "
        );

        let piece_indices = pieces_not_found_in_farmer_cache
            .into_iter()
            .collect::<Vec<_>>();
        let piece_indices_to_get =
            Mutex::new(piece_indices.iter().copied().collect::<HashSet<_>>());

        let mut cached_pieces_by_cache_id = HashMap::<PieceCacheId, Vec<PieceCacheOffset>>::new();

        {
            let mut cached_pieces = self
                .inner
                .nats_client
                .stream_request(
                    &ClusterControllerFindPiecesInCacheRequest { piece_indices },
                    None,
                )
                .await?;

            while let Some((_piece_index, piece_cache_id, piece_cache_offset)) =
                cached_pieces.next().await
            {
                cached_pieces_by_cache_id
                    .entry(piece_cache_id)
                    .or_default()
                    .push(piece_cache_offset);
            }
        }

        let fut = async move {
            let tx = &tx;

            cached_pieces_by_cache_id
                .into_iter()
                .map(|(piece_cache_id, offsets)| {
                    let piece_indices_to_get = &piece_indices_to_get;

                    async move {
                        let mut pieces_stream = match self
                            .inner
                            .nats_client
                            .stream_request(
                                &ClusterCacheReadPiecesRequest { offsets },
                                Some(&piece_cache_id.to_string()),
                            )
                            .await
                        {
                            Ok(pieces) => pieces,
                            Err(error) => {
                                warn!(
                                    %error,
                                    %piece_cache_id,
                                    "Failed to request pieces from cache"
                                );

                                return;
                            }
                        };

                        while let Some(piece_result) = pieces_stream.next().await {
                            let (piece_offset, maybe_piece) = match piece_result {
                                Ok(result) => result,
                                Err(error) => {
                                    warn!(%error, "Failed to get piece from cache");
                                    continue;
                                }
                            };

                            if let Some((piece_index, piece)) = maybe_piece {
                                piece_indices_to_get.lock().remove(&piece_index);

                                // TODO: Would be nice to have concurrency here
                                self.inner
                                    .farmer_cache
                                    .maybe_store_additional_piece(piece_index, &piece)
                                    .await;

                                tx.unbounded_send((piece_index, Ok(Some(piece)))).expect(
                                    "This future isn't polled after receiver is dropped; qed",
                                );
                            } else {
                                warn!(
                                    %piece_cache_id,
                                    %piece_offset,
                                    "Failed to get piece from cache, it was missing or already gone"
                                );
                            }
                        }
                    }
                })
                .collect::<FuturesUnordered<_>>()
                // Simply drain everything
                .for_each(|()| async {})
                .await;

            let mut piece_indices_to_get = piece_indices_to_get.into_inner();
            if piece_indices_to_get.is_empty() {
                return;
            }

            let mut pieces_from_controller = match self
                .inner
                .nats_client
                .stream_request(
                    &ClusterControllerPiecesRequest {
                        piece_indices: piece_indices_to_get.iter().copied().collect(),
                    },
                    None,
                )
                .await
            {
                Ok(pieces_from_controller) => pieces_from_controller,
                Err(error) => {
                    error!(%error, "Failed to get pieces from controller");

                    for piece_index in piece_indices_to_get {
                        tx.unbounded_send((
                            piece_index,
                            Err(anyhow::anyhow!("Failed to get piece from controller")),
                        ))
                        .expect("This future isn't polled after receiver is dropped; qed");
                    }
                    return;
                }
            };

            while let Some((piece_index, piece)) = pieces_from_controller.next().await {
                piece_indices_to_get.remove(&piece_index);
                tx.unbounded_send((piece_index, Ok(Some(piece))))
                    .expect("This future isn't polled after receiver is dropped; qed");
            }

            for piece_index in piece_indices_to_get {
                tx.unbounded_send((piece_index, Err(anyhow::anyhow!("Failed to get piece"))))
                    .expect("This future isn't polled after receiver is dropped; qed");
            }
        };
        let mut fut = Box::pin(fut.fuse());

        // Drive above future and stream back any pieces that were downloaded so far
        Ok(Box::new(stream::poll_fn(move |cx| {
            if !fut.is_terminated() {
                // Result doesn't matter, we'll need to poll stream below anyway
                let _ = fut.poll_unpin(cx);
            }

            if let Poll::Ready(maybe_result) = rx.poll_next_unpin(cx) {
                return Poll::Ready(maybe_result);
            }

            // Exit will be done by the stream above
            Poll::Pending
        })))
    }
}

/// Weak farmer piece getter, can be upgraded to [`FarmerPieceGetter`]
pub struct WeakFarmerPieceGetter<FarmIndex, PV, NC> {
    inner: Weak<Inner<FarmIndex, PV, NC>>,
}

impl<FarmIndex, PV, NC> fmt::Debug for WeakFarmerPieceGetter<FarmIndex, PV, NC> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WeakFarmerPieceGetter")
            .finish_non_exhaustive()
    }
}

impl<FarmIndex, PV, NC> Clone for WeakFarmerPieceGetter<FarmIndex, PV, NC> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

/// This wrapper allows us to return the stream, which in turn depends on `piece_getter` that was
/// previously on the stack of the inner function. What this wrapper does is create a
/// self-referential data structure, so we can move both together, while still implementing `Stream`
/// trait as necessary.
#[ouroboros::self_referencing]
struct StreamWithPieceGetter<FarmIndex, PV, NC>
where
    FarmIndex: 'static,
    PV: 'static,
    NC: 'static,
{
    piece_getter: RemotePieceGetter<FarmIndex, PV, NC>,
    #[borrows(piece_getter)]
    #[covariant]
    stream:
        Box<dyn Stream<Item = (PieceIndex, anyhow::Result<Option<Piece>>)> + Send + Unpin + 'this>,
}

impl<FarmIndex, PV, NC> Stream for StreamWithPieceGetter<FarmIndex, PV, NC>
where
    FarmIndex: 'static,
    PV: 'static,
    NC: 'static,
{
    type Item = (PieceIndex, anyhow::Result<Option<Piece>>);

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut()
            .with_stream_mut(|stream| stream.poll_next_unpin(cx))
    }
}

#[async_trait]
impl<FarmIndex, PV, NC> PieceGetter for WeakFarmerPieceGetter<FarmIndex, PV, NC>
where
    FarmIndex: Hash + Eq + Copy + fmt::Debug + Send + Sync + 'static,
    usize: From<FarmIndex>,
    PV: PieceValidator + Send + 'static,
    NC: NodeClient,
{
    async fn get_piece(&self, piece_index: PieceIndex) -> anyhow::Result<Option<Piece>> {
        let Some(piece_getter) = self.upgrade() else {
            debug!("Farmer piece getter upgrade didn't succeed");
            return Ok(None);
        };

        piece_getter.get_piece(piece_index).await
    }

    async fn get_pieces<'a, PieceIndices>(
        &'a self,
        piece_indices: PieceIndices,
    ) -> anyhow::Result<
        Box<dyn Stream<Item = (PieceIndex, anyhow::Result<Option<Piece>>)> + Send + Unpin + 'a>,
    >
    where
        PieceIndices: IntoIterator<Item = PieceIndex, IntoIter: Send> + Send + 'a,
    {
        let Some(piece_getter) = self.upgrade() else {
            debug!("Farmer piece getter upgrade didn't succeed");
            return Ok(Box::new(stream::iter(
                piece_indices
                    .into_iter()
                    .map(|piece_index| (piece_index, Ok(None))),
            )));
        };

        // TODO: This is necessary due to more complex lifetimes not yet supported by ouroboros, see
        //  https://github.com/someguynamedjosh/ouroboros/issues/112
        let piece_indices = piece_indices.into_iter().collect::<Vec<_>>();
        let stream_with_piece_getter =
            StreamWithPieceGetter::try_new_async_send(piece_getter, move |piece_getter| {
                piece_getter.get_pieces(piece_indices)
            })
            .await?;

        Ok(Box::new(stream_with_piece_getter))
    }
}

impl<FarmIndex, PV, NC> WeakFarmerPieceGetter<FarmIndex, PV, NC> {
    /// Try to upgrade to [`FarmerPieceGetter`] if there is at least one other instance of it alive
    pub fn upgrade(&self) -> Option<RemotePieceGetter<FarmIndex, PV, NC>> {
        Some(RemotePieceGetter {
            inner: self.inner.upgrade()?,
        })
    }
}
