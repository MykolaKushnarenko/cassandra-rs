pub use crate::cluster::Node;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Number of virtual nodes created per physical node.
///
/// Virtual nodes improve data distribution by creating multiple points
/// on the hash ring for each physical node, reducing hot spots.
const VIRTUAL_PARTITION_COUNT: usize = 256;

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
#[derive(Debug)]
pub struct ConsistentHashRing {
    /// Maps hash values (tokens) to node indices
    ring: BTreeMap<u64, String>,
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

        for node in nodes.iter() {
            Self::insert_virtual_nodes(&mut ring, &node.address);
        }

        ConsistentHashRing { ring, nodes }
    }

    /// Inserts virtual nodes for a given node address into the ring.
    fn insert_virtual_nodes(ring: &mut BTreeMap<u64, String>, node_address: &str) {
        for i in 0..VIRTUAL_PARTITION_COUNT {
            let key = format!("{}:vnode:{}", node_address, i);
            let hash = Self::calculate_hash_with_seed(&key, i as u32);
            ring.insert(hash, node_address.to_string());
        }
    }

    /// Removes virtual nodes for a given node address from the ring.
    fn remove_virtual_nodes(ring: &mut BTreeMap<u64, String>, node_address: &str) {
        for i in 0..VIRTUAL_PARTITION_COUNT {
            let key = format!("{}:vnode:{}", node_address, i);
            let hash = Self::calculate_hash_with_seed(&key, i as u32);
            ring.remove(&hash);
        }
    }

    /// Returns the node responsible for a given hash value.
    pub fn get_node(&self, hash: u64) -> &Node {
        let selected_node = self
            .ring
            .range(hash..)
            .next()
            .or_else(|| self.ring.iter().next())
            .map(|(_, address)| address)
            .unwrap();

        self.nodes
            .iter()
            .find(|n| &n.address == selected_node)
            .unwrap()
    }

    pub fn get_entities(&self) -> Vec<(u64, &str)> {
        self.ring
            .iter()
            .map(|(hash, node_address)| (*hash, node_address.as_str()))
            .collect()
    }

    pub fn calculate_hash(value: &str) -> u64 {
        let hash = murmur3::murmur3_x64_128(&mut std::io::Cursor::new(value), 0).unwrap();
        hash as u64
    }

    pub fn calculate_hash_with_seed(value: &str, seed: u32) -> u64 {
        let hash = murmur3::murmur3_x64_128(&mut std::io::Cursor::new(value), seed).unwrap();
        hash as u64
    }

    pub fn add_node(&mut self, node: Node) {
        let node_address = node.address.clone();
        Self::insert_virtual_nodes(&mut self.ring, &node_address);
        self.nodes.push(node);
    }

    pub fn drop_node(&mut self, node: &Node) {
        Self::remove_virtual_nodes(&mut self.ring, &node.address);
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

        let new_node = Node {
            address: "127.0.0.1:3003".to_string(),
        };

        let mut ring = ConsistentHashRing::new(nodes_ips);

        let value_hash = ConsistentHashRing::calculate_hash("test_key1");
        let node_hash = ring.get_node(value_hash);
        assert_eq!(node_hash.address, "127.0.0.1:3002");

        ring.add_node(new_node);
    }

    #[test]
    fn test_get_ring_() {
        let nodes_ips = vec![
            Node {
                address: "localhost:3000".to_string(),
            },
            Node {
                address: "localhost:4000".to_string(),
            },
            Node {
                address: "localhost:5001".to_string(),
            },
            Node {
                address: "localhost:6001".to_string(),
            },
        ];

        let new_node = Node {
            address: "localhost:7001".to_string(),
        };

        let mut ring = ConsistentHashRing::new(nodes_ips);

        let value_hash = ConsistentHashRing::calculate_hash("test_key1");
        let node_hash = ring.get_node(value_hash);
        assert_eq!(node_hash.address, "localhost:4000");

        ring.add_node(new_node);
    }
}
