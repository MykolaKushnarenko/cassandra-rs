use shared::connection::Connection;
use shared::consistent_hash_ring::{ConsistentHashRing, Node, Range};
use shared::protocol::types::{Request, Response};
use std::io::Write;
use std::net::TcpStream;
use text_colorizer::Colorize;

const NODES_ARG_KEY: &str = "--nodes=";

const LOG_INFO: &str = "INFO";
const LOG_ERROR: &str = "ERROR";
const LOG_VERBOSE: &str = "VERBOSE";

fn main() {
    let mut nodes = vec![
        Node {
            address: "localhost:3000".to_string(),
        },
        Node {
            address: "localhost:4000".to_string(),
        },
        Node {
            address: "localhost:5001".to_string(),
        },
    ];
    let mut hasher = ConsistentHashRing::new(nodes.clone());

    println!(
        "{}: {}",
        LOG_INFO.bright_green(),
        "Adding data to the cluster".green().bold()
    );
    for i in 0..3000 {
        let value = i.to_string();
        let node_address = hasher.get_node_address(&value);
        let request = Request::Add(value);
        send(request, node_address);
    }
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
        rebalance(input.trim().to_string(), &mut hasher, &mut nodes);
    }
}

fn rebalance(new_node: String, hasher: &mut ConsistentHashRing, nodes: &mut Vec<Node>) {
    let new_node = Node {
        address: new_node.clone(),
    };
    let ranges = add_node(new_node.clone(), hasher);

    for range in ranges.iter() {
        println!("{}: {}", LOG_VERBOSE, format!("Range: {:?}", range));
    }

    let mut rebalanced_items: Vec<String> = Vec::new();

    for node in nodes.iter() {
        let stream_result = TcpStream::connect(node.address.as_str()).unwrap();
        let mut connection = Connection::new(&stream_result);

        let request = Request::GetBatch(ranges.clone());
        let mut response = connection.send_request_with_response(request).unwrap();
        match response {
            Response::Array(ref mut data) => {
                rebalanced_items.append(data);
                println!(
                    "{}: {}",
                    LOG_VERBOSE,
                    format!("[{}] Received data: {:?}", node.address, data)
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
    }

    let stream_result = TcpStream::connect(new_node.address.trim()).unwrap();
    let add_range_request = Request::AddBatch(rebalanced_items);
    let mut connection = Connection::new(&stream_result);
    let op_result = connection.send_request_with_response(add_range_request);

    match op_result {
        Ok(response) => println!(
            "{}: {}",
            LOG_VERBOSE,
            format!("Received response: {:?}", response)
        ),
        Err(error) => println!(
            "{}: {}",
            LOG_VERBOSE,
            format!("Received error: {:?}", error)
        ),
    }

    for node in nodes.iter() {
        let stream_result = TcpStream::connect(node.address.as_str()).unwrap();
        let mut connection = Connection::new(&stream_result);

        let request = Request::DropBatch(ranges.clone());
        let response = connection.send_request_with_response(request).unwrap();
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
    }
    
    println!("{}: {}", LOG_VERBOSE, format!("Rebalanced with new node {}", new_node.address));
    nodes.push(new_node);
}

fn send(request: Request, node_address: &str) {
    let stream_result = TcpStream::connect(node_address);

    let stream;
    match stream_result {
        Ok(value) => stream = value,
        Err(_) => {
            eprintln!(
                "{}: {}",
                LOG_ERROR.bright_red(),
                format!("Cannot connect to server {}", node_address)
                    .red()
                    .bold()
            );
            return;
        }
    }

    let mut connection = Connection::new(&stream);

    let response_result = connection.send_request_with_response(request);

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

    println!(
        "{} - [{}]: {}",
        LOG_VERBOSE,
        node_address,
        format!("Response: {:?}", response)
    );
    std::io::stdout().flush().unwrap();
}

fn add_node(node: Node, ring: &mut ConsistentHashRing) -> Vec<Range> {
    let address = node.address.clone();
    ring.add_node(node);

    ring.get_ranges_for_node(address.as_str())
}
