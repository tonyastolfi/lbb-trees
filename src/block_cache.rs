use std::collections::HashMap;
use std::sync::Arc;

pub type BlockId = String;

pub struct BlockCache<B> {
    blocks: HashMap<BlockId, Arc<B>>,
}

impl<B> BlockCache<B> {
    // TODO - this method should take some kind of config info param; which device, file/dir, URL, etc.
    //
    async fn recover() -> Result<(Self, BlockId), ()> {
        Err(())
    }

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
