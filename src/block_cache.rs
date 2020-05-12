use std::collections::HashMap;
use std::sync::Arc;

pub type BlockId = String;

pub struct BlockCache<B> {
    blocks: HashMap<BlockId, Arc<B>>,
}

impl<B> BlockCache<B> {
    async fn alloc(&self) -> Result<BlockId, ()> {
        Err(())
    }

    async fn load(&self, id: BlockId) -> Result<Arc<B>, ()> {
        Err(())
    }

    async fn store(&self, id: BlockId, block: Arc<B>, owner: Option<BlockId>) -> Result<(), ()> {
        Err(())
    }
}
