use crate::LOG_VERBOSE;
use shared::cluster::{Cluster, Node, RoutingStrategy};
use shared::connection_pool::ConnectionPool;
use shared::consistent_hash_ring::Range;
use shared::protocol::types::{Request, Response};

pub enum RebalanceAction {
    AddNode,
    DropNode,
}

pub struct Rebalancer {
    pub connection_pool: ConnectionPool,
    pub cluster: Cluster,
}

impl Rebalancer {
    pub fn new(cluster: Cluster) -> Self {
        Self {
            connection_pool: ConnectionPool::new(),
            cluster,
        }
    }

    pub fn rebalance(&mut self, action: RebalanceAction, node: String) {
        match action {
            RebalanceAction::AddNode => {
                self.add_new_node(node);
            }
            RebalanceAction::DropNode => self.remove_node(node),
        }
    }

    fn add_new_node(&mut self, node_address: String) {
        let node = Node {
            address: node_address.clone(),
        };
        self.cluster.add_node(node.clone());

        let mut entities = self.cluster.get_ring_entities();
        entities.sort_by_key(|&(hash, _)| hash);

        let ranges = Self::get_ranges(entities, node_address.as_str());
        let mut rebalanced_items = Vec::new();
        
        let request = Request::GetBatch(ranges.clone());
        let routing_strategy = self.cluster.route_request(&request);

        let result =
            self.connection_pool
                .execute(routing_strategy, request, Some(node.address.as_str()));

        match result {
            Ok(Response::Array(value)) => {
                rebalanced_items = value;
            }
            Err(_) => {}
            _ => {}
        }

        let strategy = RoutingStrategy::Direct(&node);
        let result =
            self.connection_pool
                .execute(strategy, Request::AddBatch(rebalanced_items), None);

        match result {
            Ok(_) => {
                println!("Added batch successfully to new node: {}", node_address);
            }
            Err(e) => {
                println!("Failed to add batch: {:?}", e);
            }
        }

        let request = Request::DropBatch(ranges.clone());
        let routing_strategy = self.cluster.route_request(&request);
        let response = self
            .connection_pool
            .execute(routing_strategy, request, Some(node.address.as_str()))
            .unwrap();
        match response {
            Response::Array(value) => {
                println!(
                    "{}: {}",
                    LOG_VERBOSE,
                    format!("[{}] Received data: {:?}", node.address, value)
                );
            }
            _ => {
                println!(
                    "{}: {}",
                    LOG_VERBOSE,
                    format!("Received data: {:?}", response)
                );
            }
        }

        println!(
            "{}: {}",
            LOG_VERBOSE,
            format!("Rebalanced with new node {}", node.address)
        );

        let count_request = Request::Count;
        let strategy = self.cluster.route_request(&count_request);
        let _ = self.connection_pool.execute(strategy, count_request, None);
    }

    fn remove_node(&mut self, node_address: String) {
        let node = Node {
            address: node_address,
        };
        self.cluster.drop_node(&node);

        let mut dropped_items = Vec::new();

        let request = Request::GetBatch(vec![Range {
            start: 0,
            end: u64::MAX,
        }]);
        let routing_strategy = RoutingStrategy::Direct(&node);
        let result = self
            .connection_pool
            .execute(routing_strategy, request, None);

        match result {
            Ok(Response::Array(value)) => {
                dropped_items = value;
            }
            Err(_) => {}
            _ => {}
        }

        for item in dropped_items {
            let request = Request::Add(item);
            let strategy = self.cluster.route_request(&request);
            let result = self.connection_pool.execute(strategy, request, None);

            match result {
                Ok(Response::String(val)) => {
                    println!("{}", val);
                }
                Ok(_) => {}
                Err(e) => {
                    println!("Failed to add: {:?}", e);
                }
            }
        }

        let drop_request = Request::GetBatch(vec![Range {
            start: 0,
            end: u64::MAX,
        }]);
        let routing_strategy = RoutingStrategy::Direct(&node);
        let result = self
            .connection_pool
            .execute(routing_strategy, drop_request, None);
        match result {
            Ok(_) => {
                println!("Dropped all items from node: {}", &node.address);
            }
            _ => {}
        }

        println!(
            "{}: {}",
            LOG_VERBOSE,
            format!("Rebalanced with new node {}", node.address)
        );

        let count_request = Request::Count;
        let strategy = self.cluster.route_request(&count_request);
        let _ = self.connection_pool.execute(strategy, count_request, None);
    }

    fn get_ranges(entities: Vec<(u64, &str)>, look_up_node: &str) -> Vec<Range> {
        let mut ranges = Vec::new();

        for i in 0..entities.len() {
            let (current_hash, current_node) = entities[i];

            //if it's not the newly added node then skip
            if current_node != look_up_node {
                continue;
            }

            // get previous node hash in the ring that will serve as a starting point
            // since if hash of the value will be greater then the hash of the node in the ring
            // it will then take the next node in the ring
            let previous_hash = if i == 0 { 0 } else { entities[i - 1].0 };

            ranges.push(Range {
                start: previous_hash,
                end: current_hash,
            })
        }

        ranges
    }
}
