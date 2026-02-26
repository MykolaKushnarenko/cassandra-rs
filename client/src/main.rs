mod consistent_hash;
use crate::consistent_hash::{Cluster, ConsistentHashRing};
use std::io::Write;
use std::net::TcpStream;
use text_colorizer::*;
use shared::connection::Connection;
use shared::protocol::types::Request;

const NODES_ARG_KEY: &str = "--nodes=";

const LOG_INFO: &str = "INFO";
const LOG_ERROR: &str = "ERROR";
const LOG_VERBOSE: &str = "VERBOSE";

fn main() {
    let cluster = initialize_cluster_config();
    let hasher = ConsistentHashRing::new(&cluster);

    println!("Nodes: {:?}", cluster);

    start_processing(hasher);
    println!("Connected");
}

fn initialize_cluster_config() -> Cluster {
    let nodes = std::env::args()
        .find(|arg| arg.starts_with(NODES_ARG_KEY))
        .expect("--nodes argument is required")
        .trim_start_matches(NODES_ARG_KEY)
        .split(",")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    Cluster { nodes }
}

fn start_processing(mut hasher: ConsistentHashRing) {
    loop {
        println!("{}: {}", LOG_INFO.bright_green(), "Supported command: Add, Check (Enter command)".green().bold());
        print!("{}: {}", LOG_INFO.bright_green(), "Enter command -> ".green().bold());
        std::io::stdout().flush().unwrap();

        let mut command_input = String::new();
        std::io::stdin().read_line(&mut command_input).unwrap();

        println!("{}: {}", LOG_INFO.bright_green(), "Enter values (e.g. '123')".blue().bold());
        print!("{}: {}", LOG_INFO.bright_green(), "Enter value -> ".green().bold());
        std::io::stdout().flush().unwrap();

        let mut value = String::new();
        std::io::stdin().read_line(&mut value).unwrap();
        send(command_input.trim(), value.trim(), &mut hasher);
    }
}

fn send(operation: &str, value: &str, hasher: &mut ConsistentHashRing) {
    let node_address = hasher.get_node_address(value);

    let request;
    match operation {
        "add" => {
            request = Request::Add(value.to_string());
        }
        "check" => {
            request = Request::Check(value.to_string());
        }
        _ => {
            eprintln!("{}: {}", LOG_ERROR.bright_red(), format!("Operation {} is not supported", operation).red().bold());
            return;
        }
    }

    println!("{}: Sending {} to {}", LOG_VERBOSE, operation, node_address);
    let stream_result = TcpStream::connect(node_address);

    let stream;
    match stream_result {
        Ok(value) => stream = value,
        Err(_) => {
            eprintln!("{}: {}", LOG_ERROR.bright_red(), format!("Cannot connect to server {}", node_address).red().bold());
            return;
        }
    }

    let mut connection = Connection::new(&stream);

    let response_result = connection.send_request_with_response(request);

    let response;
    match response_result {
        Ok(value) => { response = value},
        Err(e) => {
            eprintln!("{}: {}", LOG_ERROR.bright_red(), format!("Error occurred while processing message: {:?}", e).red().bold());
            return;
        }
    }

    println!("{}: {}", LOG_VERBOSE, format!("Response: {:?}", response));
    std::io::stdout().flush().unwrap();
}
