# Interacción con el Mempool de Bitcoin Core

Este proyecto proporciona una herramienta en Rust para interactuar con el mempool de un nodo de Bitcoin Core. Permite realizar diversas operaciones como recuperar, contar y buscar transacciones específicas en el mempool.

Todo ello se realiza con `structs`, una vez que se han leido las Txs de Bitcoin Core estas se almacenan en la struct RawMempool la cual depende de MempoolTransaction.

## Características

- Conexión con un nodo de Bitcoin Core.
- Recuperación de las transacciones en el mempool.
- Conteo de las transacciones en el mempool.
- Búsqueda de una transacción específica por TXID.
- Visualización de detalles de transacciones específicas.

## Requisitos Previos

Asegúrese de tener instalado Rust y acceso a un nodo de Bitcoin Core. Este proyecto está desarrollado y probado en `Ubuntu 22`.

## Configuración

1. **Configuración del Nodo Bitcoin Core**: Asegúrese de que su nodo Bitcoin Core esté configurado correctamente para aceptar conexiones RPC. Necesitará las credenciales de usuario y contraseña para conectarse al nodo.

2. **Variables de Entorno**: Configure las constantes `USER` y `PWS` en el script con su usuario y contraseña del nodo Bitcoin Core.

## Instalación y Ejecución

1. **Clonar Repositorio**: Clone este repositorio en su máquina local utilizando `git clone`.

2. **Instalar Dependencias**: Navegue hasta el directorio del proyecto y ejecute `cargo build` para instalar todas las dependencias necesarias.

3. **Ejecutar el Script**: Ejecute el proyecto con `cargo run`.

## Estructura del Proyecto

- `main.rs`: Contiene el código principal para interactuar con el mempool.
- `mempool_data.rs`: Define las estructuras y métodos para manejar los datos del mempool.
- `api.rs`: (Si aplica) Contiene la lógica para exponer funcionalidades como una API.

## Dependencias

- `bitcoincore_rpc`: Una biblioteca en Rust para comunicarse con el nodo Bitcoin Core a través de RPC.

## Contribuciones

Las contribuciones a este proyecto son bienvenidas. Por favor, siga las mejores prácticas de desarrollo y mantenga el código bien documentado.

## Licencia

Este proyecto se distribuye bajo la licencia [LICENCIA-DESEADA], que permite el uso, distribución y modificación.

## Contacto

Si tiene alguna pregunta o sugerencia, no dude en contactarme en [Correo/Perfil de GitHub].

---


