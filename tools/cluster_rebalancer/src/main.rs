mod rebalancer;

use crate::rebalancer::{RebalanceAction, Rebalancer};
use shared::cluster::Cluster;
use shared::consistent_hash_ring::Node;
use shared::protocol::types::Request;
use text_colorizer::Colorize;

const NODES_ARG_KEY: &str = "--nodes=";

const LOG_INFO: &str = "INFO";
const LOG_VERBOSE: &str = "VERBOSE";

fn main() {
    let nodes = initialize_cluster_config();

    let cluster = Cluster::new(nodes.clone());
    let mut rebalancer = Rebalancer::new(cluster);
    println!(
        "{}: {}",
        LOG_INFO.bright_green(),
        "Adding data to the cluster".green().bold()
    );

    for i in 0..3000 {
        let value = i.to_string();
        let request = Request::Add(value);

        let strategy = rebalancer.cluster.route_request(&request);
        _ = rebalancer.connection_pool.execute(strategy, request, None);
    }

    let count_request = Request::Count;
    let strategy = rebalancer.cluster.route_request(&count_request);
    let _ = rebalancer
        .connection_pool
        .execute(strategy, count_request, None);

    println!(
        "{}: {}",
        LOG_INFO.bright_green(),
        "Data added".green().bold()
    );

    loop {
        println!(
            "{}: {}",
            LOG_INFO.bright_green(),
            "Enter address of a new node to start rebalancing"
                .green()
                .bold()
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        println!(
            "{}: {}",
            LOG_INFO.bright_green(),
            "Enter address of a new node to start rebalancing"
                .green()
                .bold()
        );

        let mut node = String::new();
        std::io::stdin().read_line(&mut node).unwrap();

        let new_node = Node {
            address: node.trim().to_string(),
        };

        match input.trim() {
            "add" => {
                let action = RebalanceAction::AddNode;
                rebalancer.rebalance(action, new_node.address.clone());
            }
            "drop" => {
                let action = RebalanceAction::DropNode;
                rebalancer.rebalance(action, new_node.address.clone());
            }
            _ => {}
        }
    }

    fn initialize_cluster_config() -> Vec<Node> {
        let nodes = std::env::args()
            .find(|arg| arg.starts_with(NODES_ARG_KEY))
            .expect("--nodes argument is required")
            .trim_start_matches(NODES_ARG_KEY)
            .split(",")
            .map(|s| Node {
                address: s.to_string(),
            })
            .collect::<Vec<Node>>();

        println!("Nodes: {:?}", nodes);

        nodes
    }
}
