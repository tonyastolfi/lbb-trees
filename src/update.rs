use super::item::Item;
use super::msg::Msg;

pub struct Update<K, M> {
    pub key: K,
    pub msg: M,
}

impl<K, M> Update<K, M>
where
    K: Eq,
    M: Msg,
{
    pub fn try_merge(self, other: Self) -> Result<Self, (Self, Self)> {
        if self.key == other.key {
            Ok(Update {
                key: self.key,
                msg: self.msg.merge(other.msg),
            })
        } else {
            Err((self, other))
        }
    }

    pub fn deliver(self, item: Option<&Item<K, M::Val>>) -> Option<Item<K, M::Val>>
    where
        K: Eq,
    {
        assert!(item.map_or(true, |item| self.key == item.key));

        let key = self.key;
        self.msg
            .deliver(item.map(|item| &item.val))
            .map(|val| Item { key, val })
    }
}
