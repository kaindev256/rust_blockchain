//! main.rs
//! Programa principal: inicializa blockchain y server (esquema minimal).

use rust_blockchain::blockchain::Blockchain;
use rust_blockchain::config::NODE_ADDR;
use rust_blockchain::network::{serve, GLOBAL_NODES, send_version};
use std::thread;

fn main() {
    // Mensaje inicial
    println!("Iniciando prueba de blockchain (capítulo 4)...");

    // Creamos/abrimos la blockchain (con genesis si hace falta)
    let bc = Blockchain::new_blockchain().expect("No se pudo crear blockchain");
    println!("Blockchain creada. tip: {}", bc.get_tip_hash());

    // Leer dirección del nodo desde la config (puedes cambiar NODE_ADDR antes de correr)
    let addr = NODE_ADDR.read().unwrap().clone();
    println!("Nodo escuchando en: {}", addr);

    // Ejemplo: añadir un nodo bootstrap si no existe (simulado)
    {
        let gn = GLOBAL_NODES.lock().unwrap();
        gn.add_node(addr.clone());
    }

    // Lanzamos el servidor en un hilo — handler simple imprime el paquete entrante
    thread::spawn(move || {
        serve(&addr, |pkg| {
            println!("Paquete recibido en handler: {:?}", pkg);
            // En el libro aquí se procesa cada variante y se actualiza blockchain, mempool, etc.
        });
    });

    // Enviamos un version al nodo central (ejemplo)
    send_version("127.0.0.1:3000", 0);

    // Pausa simple para que el servidor corra (en una app real esperas señales)
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("Terminado (demo).");
}
