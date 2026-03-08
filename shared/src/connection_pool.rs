use crate::cluster::RoutingStrategy;
use crate::connection::Connection;
use crate::error::AppResult;
use crate::protocol::types::{Request, Response};
use std::collections::HashMap;
use std::net::TcpStream;

/// A connection pool that manages persistent TCP connections to cluster nodes.
///
/// The pool maintains a cache of connections keyed by node address, automatically
/// creating new connections as needed and reusing existing ones to avoid the
/// overhead of repeated TCP handshakes.
///
/// # Routing Strategies
///
/// The pool supports three routing strategies:
/// - **Direct**: Sends request to a single node
/// - **Fanout**: Broadcasts request to multiple nodes and merges responses
pub struct ConnectionPool {
    connections: HashMap<String, Connection>,
}

impl ConnectionPool {
    /// Creates a new empty connection pool.
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    /// Executes a request based on the provided routing strategy.
    ///
    /// This method handles three types of routing strategies:
    /// - `Direct`: Sends request to a single node and returns its response
    /// - `Fanout`: Broadcasts request to multiple nodes and merges their responses into a single array
    ///
    /// # Arguments
    ///
    /// * `strategy` - The routing strategy determining how the request is distributed
    /// * `request` - The request to execute
    /// * `exclude_node` - Optional node address to exclude from fanout operations (used during rebalancing)
    ///
    /// # Returns
    ///
    /// Returns the response from the node(s), merged if multiple nodes were queried.
    ///
    pub fn execute(
        &mut self,
        strategy: RoutingStrategy,
        request: Request,
        exclude_node: Option<&str>,
    ) -> AppResult<Response> {
        match strategy {
            RoutingStrategy::Direct(node_addr) => {
                let connection = self.get_or_create_connection(node_addr.address.as_str())?;
                connection.send_request_with_response(&request)
            }

            RoutingStrategy::Fanout(nodes) => {
                let mut all_responses = Vec::new();

                for node_addr in nodes {
                    if exclude_node.is_some() && node_addr.address == exclude_node.unwrap() {
                        continue;
                    }
                    let connection = self.get_or_create_connection(node_addr.address.as_str())?;
                    let response = connection.send_request_with_response(&request)?;
                    all_responses.push(response.clone());
                    println!("{}: Response {:?}", node_addr.address, response);
                }

                Self::merge_responses(all_responses)
            }
        }
    }

    /// Get existing connection or create new one
    fn get_or_create_connection(&mut self, node_addr: &str) -> AppResult<&mut Connection> {
        // Check if connection exists
        if !self.connections.contains_key(node_addr) {
            // Create new connection
            let stream = TcpStream::connect(node_addr)
                .map_err(|e| crate::error::Error::ConnectionError(Some(e)))?;
            let connection = Connection::new(stream);
            self.connections.insert(node_addr.to_string(), connection);
        }

        // Return mutable reference
        Ok(self.connections.get_mut(node_addr).unwrap())
    }

    /// Merge multiple responses into one
    fn merge_responses(responses: Vec<Response>) -> AppResult<Response> {
        let mut merged = Vec::new();

        for response in responses {
            match response {
                Response::Array(values) => merged.extend(values),
                Response::String(s) => merged.push(s),
                Response::Bool(_) => {} // Skip bools in merge
            }
        }

        Ok(Response::Array(merged))
    }
}
