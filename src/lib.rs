//! lib.rs
//! Punto de entrada de la librería del proyecto.
//! Exporta los módulos principales para que `main.rs` u otras herramientas (CLI) los usen.
//! (Esta organización no cambia la lógica del libro; solo facilita la importación).

pub mod config;
pub mod block;
pub mod blockchain;
pub mod transaction;
pub mod proof_of_work;
pub mod network;
pub mod node;

