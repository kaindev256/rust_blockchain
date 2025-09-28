//! network.rs
//! Funciones de red: empaquetado de mensajes (Package), envío y servidor básico (serve).
//! - send_data: conecta TCP y envía JSON (serde_json)
//! - helpers: send_block, send_tx, send_inv, send_version, send_get_blocks, send_get_data
//! - serve: función que procesa paquetes entrantes (esquema simplificado, siguiendo capítulo 4)

use serde::{Serialize, Deserialize};
use std::net::{TcpStream, TcpListener, Shutdown, SocketAddr};
use std::io::{BufReader, BufRead};
use crate::config::{TCP_WRITE_TIMEOUT_MS, NODE_ADDR};
use std::time::Duration;
use log::{info, error};
use crate::block::Block;
use crate::transaction::Transaction;
use crate::node::Nodes;
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Un contenedor global de nodos (simulación simple).
/// En el libro usan una variable global; aquí replicamos ese patrón con once_cell.
pub static GLOBAL_NODES: Lazy<Mutex<Nodes>> = Lazy::new(|| Mutex::new(Nodes::new()));

pub const NODE_VERSION: usize = 1;

#[derive(Serialize, Deserialize, Debug)]
pub enum Package {
    Version { addr_from: String, version: usize, best_height: u64 },
    GetBlocks { addr_from: String },
    Inv { addr_from: String, op_type: OpType, items: Vec<Vec<u8>> },
    GetData { addr_from: String, op_type: OpType, id: Vec<u8> },
    Block { addr_from: String, block: Vec<u8> },
    Tx { addr_from: String, transaction: Vec<u8> },
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum OpType {
    BlockType,
    TxType,
}

/// Envía un paquete serializado en JSON a la dirección `addr`.
pub fn send_data(addr: SocketAddr, pkg: &Package) {
    info!("send package: {:?}", pkg);
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            let _ = stream.set_write_timeout(Some(Duration::from_millis(TCP_WRITE_TIMEOUT_MS)));
            if serde_json::to_writer(&mut stream, pkg).is_err() {
                error!("failed to write package to stream");
            }
            let _ = stream.shutdown(Shutdown::Both);
        }
        Err(e) => {
            error!("failed to connect {}: {:?}", addr, e);
            // Evict node de la lista global si existe
            let gn = GLOBAL_NODES.lock().unwrap();
            gn.evict_node(&addr.to_string());
        }
    }
}

/// Envia GetData (request) a un peer
pub fn send_get_data(addr: &str, op_type: OpType, id: &[u8]) {
    let socket_addr = addr.parse().unwrap();
    let node_addr = NODE_ADDR.read().unwrap().clone();
    let pkg = Package::GetData { addr_from: node_addr, op_type, id: id.to_vec() };
    send_data(socket_addr, &pkg);
}

/// Envia inv (inventario)
pub fn send_inv(addr: &str, op_type: OpType, items: &[Vec<u8>]) {
    let socket_addr = addr.parse().unwrap();
    let node_addr = NODE_ADDR.read().unwrap().clone();
    let pkg = Package::Inv { addr_from: node_addr, op_type, items: items.to_vec() };
    send_data(socket_addr, &pkg);
}

/// Envia bloque
pub fn send_block(addr: &str, block: &Block) {
    let socket_addr = addr.parse().unwrap();
    let node_addr = NODE_ADDR.read().unwrap().clone();
    let pkg = Package::Block { addr_from: node_addr, block: block.serialize() };
    send_data(socket_addr, &pkg);
}

/// Envia transacción
pub fn send_tx(addr: &str, tx: &Transaction) {
    let socket_addr = addr.parse().unwrap();
    let node_addr = NODE_ADDR.read().unwrap().clone();
    let pkg = Package::Tx { addr_from: node_addr, transaction: tx.serialize() };
    send_data(socket_addr, &pkg);
}

/// Envia version
pub fn send_version(addr: &str, best_height: u64) {
    let socket_addr = addr.parse().unwrap();
    let node_addr = NODE_ADDR.read().unwrap().clone();
    let pkg = Package::Version { addr_from: node_addr, version: NODE_VERSION, best_height };
    send_data(socket_addr, &pkg);
}

/// Envia request para obtener bloques
pub fn send_get_blocks(addr: &str) {
    let socket_addr = addr.parse().unwrap();
    let node_addr = NODE_ADDR.read().unwrap().clone();
    let pkg = Package::GetBlocks { addr_from: node_addr };
    send_data(socket_addr, &pkg);
}

/// Serve: esquema simplificado del servidor que procesa paquetes entrantes.
/// El libro tiene una función `serve` extensa; aquí dejamos un esquema claro que puedes ampliar.
pub fn serve(my_addr: &str, blockchain_handler: impl Fn(Package) + Send + 'static) {
    let listener = TcpListener::bind(my_addr).expect("failed to bind listener");
    info!("Listening on {}", my_addr);
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let mut reader = BufReader::new(s);
                let mut buf = String::new();
                if reader.read_line(&mut buf).is_ok() {
                    if let Ok(pkg) = serde_json::from_str::<Package>(&buf) {
                        // llamamos al handler proporcionado (ej: en el libro procesan según el enum)
                        blockchain_handler(pkg);
                    }
                }
            }
            Err(e) => {
                error!("Connection failed: {:?}", e);
            }
        }
    }
}
