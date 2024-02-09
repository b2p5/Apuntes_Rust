# Servidor API para obtener las Txs de la Mempool Bitcoin

Este proyecto es un servidor web desarrollado en Rust, utilizando el framework Rocket para exponer una API que permite consultar las transacciones no confirmadas (mempool) de un nodo Bitcoin Core. Utiliza `bitcoincore_rpc` para la comunicación con Bitcoin Core y `serde` para el manejo de datos en formato JSON.

## Características

- **Consulta de Mempool**: Permite obtener una lista de las transacciones no confirmadas en el mempool de un nodo Bitcoin Core.
- **API Web**: Expone una interfaz de programación de aplicaciones (API) para acceder a los datos del mempool mediante solicitudes HTTP GET.
- **Serialización JSON**: Usa `serde_json` para serializar las transacciones del mempool a formato JSON, facilitando su integración con otros sistemas o aplicaciones web.
- **Concurrencia Segura**: Maneja el acceso concurrente a los datos del mempool mediante `Arc` y `Mutex`, garantizando la seguridad en el acceso y la modificación de los datos.

## Requisitos

Para ejecutar este proyecto, necesitarás:

- Rust (última versión estable recomendada)
- Cargo (incluido con Rust)
- Un nodo de Bitcoin Core en ejecución con la API JSON-RPC habilitada

## Configuración

1. **Nodo Bitcoin Core**: Asegúrate de que tu nodo Bitcoin Core esté configurado correctamente para aceptar conexiones JSON-RPC. Esto generalmente implica ajustar el archivo `bitcoin.conf` para incluir `rpcuser`, `rpcpassword`, `rpcport` y `server=1`.

2. **Variables de Entorno**: Configura las constantes `USER` y `PWS` en el script para que coincidan con tu usuario y contraseña de la API JSON-RPC de Bitcoin Core.

## Uso

Para ejecutar el servidor:

1. Clona este repositorio a tu máquina local.
2. Cambia USER y PWS en main.rs
3. Navega al directorio del proyecto en una terminal.
4. Ejecuta `cargo run` para iniciar el servidor Rocket.
5. Accede a la API mediante `http://localhost:8000/mempool` para obtener las transacciones no confirmadas del mempool en formato JSON.

## Contribuir

Las contribuciones son bienvenidas. Si deseas contribuir al proyecto, por favor, envía un pull request o abre un issue para discutir los cambios propuestos.


## Contacto

Para más información o consultas, por favor, contacta a [tu correo electrónico] o abre un issue en este repositorio.
