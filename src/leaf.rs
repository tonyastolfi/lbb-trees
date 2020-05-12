use itertools::Itertools;

use super::block_cache::BlockCache;
use super::item::Item;
use super::msg::Msg;
use super::update::Update;

pub struct Leaf<K, V> {
    pub items: Vec<Item<K, V>>,
}

impl<K, V> Leaf<K, V> {
    pub fn batch_update<B, M: Msg<Val = V>>(&self, batch: B) -> Leaf<K, V>
    where
        K: Eq + Ord + Clone,
        V: Clone,
        B: IntoIterator<Item = Update<K, M>>,
    {
        use itertools::EitherOrBoth::*;

        Leaf {
            items: self
                .items
                .iter()
                .merge_join_by(batch.into_iter(), |item, update| item.key.cmp(&update.key))
                .filter_map(|either| match either {
                    Left(item) => Some(item.clone()),
                    Right(update) => update.deliver(None),
                    Both(item, update) => update.deliver(Some(item)),
                })
                .collect(),
        }
    }
}
