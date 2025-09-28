//! config.rs
//! Pequeña configuración global para el nodo (dirección por defecto, tiempo de escritura TCP, etc).
//! Esto es una mejora práctica que facilita pruebas y evita hardcodear strings en múltiples ficheros.

use once_cell::sync::Lazy;
use std::sync::RwLock;

/// Dirección del nodo por defecto (puedes cambiar al ejecutar)
pub static NODE_ADDR: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new("127.0.0.1:3000".to_string()));

/// Nodo central por defecto (simula el bootstrap node)
pub static CENTERAL_NODE: &str = "127.0.0.1:3000";

/// Timeout de escritura TCP en ms
pub const TCP_WRITE_TIMEOUT_MS: u64 = 5000;
