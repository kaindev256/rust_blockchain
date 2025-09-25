// transaction.rs
// Define la estructura de una transacción.
// En esta versión inicial es muy simplificada: solo tiene un id y un payload (data).
// Más adelante se extenderá con inputs, outputs y firmas.

use serde::{Serialize, Deserialize};
use bincode;
use sha2::{Sha256, Digest};

// Transacción de la blockchain
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: Vec<u8>,   // identificador hash
    pub data: String,  // payload de la transacción
}

impl Transaction {
    // Crea una nueva transacción con payload arbitrario
    pub fn new(data: &str) -> Self {
        let mut tx = Transaction {
            id: vec![],
            data: data.to_string(),
        };
        tx.set_id();
        tx
    }

    // Transacción coinbase: recompensa para el minero (bloque génesis o nuevos bloques)
    pub fn new_coinbase_tx(address: &str) -> Self {
        Transaction::new(&format!("coinbase to {}", address))
    }

    // Calcula y establece el id de la transacción (hash de su contenido)
    pub fn set_id(&mut self) {
        let ser = bincode::serialize(&self.data).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(ser);
        self.id = hasher.finalize().to_vec();
    }

    pub fn get_id(&self) -> &[u8] {
        &self.id
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn deserialize(bytes: &[u8]) -> Transaction {
        bincode::deserialize(bytes).unwrap()
    }
}
