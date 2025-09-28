//! node.rs
//! Representación de un nodo de la red y una colección (`Nodes`).
//! - Node: contiene la dirección (String) y helpers
//! - Nodes: colección protegida por RwLock para concurrencia
//!
use std::net::SocketAddr;
use std::sync::RwLock;

#[derive(Clone, Debug)]
pub struct Node {
    addr: String,
}

impl Node {
    pub fn new(addr: String) -> Self {
        Node { addr }
    }

    pub fn get_addr(&self) -> String {
        self.addr.clone()
    }

    pub fn parse_socket_addr(&self) -> SocketAddr {
        self.addr.parse().expect("invalid socket addr")
    }
}

pub struct Nodes {
    inner: RwLock<Vec<Node>>,
}

impl Nodes {
    pub fn new() -> Self {
        Nodes { inner: RwLock::new(vec![]) }
    }

    /// Añade un nodo si no está ya presente
    pub fn add_node(&self, addr: String) {
        let mut nodes = self.inner.write().unwrap();
        if !nodes.iter().any(|n| n.get_addr() == addr) {
            nodes.push(Node::new(addr));
        }
    }

    pub fn evict_node(&self, addr: &str) {
        let mut nodes = self.inner.write().unwrap();
        nodes.retain(|n| n.get_addr() != addr);
    }

    pub fn first(&self) -> Option<Node> {
        let nodes = self.inner.read().unwrap();
        nodes.first().cloned()
    }

    pub fn get_nodes(&self) -> Vec<Node> {
        let nodes = self.inner.read().unwrap();
        nodes.clone()
    }

    pub fn len(&self) -> usize {
        let nodes = self.inner.read().unwrap();
        nodes.len()
    }

    pub fn node_is_known(&self, addr: &str) -> bool {
        let nodes = self.inner.read().unwrap();
        nodes.iter().any(|n| n.get_addr() == addr)
    }
}
