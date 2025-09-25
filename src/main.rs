// main.rs
// Programa principal: prueba mínima del blockchain.
// Crea la estructura de datos, inicializa la base de datos y confirma que funciona.

use rust_blockchain::blockchain::Blockchain;

fn main() {
    println!("Iniciando prueba de blockchain (capítulo 4)...");
    
    // Crea un nuevo blockchain (con bloque génesis si no existía antes)
    let _bc = Blockchain::new_blockchain().expect("No se pudo crear blockchain");
    
    println!("Blockchain creada correctamente.");
}

