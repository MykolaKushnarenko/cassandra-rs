use crate::consistent_hash_ring::ConsistentHashRing;
use crate::protocol::types::Request;

#[derive(Debug)]
pub enum RoutingStrategy<'a> {
    Direct(&'a Node),
    Fanout(&'a [Node]),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub address: String,
}

impl Node {
    pub fn new(address: String) -> Node {
        Node { address }
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

    pub fn route_request(&self, request: &Request) -> RoutingStrategy<'_> {
        let strategy = match request {
            Request::Add(val) | Request::Check(val) => {
                let hash = ConsistentHashRing::calculate_hash(val);
                let node = self.ring.get_node(hash);
                RoutingStrategy::Direct(node)
            }
            _ => RoutingStrategy::Fanout(self.ring.get_nodes()),
        };

        strategy
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

    pub fn get_nodes(&self) -> &[Node] {
        self.ring.get_nodes()
    }
}
