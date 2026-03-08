pub use crate::cluster::Node;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Number of virtual nodes created per physical node.
///
/// Virtual nodes improve data distribution by creating multiple points
/// on the hash ring for each physical node, reducing hot spots.
const VIRTUAL_PARTITION_COUNT: usize = 10;

/// A consistent hashing ring for distributing keys across nodes.
///
/// This implementation uses virtual nodes to ensure even distribution
/// of data across the cluster. Each physical node is assigned multiple positions
/// on the ring, determined by hashing the node address with a replica index.
///
/// # How it works
///
/// 1. Each node gets 10 virtual tokens placed at hash positions on the ring
/// 2. Keys are hashed and mapped to the next token clockwise on the ring
/// 3. The node owning that token is responsible for storing the key
pub struct ConsistentHashRing {
    /// Maps hash values (tokens) to node indices
    ring: BTreeMap<u64, usize>,
    /// All nodes in the cluster
    nodes: Vec<Node>,
}

/// Represents a range of hash values on the consistent hash ring.
///
/// A range is defined by a start hash (exclusive) and end hash (inclusive).
/// When `start > end`, this represents a wraparound range that spans from
/// start to u64::MAX and then from 0 to end.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Range {
    /// Start of the range (exclusive)
    pub start: u64,
    /// End of the range (inclusive)
    pub end: u64,
}

impl ConsistentHashRing {
    /// Creates a new consistent hash ring with the given nodes.
    ///
    /// Each node is assigned multiple virtual partitions to ensure even distribution.
    pub fn new(nodes: Vec<Node>) -> ConsistentHashRing {
        let mut ring = BTreeMap::new();

        for (index, node) in nodes.iter().enumerate() {
            for i in 0..VIRTUAL_PARTITION_COUNT {
                let key = format!("{}:{}", node.address, i);
                let node_hash = Self::calculate_hash(&key);
                ring.insert(node_hash, index);
            }
        }

        ConsistentHashRing { ring, nodes }
    }

    /// Returns the node responsible for a given hash value.
    pub fn get_node(&self, hash: u64) -> &Node {
        let selected_node = self
            .ring
            .range(hash..)
            .next()
            .or_else(|| self.ring.iter().next())
            .map(|(_, address)| address);

        &self.nodes[*selected_node.unwrap()]
    }

    pub fn get_entities(&self) -> Vec<(u64, &str)> {
        self.ring
            .iter()
            .map(|(hash, index)| (*hash, self.nodes[*index].address.as_str()))
            .collect()
    }

    pub fn calculate_hash(value: &str) -> u64 {
        let hash = murmur3::murmur3_x64_128(&mut std::io::Cursor::new(value), 0).unwrap();
        hash as u64
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
        let node_index = self.nodes.len() - 1;
        let inserted_node = self.nodes.last().unwrap();
        for i in 0..VIRTUAL_PARTITION_COUNT {
            let key = format!("{}:{}", inserted_node.address, i);
            let node_hash = Self::calculate_hash(&key);
            self.ring.insert(node_hash, node_index);
        }
    }

    pub fn drop_node(&mut self, node: &Node) {
        for i in 0..VIRTUAL_PARTITION_COUNT {
            let key = format!("{}:{}", node.address, i);
            let node_hash = Self::calculate_hash(&key);
            self.ring.remove(&node_hash);
        }
        self.nodes.retain(|n| n.address != node.address);
    }

    pub fn get_nodes(&self) -> &[Node] {
        &self.nodes
    }
}

#[cfg(test)]
mod tests {
    use crate::consistent_hash_ring::{ConsistentHashRing, Node};

    #[test]
    fn test_hash() {
        let ip = "127.0.0.1:3000";
        let hash = ConsistentHashRing::calculate_hash(ip);
        assert_eq!(hash, 2784727742823359555);
    }

    #[test]
    fn test_add_node_to_the_ring() {
        let nodes_ips = vec![
            Node {
                address: "127.0.0.1:3000".to_string(),
            },
            Node {
                address: "127.0.0.1:3001".to_string(),
            },
            Node {
                address: "127.0.0.1:3002".to_string(),
            },
        ];

        let new_node_ip = "127.0.0.1:3003";
        let new_node = Node {
            address: "127.0.0.1:3003".to_string(),
        };

        let mut ring = ConsistentHashRing::new(nodes_ips);

        let value_hash = ConsistentHashRing::calculate_hash("test_key1");
        let node_hash = ring.get_node(value_hash);
        assert_eq!(node_hash.address, "127.0.0.1:3002");

        ring.add_node(new_node);
    }
}
