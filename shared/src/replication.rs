use crate::cluster::Node;
use crate::consistent_hash_ring::ConsistentHashRing;
use std::collections::HashSet;

pub struct ReplicationStrategy<'a> {
    ring: &'a ConsistentHashRing,
}

impl<'a> ReplicationStrategy<'a> {
    pub fn new(ring: &'a ConsistentHashRing) -> Self {
        Self { ring }
    }

    /// Returns the node addresses where data should be replicated.
    ///
    /// Finds the next N nodes on the ring (clockwise) after the given hash position,
    /// excluding the original node.
    ///
    /// # Arguments
    /// * `value_hash` - The hash of the value to replicate
    /// * `exclude_node` - The node to exclude from replication (usually the primary node)
    /// * `replication_factor` - Number of replica nodes to select
    pub fn get_replica_nodes(
        &self,
        value_hash: u64,
        exclude_node: &Node,
        replication_factor: usize,
    ) -> Vec<&str> {
        let mut nodes = self.ring.get_entities();
        nodes.sort_by(|(hash1, _), (hash2, _)| hash1.cmp(hash2));

        // Filter out the excluded node
        let filtered_nodes: Vec<(u64, &str)> = nodes
            .iter()
            .filter(|(_, node_address)| *node_address != exclude_node.address)
            .map(|(hash, node_address)| (*hash, *node_address))
            .collect();

        // Find position on the ring after the value hash
        let position = filtered_nodes
            .iter()
            .position(|(hash, _)| *hash > value_hash)
            .unwrap_or(0);

        // Select next N nodes (wrapping around if necessary)
        let target_nodes = filtered_nodes
            .iter()
            .cycle()
            .skip(position)
            .take(replication_factor)
            .map(|(_, node_address)| *node_address)
            .collect::<HashSet<&str>>();

        target_nodes.into_iter().collect()
    }
}
