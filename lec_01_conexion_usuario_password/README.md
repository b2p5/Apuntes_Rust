# Bitcoin Core RPC Client en Rust

Este proyecto contiene un script en Rust diseñado para interactuar con un nodo de Bitcoin Core a través de su interfaz RPC. El script establece una conexión con el nodo, autenticándose con credenciales de usuario, y recupera información básica del blockchain.

## Requisitos

- **Rust:** Este script requiere Rust para su compilación y ejecución. Puede instalar Rust desde [la página oficial](https://www.rust-lang.org/tools/install).
- **Bitcoin Core:** Se necesita un nodo de Bitcoin Core en ejecución. Puede descargarlo e instalarlo desde [bitcoincore.org](https://bitcoincore.org/en/download/).
- **Credenciales de RPC:** Asegúrese de tener configurado el acceso RPC en su nodo de Bitcoin Core, con un usuario y contraseña definidos.

## Configuración

Antes de ejecutar el script, es necesario configurar las credenciales y la URL del nodo de Bitcoin Core. Modifique las constantes `USER` y `PWS` en el script para reflejar su usuario y contraseña de RPC.

```rust
const USER:&str = "tu usuario"; // Reemplace 'tu usuario' con su usuario
const PWS:&str  = "tu clave";   // Reemplace 'tu clave' con su contraseña


## Uso

Para usar el script, siga estos pasos:

1. Compile el programa con el comando `cargo build`.
2. Ejecute el programa compilado con `cargo run`.
3. Al ejecutar, el script se conectará al nodo de Bitcoin Core y mostrará información del blockchain en la consola.

## Funcionalidad

El script realiza las siguientes operaciones:

1. Establece una conexión con el nodo Bitcoin Core usando las credenciales de RPC.
2. Utiliza la función `get_blockchain_info` para obtener información del blockchain.
3. Imprime esta información en un formato legible.

