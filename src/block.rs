// block.rs
// Define la estructura de un bloque en la blockchain.
// Cada bloque contiene hash propio, hash previo, transacciones, altura y timestamp.
// Incluye funciones para crear bloques nuevos, serializarlos y obtener su información.

use serde::{Serialize, Deserialize};
use bincode;
use crate::proof_of_work::ProofOfWork;
use crate::transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};

// Representa un bloque en la blockchain
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub hash: String,             // hash del bloque
    pub prev_hash: String,        // hash del bloque anterior
    pub transactions: Vec<Vec<u8>>, // transacciones serializadas
    pub height: usize,            // altura del bloque en la cadena
    pub timestamp: i64,           // tiempo de creación
}

impl Block {
    // Crea un nuevo bloque a partir de hash previo y transacciones
    pub fn new(prev_hash: String, transactions: Vec<Vec<u8>>, height: usize) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH).unwrap()
            .as_secs() as i64;
        
        let mut b = Block {
            hash: String::new(),
            prev_hash,
            transactions,
            height,
            timestamp,
        };

        // Ejecuta Proof of Work para generar hash válido
        let pow = ProofOfWork::new(b.clone());
        let (_nonce, hash_hex) = pow.run();
        b.hash = hash_hex;
        b
    }

    // Genera el bloque génesis (primer bloque)
    pub fn generate_genesis_block(coinbase_tx: &Transaction) -> Self {
        let tx_ser = coinbase_tx.serialize();
        Block::new(String::from(""), vec![tx_ser], 0)
    }

    // Serializa el bloque a bytes
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    // Deserializa un bloque desde bytes
    pub fn deserialize(bytes: &[u8]) -> Block {
        bincode::deserialize(bytes).unwrap()
    }

    pub fn get_hash(&self) -> Vec<u8> {
        self.hash.as_bytes().to_vec()
    }

    pub fn get_transactions(&self) -> Vec<Vec<u8>> {
        self.transactions.clone()
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}
