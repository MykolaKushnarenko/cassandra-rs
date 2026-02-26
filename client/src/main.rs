mod consistent_hash;

use crate::consistent_hash::{Cluster, ConsistentHashRing};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};
use std::net::TcpStream;
use text_colorizer::*;

const NODES_ARG_KEY: &str = "--nodes=";

const LOG_INFO: &str = "INFO";
const LOG_ERROR: &str = "ERROR";
const LOG_VERBOSE: &str = "VERBOSE";


#[derive(Debug, Serialize, )]
struct OutboundCommand{
    command: String,
    args: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct InboundMessage{
    status: String,
    result: String,
}

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
        .expect("--node argument is required")
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

    let command = OutboundCommand {
        command: String::from(operation),
        args: vec![String::from(value.trim())]
    };

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

    let mut writer = std::io::BufWriter::new(&stream);
    let mut reader = std::io::BufReader::new(&stream);

    let mut butes = serde_json::to_vec(&command).unwrap();
    butes.push(0u8);

    writer.write_all(&butes).unwrap();
    writer.flush().unwrap();

    let mut response_bytes = Vec::<u8>::new();
    reader.read_until(0u8, &mut response_bytes).unwrap();
    response_bytes.pop();

    let response: InboundMessage = serde_json::from_slice(&response_bytes).unwrap();

    println!("{}: {}", LOG_VERBOSE, format!("Response: {:?}", response));
    std::io::stdout().flush().unwrap();
}
