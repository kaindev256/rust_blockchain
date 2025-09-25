// network.rs
// Maneja la comunicación entre nodos de la red P2P.
// Define los mensajes (Package) y operaciones (OpType) que los nodos intercambian.
// Implementa send_data, que envía paquetes a otros nodos.

use serde::{Serialize, Deserialize};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use std::io::Write;
use log::{info, error};

// Versión del protocolo
pub const NODE_VERSION: usize = 1;
// Timeout en milisegundos para escritura en sockets
pub const TCP_WRITE_TIMEOUT: u64 = 2000; 

// Tipo de operación: enviar transacción o bloque
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OpType {
    Tx,
    Block,
}

// Paquete genérico que viaja entre nodos
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Package {
    Block { addr_from: String, block: Vec<u8> },
    GetBlocks { addr_from: String },
    GetData { addr_from: String, op_type: OpType, id: Vec<u8> },
    Inv { addr_from: String, op_type: OpType, items: Vec<Vec<u8>> },
    Tx { addr_from: String, transaction: Vec<u8> },
    Version { addr_from: String, version: usize, best_height: usize },
}

// Envía un paquete serializado a un nodo de la red
pub fn send_data(addr: SocketAddr, pkg: Package) {
    info!("send package: {:?}", &pkg);
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            let _ = stream.set_write_timeout(Some(Duration::from_millis(TCP_WRITE_TIMEOUT)));
            
            // Serializamos el paquete a JSON y lo enviamos por el stream
            if serde_json::to_writer(&mut stream, &pkg).is_err() {
                error!("failed to write package to stream");
            }
            // flush asegura que los datos se envíen inmediatamente
            let _ = stream.flush();
        }
        Err(e) => {
            error!("failed to connect {}: {}", addr, e);
            // En el libro aquí se eliminaría el nodo de la lista global
        }
    }
}
