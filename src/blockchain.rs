// blockchain.rs
// Maneja la cadena de bloques completa usando una base de datos (sled).
// Define la estructura Blockchain y sus métodos para crear el génesis, añadir bloques y obtener datos.

use sled;
use std::sync::{Arc, RwLock};
use crate::block::Block;
use crate::transaction::Transaction;
use std::env::current_dir;
use sled::transaction::TransactionResult;
use std::collections::HashMap;

// Nombre del árbol de bloques en la DB
pub const BLOCKS_TREE: &str = "blocks_tree";
// Clave especial para guardar el hash del último bloque
pub const TIP_BLOCK_HASH_KEY: &str = "L";

// Representa la blockchain completa
pub struct Blockchain {
    pub db: sled::Db,
    pub tip_hash: Arc<RwLock<String>>,
}

impl Blockchain {
    // Crea una nueva blockchain, con bloque génesis si no existía antes
    pub fn create_blockchain(genesis_address: &str) -> Blockchain {
        let db = sled::open(current_dir().unwrap().join("data")).unwrap();
        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();
        let data = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
        let tip_hash: String;
        
        if data.is_none() {
            // No existía: creamos el génesis
            let coinbase_tx = Transaction::new_coinbase_tx(genesis_address);
            let genesis = Block::generate_genesis_block(&coinbase_tx);

            let _ = blocks_tree.insert(genesis.get_hash(), genesis.serialize()).unwrap();
            let _ = blocks_tree.insert(TIP_BLOCK_HASH_KEY, genesis.get_hash()).unwrap();
            tip_hash = String::from_utf8(genesis.get_hash()).unwrap();
        } else {
            // Ya existía blockchain, usamos el tip guardado
            tip_hash = String::from_utf8(data.unwrap().to_vec()).unwrap();
        }

        Blockchain {
            db,
            tip_hash: Arc::new(RwLock::new(tip_hash)),
        }
    }

    // Helper para crear un blockchain nuevo con dirección fija
    pub fn new_blockchain() -> Result<Blockchain, sled::Error> {
        Ok(Self::create_blockchain("genesis-address"))
    }

    pub fn get_db(&self) -> &sled::Db {
        &self.db
    }

    pub fn get_tip_hash(&self) -> String {
        self.tip_hash.read().unwrap().clone()
    }

    pub fn set_tip_hash(&self, new_tip_hash: &str) {
        let mut tip_hash = self.tip_hash.write().unwrap();
        *tip_hash = new_tip_hash.to_string();
    }

    // Añade un bloque nuevo al blockchain
    pub fn add_block(&self, block: &Block) {
        let block_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        if block_tree.get(block.get_hash()).unwrap().is_some() {
            return;
        }

        let _: TransactionResult<(), ()> = block_tree.transaction(|tx_db| {
            let _ = tx_db.insert(block.get_hash(), block.serialize()).unwrap();
            let tip = self.get_tip_hash();
            let tip_bytes = tx_db.get(tip).unwrap().expect("Tip not found");
            let tip_block = Block::deserialize(tip_bytes.as_ref());

            // Actualizamos tip si este bloque es más alto
            if block.get_height() > tip_block.get_height() {
                let _ = tx_db.insert(TIP_BLOCK_HASH_KEY, block.get_hash()).unwrap();
                self.set_tip_hash(std::str::from_utf8(&block.get_hash()).unwrap());
            }
            Ok(())
        });
    }

    // Encuentra UTXOs — placeholder simplificado
    pub fn find_utxo(&self) -> HashMap<String, Vec<Vec<u8>>> {
        HashMap::new()
    }
}
