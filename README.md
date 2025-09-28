Perfecto 🙌, lo que quieres es un **bloque único de código Markdown** con todo el contenido del `README.md`, listo para copiar y pegar. Aquí lo tienes:

```markdown
# Rust Blockchain (Capítulo 4)

Este proyecto implementa una **blockchain simplificada en Rust**, siguiendo el libro  
**Rust for Blockchain Application Development** (hasta el capítulo 4).

El objetivo es **aprender los fundamentos de las blockchains**:
- cómo se representan bloques y transacciones,
- cómo se crea el bloque génesis,
- cómo se guarda la cadena en una base de datos (sled),
- cómo funciona un **Proof of Work** básico,
- y cómo se comunican los nodos en una red P2P.

⚠️ **Nota:** Este proyecto **no es seguro para producción**. Es un código educativo y simplificado, pensado para entender los conceptos principales.

---

## 🚀 ¿Qué hace?

- Crea una blockchain nueva con un **bloque génesis**.
- Permite añadir bloques con transacciones simples.
- Guarda los bloques en una base de datos embebida (`sled`).
- Implementa un algoritmo **Proof of Work (PoW)** didáctico.
- Simula una **red P2P**: nodos que se envían mensajes (`version`, `inv`, `getblocks`, `block`, `tx`).
- Incluye una estructura `Node` y `Nodes` para gestionar los nodos conocidos en la red.

---

## 📂 Estructura del proyecto

```

rust_blockchain/
├─ Cargo.toml          # Configuración del proyecto y dependencias
├─ src/
│  ├─ lib.rs           # Punto de entrada de la librería, exporta los módulos
│  ├─ main.rs          # Programa principal (ejemplo de uso de blockchain y red)
│  ├─ config.rs        # Configuración global (direcciones, timeouts)
│  ├─ block.rs         # Definición de Block (bloques de la cadena)
│  ├─ blockchain.rs    # Manejo de la cadena completa y almacenamiento en sled
│  ├─ transaction.rs   # Definición de transacciones (Coinbase incluida)
│  ├─ proof_of_work.rs # Implementación simplificada de Proof of Work
│  ├─ node.rs          # Representación de nodos y lista de nodos conocidos
│  └─ network.rs       # Funciones de red: mensajes, envío y recepción

````

---

## 🔎 Explicación de cada archivo

### `Cargo.toml`
Define el nombre del crate (`rust_blockchain`) y las dependencias:
- `serde`, `serde_json`, `bincode`: serialización
- `sled`: base de datos embebida
- `sha2`, `hex`: hashing
- `once_cell`: variables globales seguras
- `log`: logging básico

### `src/lib.rs`
Expone todos los módulos para que puedan ser usados desde `main.rs` o cualquier binario adicional.

### `src/main.rs`
Ejemplo de ejecución:
- Inicializa la blockchain (creando el bloque génesis si no existe).
- Configura un nodo y lo añade a la lista de nodos conocidos.
- Arranca un servidor TCP (`serve`) que escucha mensajes de otros nodos.
- Envía un mensaje `version` a un nodo central de ejemplo.

### `src/config.rs`
Centraliza configuración:
- Dirección del nodo (`127.0.0.1:3000` por defecto).
- Dirección del nodo central (bootstrap).
- Timeout TCP.

### `src/block.rs`
Define la estructura `Block`:
- Contiene hash, hash previo, transacciones, timestamp y altura.
- Incluye métodos para crear bloques (`new_block`), generar el génesis (`generate_genesis_block`), y serializar/deserializar.

### `src/blockchain.rs`
Gestiona la cadena completa:
- Guarda bloques en `sled` (árbol `blocks_tree`).
- Mantiene el `tip_hash` (último bloque).
- Funciones: `create_blockchain`, `add_block`, `find_utxo` (placeholder).

### `src/transaction.rs`
Define la estructura `Transaction`:
- ID (hash) y un `data` simple.
- Método especial `new_coinbase_tx` para crear la transacción del bloque génesis.
- Métodos de serialización y hashing.

### `src/proof_of_work.rs`
Implementa PoW simplificado:
- Ajusta el `nonce` hasta que el hash empiece con cierto número de ceros.
- Usa SHA-256 y salida en hex.
- Devuelve `(nonce, hash_hex)`.

### `src/node.rs`
Representa nodos de la red:
- `Node`: dirección de un nodo.
- `Nodes`: lista de nodos conocida, protegida por `RwLock`.
- Métodos para añadir, eliminar y consultar nodos.

### `src/network.rs`
Define la lógica de red:
- Enum `Package`: mensajes (`Version`, `GetBlocks`, `Inv`, `GetData`, `Block`, `Tx`).
- Funciones para enviar paquetes (`send_block`, `send_tx`, `send_inv`, etc.).
- Servidor básico `serve` que escucha mensajes entrantes.

---

## 🔗 Relación entre archivos

- **`main.rs`** usa la API expuesta en **`lib.rs`**.
- **`blockchain.rs`** depende de **`block.rs`** y **`transaction.rs`**.
- **`proof_of_work.rs`** depende de **`block.rs`** para minar bloques.
- **`network.rs`** depende de **`node.rs`** para manejar la lista de nodos y de **`config.rs`** para la dirección local.
- **`node.rs`** es usado por **`network.rs`** y **`main.rs`**.
- **`config.rs`** es usado en toda la red y en `main.rs`.

---

## ▶️ Cómo ejecutar

1. Clonar o crear el proyecto:
   ```bash
   git clone <repo>
   cd rust_blockchain
````

2. Compilar:

   ```bash
   cargo build
   ```

3. Ejecutar:

   ```bash
   cargo run
   ```

Verás en la terminal:

* La creación del blockchain y el bloque génesis.
* La inicialización de un nodo.
* El servidor escuchando en la dirección configurada.
* Un mensaje `version` enviado al nodo central de ejemplo.

---

## 📖 Próximos pasos

Hasta el capítulo 4 del libro, el proyecto **ya tiene una blockchain básica**: bloques, transacciones muy simples, PoW y comunicación de red inicial.  
Sin embargo, hay varias limitaciones importantes que deben resolverse para que el sistema sea más realista:

1. **Completar la lógica del servidor (`serve`)**  
   - Actualmente el servidor es un esqueleto: abre conexiones TCP, pero solo muestra la estructura básica de cómo debería reaccionar a cada tipo de mensaje (`Version`, `Inv`, `GetBlocks`, etc.).  
   - Falta implementar el manejo detallado: por ejemplo, cuando llega un `Inv`, el nodo debería pedir bloques que no tiene (`GetData`) y añadirlos a la cadena.  
   - Sin esto, los nodos no pueden **sincronizar** correctamente sus blockchains.

2. **Implementar correctamente la gestión de transacciones y UTXO**  
   - Ahora mismo la función `find_utxo` en `blockchain.rs` es solo un **placeholder** (devuelve un `HashMap` vacío).  
   - Esto significa que no se está calculando qué salidas no gastadas (UTXO) están disponibles en la blockchain.  
   - Sin una gestión de UTXO:  
     - No se puede verificar si una transacción es válida.  
     - No se pueden calcular balances reales.  
     - Todas las transacciones son aceptadas “a ciegas”.  
   - Implementar esto requiere recorrer toda la blockchain, rastrear outputs gastados y disponibles, y mantener un **UTXO set** actualizado.

3. **Añadir un CLI (interfaz de línea de comandos)**  
   - En el libro, más adelante, se introduce un CLI que permite consultar balances y enviar transacciones desde la terminal.  
   - Con el CLI, los usuarios podrían interactuar con su blockchain de forma más parecida a Bitcoin Core, en vez de solo ver `println!` en el main.

4. **Conectar varios nodos en distintas terminales**  
   - Ahora mismo el nodo arranca y envía un mensaje `version` al nodo central, pero la red sigue siendo mínima.  
   - Para simular una red real, se deben correr varios binarios (varias instancias del mismo programa) y dejar que intercambien bloques y transacciones.  
   - Esto permitirá ver cómo los nodos alcanzan consenso y mantienen la misma copia de la blockchain.

En resumen:  
👉 El proyecto actual es una **base funcional y educativa** hasta el capítulo 4.  
👉 Lo siguiente es darle **vida de red real**: validar transacciones con UTXO, sincronizar nodos entre sí y permitir interacción de usuarios mediante un CLI.

