//! proof_of_work.rs
//! Implementación simplificada de Proof-of-Work.
//! - Para el ejemplo: prueba con un target "con n ceros" sobre el hash hex.
//! - Devuelve nonce y hash en hexadecimal.

use crate::block::Block;
use sha2::{Digest, Sha256};
use data_encoding::HEXLOWER;

pub struct ProofOfWork {
    pub block: Block,
    pub target_zeroes: usize,
}

impl ProofOfWork {
    pub fn new(block: Block, target_zeroes: usize) -> Self {
        ProofOfWork { block, target_zeroes }
    }

    /// Ejecuta PoW: ciclo de nonce hasta que el hash hex tenga `target_zeroes` ceros al inicio.
    /// (Muy simple y lento para producción; sirve para aprendizaje.)
    pub fn run(&mut self) -> (i64, String) {
        let mut nonce: i64 = 0;
        loop {
            let mut trial = self.block.header_bytes();
            trial.extend_from_slice(&nonce.to_be_bytes());
            let hash = sha256(&trial);
            let hex = HEXLOWER.encode(&hash);
            if Self::hash_matches_target(&hex, self.target_zeroes) {
                return (nonce, hex);
            }
            nonce += 1;
            // nota: en ejemplos reales controlar límite o interrupción
        }
    }

    fn hash_matches_target(hex: &str, target_zeroes: usize) -> bool {
        hex.starts_with(&"0".repeat(target_zeroes))
    }
}

fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
