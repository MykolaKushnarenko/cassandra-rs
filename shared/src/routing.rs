use crate::cluster::Node;
use crate::consistent_hash_ring::ConsistentHashRing;
use crate::protocol::types::Request;

#[derive(Debug)]
pub enum RoutingStrategy<'a> {
    Direct(&'a Node),
    Fanout(&'a [Node]),
}

pub struct Router<'a> {
    ring: &'a ConsistentHashRing,
}

impl<'a> Router<'a> {
    pub fn new(ring: &'a ConsistentHashRing) -> Self {
        Self { ring }
    }

    pub fn route_request(&self, request: &Request) -> RoutingStrategy<'a> {
        match request {
            Request::Add(val) => {
                let hash = ConsistentHashRing::calculate_hash(val.value.as_str());
                let node = self.ring.get_node(hash);
                RoutingStrategy::Direct(node)
            }
            Request::Check(val) => {
                let hash = ConsistentHashRing::calculate_hash(val.as_str());
                let node = self.ring.get_node(hash);
                RoutingStrategy::Direct(node)
            }
            _ => RoutingStrategy::Fanout(self.ring.get_nodes()),
        }
    }
}
