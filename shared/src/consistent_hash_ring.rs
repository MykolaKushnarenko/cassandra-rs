use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const REPLICAS: usize = 10;

#[derive(Debug, Clone)]
pub struct Node {
    pub address: String,
}

pub struct ConsistentHashRing {
    ring: BTreeMap<u64, usize>,
    nodes: Vec<Node>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

impl ConsistentHashRing {
    pub fn new(nodes: Vec<Node>) -> ConsistentHashRing {
        let mut ring = BTreeMap::new();

        for (index, node) in nodes.iter().enumerate() {
            for i in 0..REPLICAS {
                let key = format!("{}:{}", node.address, i);
                let node_hash = Self::calculate_hash(&key);
                ring.insert(node_hash, index);
            }
        }

        ConsistentHashRing { ring, nodes }
    }

    pub fn get_ranges_for_node(&self, look_up_node: &str) -> Vec<Range> {
        let mut ranges = Vec::new();

        let node_hash_values = self
            .ring
            .iter()
            .map(|(hash, node)| (*hash, *node))
            .collect::<Vec<(u64, usize)>>();

        for i in 0..node_hash_values.len() {
            let (current_hash, current_node_index) = node_hash_values[i];

            if self.nodes[current_node_index].address != look_up_node {
                continue;
            }

            let previous_hash = if i == 0 { 0 } else { node_hash_values[i - 1].0 };

            ranges.push(Range {
                start: previous_hash,
                end: current_hash,
            })
        }

        ranges
    }

    pub fn get_node_address(&self, value: &str) -> &str {
        let hash = Self::calculate_hash(value);

        let selected_node = self
            .ring
            .range(hash..)
            .next()
            .or_else(|| self.ring.iter().next())
            .map(|(_, address)| address);

        let index = selected_node.unwrap();

        self.nodes[*index].address.as_str()
    }

    pub fn calculate_hash(value: &str) -> u64 {
        let hash = murmur3::murmur3_x64_128(&mut std::io::Cursor::new(value), 0).unwrap();
        hash as u64
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
        let node_index = self.nodes.len() - 1;
        let inserted_node = self.nodes.last().unwrap();
        for i in 0..REPLICAS {
            let key = format!("{}:{}", inserted_node.address, i);
            let node_hash = Self::calculate_hash(&key);
            self.ring.insert(node_hash, node_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::consistent_hash_ring::{ConsistentHashRing, Node};

    #[test]
    fn test_get_node_address() {
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

        let ring = ConsistentHashRing::new(nodes_ips);

        assert_eq!(ring.get_node_address("test_key"), "127.0.0.1:3002");
    }

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

        assert_eq!(ring.get_node_address("test_key1"), "127.0.0.1:3002");

        ring.add_node(new_node);

        let hash1 = ConsistentHashRing::calculate_hash("test_key1");
        let mut count = 0;

        let new_node_ranges = ring.get_ranges_for_node(new_node_ip);
        for range in new_node_ranges.iter() {
            if hash1 > range.start && hash1 < range.end {
                count += 1;
            }
        }

        assert_eq!(count, 1);
        assert_eq!(ring.get_node_address("test_key1"), "127.0.0.1:3003");

        let res = ring.get_ranges_for_node("127.0.0.1:3003");

        for range in res.iter() {
            if range.start > range.end {
                panic!("Wrong range!");
            }
        }
    }
}
