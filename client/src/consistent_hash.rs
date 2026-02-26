use std::collections::BTreeMap;
use std::hash::{DefaultHasher, Hash, Hasher};

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
                let node_hash = Self::calculate_hash(&key);
                ring.insert(node_hash, node);
            }
        }

        ConsistentHashRing {
            ring,
        }
    }

    pub fn get_node_address(&self, value: &str) -> &str {
        let hash = Self::calculate_hash(value);

        let selected_node = self.ring.range(hash..)
            .next()
            .or_else(|| self.ring.iter().next())
            .map(|(_, address)| address);

        selected_node.unwrap().as_str()
    }

    fn calculate_hash(value: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }
}