use super::msg::Msg;

pub struct PutVal<V> {
    pub val: V,
}

pub struct Delete;

pub enum DefaultMsg<V> {
    Put(PutVal<V>),
    Delete(Delete),
}

impl<V> Msg for DefaultMsg<V>
where
    V: Clone,
{
    type Val = V;

    fn merge(self, other: Self) -> Self {
        other
    }

    fn deliver(self, _val: Option<&V>) -> Option<V> {
        use DefaultMsg::*;

        match self {
            Put(put) => Some(put.val.clone()),
            Delete(_) => None,
        }
    }
}
