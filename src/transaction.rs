//! transaction.rs
//! Estructura mínima de transacción para los ejemplos del libro.
//! - contiene id (hash) y un payload simple (message) para coinbase.
//! - funciones: new_coinbase_tx, serialize/deserialize, get_id.

use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use bincode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: Vec<u8>,
    pub data: String, // simplificación: payload textual
}

impl Transaction {
    /// Construye una transacción coinbase de ejemplo (coinbase tiene "recompensa")
    pub fn new_coinbase_tx(to: &str) -> Self {
        let data = format!("Coinbase to: {}", to);
        let mut tx = Transaction { id: vec![], data };
        tx.id = tx.calc_id();
        tx
    }

    fn calc_id(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.data.as_bytes());
        hasher.finalize().to_vec()
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn deserialize(raw: &[u8]) -> Self {
        bincode::deserialize(raw).unwrap()
    }

    pub fn get_id(&self) -> &Vec<u8> {
        &self.id
    }
}
