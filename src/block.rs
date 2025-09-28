//! block.rs
//! Definición de la estructura Block y funciones relacionadas.
//! - guarda transacciones (lista simple)
//! - provee serialize/deserialize con bincode (necesario para la persistencia y envío por red)
//! - funciones helper: new_block, generate_genesis_block, hash_transactions, getters.

use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use bincode;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub hash: Vec<u8>,
    pub prev_hash: Vec<u8>,
    pub transactions: Vec<Transaction>,
    pub timestamp: u64,
    pub height: u64,
}

impl Block {
    /// Crea un nuevo bloque (sin minar). La función de PoW se encuentra en proof_of_work.rs.
    pub fn new_block(prev_hash: Vec<u8>, transactions: Vec<Transaction>, height: u64) -> Self {
        let timestamp = now_timestamp();
        let mut b = Block {
            hash: vec![],
            prev_hash,
            transactions,
            timestamp,
            height,
        };

        // Para simplicidad, el hash inicial se calcula con contenido (PoW puede reescribir)
        let hdr = b.header_bytes();
        b.hash = sha256(&hdr);
        b
    }

    /// Genera el bloque genesis con una transacción coinbase
    pub fn generate_genesis_block(coinbase: &Transaction) -> Self {
        Block::new_block(vec![], vec![coinbase.clone()], 0)
    }

    /// Serializa con bincode
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    /// Deserializa con bincode
    pub fn deserialize(raw: &[u8]) -> Self {
        bincode::deserialize(raw).unwrap()
    }

    pub fn get_hash(&self) -> Vec<u8> {
        self.hash.clone()
    }

    pub fn get_height(&self) -> u64 {
        self.height
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    /// Encabezado simple para hashing
    pub fn header_bytes(&self) -> Vec<u8> {
        let mut out = vec![];
        out.extend_from_slice(&self.prev_hash);
        out.extend_from_slice(&self.timestamp.to_be_bytes());
        out.extend_from_slice(&self.height.to_be_bytes());
        // añadimos hashes de transacciones (concatenado)
        for tx in &self.transactions {
            out.extend_from_slice(&tx.id);
        }
        out
    }
}

/// helper: timestamp unix
fn now_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

/// sha256 de bytes
fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
