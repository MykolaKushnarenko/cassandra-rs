use shared::consistent_hash::ConsistentHash;
use std::collections::BTreeMap;

const REPLICAS: usize = 10;

#[derive(Debug)]
pub struct Cluster {
    pub nodes: Vec<String>
}

pub struct ConsistentHashRing<'a> {
    ring: BTreeMap<u64, &'a String>,
}

impl<'a> ConsistentHashRing<'a> {
    pub(crate) fn new(cluster: &'a Cluster) -> ConsistentHashRing<'a> {
        let mut ring = BTreeMap::new();

        for node in cluster.nodes.iter() {
            for i in 0..REPLICAS {
                let key = format!("{}:{}", node, i);
                let node_hash = ConsistentHash::calculate_hash(&key);
                ring.insert(node_hash, node);
            }
        }

        ConsistentHashRing {
            ring,
        }
    }

    pub fn get_node_address(&self, value: &str) -> &str {
        let hash = ConsistentHash::calculate_hash(value);

        let selected_node = self.ring.range(hash..)
            .next()
            .or_else(|| self.ring.iter().next())
            .map(|(_, address)| address);

        selected_node.unwrap().as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::consistent_hash::{Cluster, ConsistentHashRing};

    #[test]
    fn test_get_node_address() {
        let nodes_ips = vec![
            "127.0.0.1:3000".to_string(),
            "127.0.0.1:3001".to_string(),
            "127.0.0.1:3002".to_string()];

        let cluster = Cluster { nodes: nodes_ips };

        let ring = ConsistentHashRing::new(&cluster);

        assert_eq!(ring.get_node_address("test_key"), "127.0.0.1:3002");
    }
}