use super::leaf::Leaf;
use super::msg::Msg;
use super::node::{BufferSegment, Node};

pub enum TreeBlock<K, M>
where
    M: Msg,
{
    BufferSegment(BufferSegment<K, M>),
    Node(Node<K, M>),
    Leaf(Leaf<K, M::Val>),
}
