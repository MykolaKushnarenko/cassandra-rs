use crate::consistent_hash_ring::ConsistentHashRing;
use crate::replication::ReplicationStrategy;
pub(crate) use crate::routing::{Router, RoutingStrategy};

#[derive(Debug, Clone)]
pub struct Node {
    pub address: String,
}

impl Node {
    pub fn new(address: String) -> Node {
        Node { address }
    }
}

impl Into<Node> for String {
    fn into(self) -> Node {
        Node::new(self)
    }
}

impl Into<Node> for &str {
    fn into(self) -> Node {
        Node::new(self.to_string())
    }
}

pub struct Cluster {
    ring: ConsistentHashRing,
}

impl Cluster {
    pub fn new(nodes: Vec<Node>) -> Cluster {
        Cluster {
            ring: ConsistentHashRing::new(nodes),
        }
    }

    pub fn router(&self) -> Router<'_> {
        Router::new(&self.ring)
    }

    pub fn replication_strategy(&self) -> ReplicationStrategy<'_> {
        ReplicationStrategy::new(&self.ring)
    }

    pub fn drop_node(&mut self, node: &Node) {
        self.ring.drop_node(node);
    }

    pub fn add_node(&mut self, node: Node) {
        self.ring.add_node(node);
    }

    pub fn get_ring_entities(&self) -> Vec<(u64, &str)> {
        self.ring.get_entities()
    }
}
