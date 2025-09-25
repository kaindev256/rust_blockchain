// proof_of_work.rs
// Implementa el algoritmo Proof of Work (PoW).
// Aquí se usa una versión simplificada: genera hashes hasta encontrar un valor aceptable.

use crate::block::Block;
use num_bigint::BigInt;
use num_traits::One;
use sha2::{Sha256, Digest};

// Número máximo de intentos de nonce
pub const MAX_NONCE: i64 = 1_000_000;

// Estructura que contiene el bloque a minar y el target de dificultad
pub struct ProofOfWork {
    pub block: Block,
    pub target: BigInt,
}

impl ProofOfWork {
    // Crea un nuevo PoW para un bloque dado
    pub fn new(block: Block) -> Self {
        // Se define un target muy alto (dificultad baja para pruebas)
        let mut target = BigInt::one();
        target = (&target << 240); 
        ProofOfWork { block, target }
    }

    // Ejecuta el proceso de minado
    pub fn run(&self) -> (i64, String) {
        let mut nonce: i64 = 0;
        while nonce < MAX_NONCE {
            // Prepara los datos del bloque
            let mut hasher = Sha256::new();
            hasher.update(self.block.prev_hash.as_bytes());
            for tx in &self.block.transactions {
                hasher.update(tx);
            }
            hasher.update(self.block.timestamp.to_string().as_bytes());
            hasher.update(nonce.to_string().as_bytes());
            
            // Calcula el hash
            let hash = hasher.finalize();
            let hex = hex::encode(hash.as_slice());

            // En esta versión simple aceptamos el primer hash generado
            return (nonce, hex);
        }
        (nonce, String::new())
    }
}
