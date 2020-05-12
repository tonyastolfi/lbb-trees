use std::marker::Sized;

pub trait Msg {
    type Val;

    fn merge(self, other: Self) -> Self
    where
        Self: Sized;

    fn deliver(self, current: Option<&Self::Val>) -> Option<Self::Val>;
}
