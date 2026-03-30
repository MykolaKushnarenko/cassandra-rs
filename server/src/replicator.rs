use log::info;
use shared::cluster::{Cluster, Node};
use shared::connection_pool::ConnectionPool;
use shared::protocol::types::{Entry, Request};
use shared::routing::RoutingStrategy;
use std::sync::mpsc::Receiver;

pub(crate) struct Replicator {
    cluster: Cluster,
    receiver: Receiver<ReplicationEntry>,
    connection_pool: ConnectionPool,
    current_host: String,
}

pub(crate) enum ReplicationEntry {
    Single(Entry),
    Batch(Vec<Entry>),
}

impl Replicator {
    pub(crate) fn new(
        current_host: String,
        cluster: Cluster,
        receiver: Receiver<ReplicationEntry>,
        connection_pool: ConnectionPool,
    ) -> Self {
        Replicator {
            current_host,
            cluster,
            receiver,
            connection_pool,
        }
    }

    pub(crate) fn run(&mut self) {
        while let Ok(replication_entry) = self.receiver.recv() {
            match replication_entry {
                ReplicationEntry::Single(entry) => self.replicate_single_value(entry),
                ReplicationEntry::Batch(entries) => self.replicate_batch(&entries),
            }
        }
    }

    fn replicate_single_value(&mut self, entry: Entry) {
        if entry.replication_factor.is_some() {
            self.replicate_value(entry.clone());
        }
        info!("Replication factor is 0");
    }

    fn replicate_batch(&mut self, entry: &[Entry]) {
        for entry in entry.iter() {
            if entry.replication_factor.is_some() {
                self.replicate_value(entry.clone());
            }
            info!("Replication factor is 0");
        }
    }

    fn replicate_value(&mut self, entry: Entry) {
        let current_host = self.current_host.clone().into();
        let hash =
            shared::consistent_hash_ring::ConsistentHashRing::calculate_hash(entry.value.as_str());

        let replication_strategy = self.cluster.replication_strategy();
        let nodes = replication_strategy.get_replica_nodes(
            hash,
            &current_host,
            entry.replication_factor.unwrap(),
        );

        for node_address in nodes.iter() {
            let node = Node::new(node_address.to_string());
            let strategy = RoutingStrategy::Direct(&node);

            //since the value is being replicated, we don't want to replicate it again
            let request = Request::Add(Entry {
                value: entry.value.clone(),
                replication_factor: None,
            });

            info!("Sending request to node: {}", node_address);
            let result = self.connection_pool.execute(strategy, request, None);

            info!("{:?}", result);
        }
    }
}
