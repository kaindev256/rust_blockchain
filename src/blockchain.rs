//! blockchain.rs
//! Implementa la estructura Blockchain y métodos para crear la db (sled),
//! añadir bloques, mantener el "tip" (hash del último bloque) y helpers.
//!
//! Sigue el esquema del capítulo 4 del libro, simplificado para que compile y sea fácil de entender.

use crate::block::Block;
use crate::transaction::Transaction;
use sled::{Db, transaction::TransactionResult};
use std::sync::{Arc, RwLock};
use std::env::current_dir;
use std::collections::HashMap;

/// Nombre del árbol dentro de sled para almacenar bloques
pub const BLOCKS_TREE: &str = "blocks_tree";
/// Clave para almacenar el tip (hash del último bloque)
pub const TIP_BLOCK_HASH_KEY: &str = "L";

pub struct Blockchain {
    pub db: Db,
    pub tip_hash: Arc<RwLock<String>>,
}

impl Blockchain {
    /// Crea la base de datos y, si no existe, crea el bloque genesis.
    pub fn create_blockchain(genesis_address: &str) -> Blockchain {
        let db_path = current_dir().unwrap().join("data");
        let db = sled::open(db_path).unwrap();
        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();
        let data = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
        let tip_hash: String;

        if data.is_none() {
            // crear genesis
            let coinbase_tx = Transaction::new_coinbase_tx(genesis_address);
            let genesis = Block::generate_genesis_block(&coinbase_tx);

            let _ = blocks_tree.insert(genesis.get_hash(), genesis.serialize()).unwrap();
            let _ = blocks_tree.insert(TIP_BLOCK_HASH_KEY, genesis.get_hash()).unwrap();
            tip_hash = hex::encode(genesis.get_hash());
        } else {
            tip_hash = hex::encode(data.unwrap().to_vec());
        }

        Blockchain {
            db,
            tip_hash: Arc::new(RwLock::new(tip_hash)),
        }
    }

    /// Constructor simple
    pub fn new_blockchain() -> Result<Blockchain, sled::Error> {
        Ok(Self::create_blockchain("genesis-address"))
    }

    pub fn get_db(&self) -> &Db {
        &self.db
    }

    pub fn get_tip_hash(&self) -> String {
        self.tip_hash.read().unwrap().clone()
    }

    /// Actualiza el tip (solo memoria). Para persistir, add_block actualiza la DB.
    pub fn set_tip_hash(&self, new_tip_hash: &str) {
        let mut tip_hash = self.tip_hash.write().unwrap();
        *tip_hash = new_tip_hash.to_string();
    }

    /// Añade un bloque a la DB (si no existe) y actualiza tip si es necesario.
    pub fn add_block(&self, block: &Block) {
        let block_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        if block_tree.get(block.get_hash()).unwrap().is_some() {
            // ya existe
            return;
        }

        // operación transaccional (sled)
        let _: TransactionResult<(), ()> = block_tree.transaction(|tx_db| {
            let _ = tx_db.insert(block.get_hash(), block.serialize()).unwrap();

            // obtener tip actual
            let tip_hash = self.get_tip_hash();
            let tip_bytes = tx_db.get(hex::decode(&tip_hash).unwrap()).unwrap()
                .expect("tip not found");
            let tip_block = Block::deserialize(tip_bytes.as_ref());

            // Actualizamos tip si el nuevo bloque tiene mayor altura
            if block.get_height() > tip_block.get_height() {
                let _ = tx_db.insert(TIP_BLOCK_HASH_KEY, block.get_hash()).unwrap();
                self.set_tip_hash(&hex::encode(block.get_hash()));
            }

            Ok(())
        });
    }

    /// Encuentra UTXO (placeholder simple). El libro desarrolla esto en capítulos posteriores.
    pub fn find_utxo(&self) -> HashMap<String, Vec<Vec<u8>>> {
        HashMap::new()
    }
}
