use std::marker::PhantomData;
use std::sync::Arc;

use super::block_cache::{BlockCache, BlockId};
use super::leaf::Leaf;
use super::msg::Msg;
use super::tree_block::TreeBlock;
use super::update::Update;

pub struct BufferSegment<K, M> {
    updates: Update<K, M>,
}

struct BufferSegmentRef<K, M> {
    pivots: [K; 2],
    block: BlockId,
    _msg: PhantomData<M>,
}

struct BufferLevel<K, M> {
    merge_depth: u8,
    segments: Vec<BufferSegmentRef<K, M>>,
}

struct ChildRef<K, M> {
    pivot: K,
    block: BlockId,
    _msg: PhantomData<M>,
}

pub struct Node<K, M> {
    buffer_levels: Vec<BufferLevel<K, M>>,
    child_first: BlockId,
    child_rest: Vec<ChildRef<K, M>>,
}

impl<K, M> Node<K, M>
where
    K: Ord + Eq,
    M: Msg,
{
    pub async fn batch_update(
        &self,
        batch: Vec<M>,
        cache: BlockCache<TreeBlock<K, M>>,
    ) -> Result<Arc<Node<K, M>>, ()> {
        Err(())
    }
}

pub async fn compact<B, K, M>(
    root: BlockId,
    cache: BlockCache<TreeBlock<K, M>>,
    owner: Option<BlockId>,
    batch: B,
) -> Result<BlockId, ()>
where
    M: Msg,
    B: IntoIterator<Item = Update<K, M>>,
{
    Err(())
}
