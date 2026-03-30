mod client;

use crate::client::{Client, Settings};
use shared::consistent_hash_ring::Node;
use shared::protocol::types::{Entry, Request};
use std::io::Write;
use text_colorizer::*;

const NODES_ARG_KEY: &str = "--nodes=";

const LOG_INFO: &str = "INFO";
const LOG_ERROR: &str = "ERROR";
const LOG_VERBOSE: &str = "VERBOSE";

fn main() {
    let nodes = initialize_cluster_config();
    let settings = Settings::new(nodes);
    let mut client = Client::new(settings);

    start_processing(&mut client);
    println!("Connected");
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

fn start_processing(client: &mut Client) {
    loop {
        println!(
            "{}: {}",
            LOG_INFO.bright_green(),
            "Supported command: Add, Check (Enter command)"
                .green()
                .bold()
        );
        print!(
            "{}: {}",
            LOG_INFO.bright_green(),
            "Enter command -> ".green().bold()
        );
        std::io::stdout().flush().unwrap();

        let mut command_input = String::new();
        std::io::stdin().read_line(&mut command_input).unwrap();

        println!(
            "{}: {}",
            LOG_INFO.bright_green(),
            "Enter values (e.g. '123')".blue().bold()
        );
        print!(
            "{}: {}",
            LOG_INFO.bright_green(),
            "Enter value -> ".green().bold()
        );
        std::io::stdout().flush().unwrap();

        let mut value = String::new();
        std::io::stdin().read_line(&mut value).unwrap();
        send(client, command_input.trim(), value.trim());
    }
}

fn send(client: &mut Client, operation: &str, value: &str) {
    let request;
    match operation {
        "add" => {
            println!("{}: {}", LOG_INFO, "Replication factor: (Empty to skip)");
            let mut replication_factor_input = String::new();
            std::io::stdin()
                .read_line(&mut replication_factor_input)
                .unwrap();
            request = match replication_factor_input.trim() {
                "" => Request::Add(Entry {
                    value: value.to_string(),
                    replication_factor: None,
                }),
                _ => {
                    let replication_factor = replication_factor_input
                        .trim()
                        .parse::<u32>()
                        .ok()
                        .or(Some(0))
                        .unwrap();
                    Request::Add(Entry {
                        value: value.to_string(),
                        replication_factor: Some(replication_factor as usize),
                    })
                }
            };
        }
        "check" => {
            request = Request::Check(value.to_string());
        }
        _ => {
            eprintln!(
                "{}: {}",
                LOG_ERROR.bright_red(),
                format!("Operation {} is not supported", operation)
                    .red()
                    .bold()
            );
            return;
        }
    }

    let response_result = client.send(request);

    let response;
    match response_result {
        Ok(value) => response = value,
        Err(e) => {
            eprintln!(
                "{}: {}",
                LOG_ERROR.bright_red(),
                format!("Error occurred while processing message: {:?}", e)
                    .red()
                    .bold()
            );
            return;
        }
    }

    println!("{}: {}", LOG_VERBOSE, format!("Response: {:?}", response));
    std::io::stdout().flush().unwrap();
}
