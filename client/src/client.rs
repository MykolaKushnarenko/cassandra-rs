use shared::cluster::Cluster;
use shared::connection_pool::ConnectionPool;
use shared::consistent_hash_ring::Node;
use shared::error::AppResult;
use shared::protocol::types::{Request, Response};

pub struct Client {
    connection_pool: ConnectionPool,
    cluster: Cluster,
}

pub struct Settings {
    nodes: Vec<Node>,
}

impl Settings {
    pub fn new(nodes: Vec<Node>) -> Self {
        Settings { nodes }
    }
}

impl Client {
    pub fn new(settings: Settings) -> Self {
        Client {
            connection_pool: ConnectionPool::new(),
            cluster: Cluster::new(settings.nodes),
        }
    }

    pub fn send(&mut self, request: Request) -> AppResult<Response> {
        let router = self.cluster.router();
        let routing_strategy = router.route_request(&request);

        println!("Sending request to: {:?}", routing_strategy);

        self.connection_pool
            .execute(routing_strategy, request, None)
    }
}
