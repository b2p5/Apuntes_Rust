
# Bitcoin RPC Client en Rust

Este proyecto proporciona un cliente RPC simple escrito en Rust para interactuar con un nodo de Bitcoin. Utiliza solicitudes HTTP para comunicarse con el nodo y obtener información a través de llamadas RPC.

## Funcionalidades

- **Leer Archivo .cookie**: Lee las credenciales necesarias para la autenticación con el nodo Bitcoin desde un archivo `.cookie`.
- **Realizar Solicitudes RPC**: Envía solicitudes RPC a un nodo Bitcoin y procesa las respuestas.

## Requisitos

- Rust
- Cargo (gestor de paquetes de Rust)
- Un nodo de Bitcoin en ejecución con acceso RPC habilitado

## Dependencias

- `reqwest`: Para realizar solicitudes HTTP.
- `serde_json`: Para trabajar con datos en formato JSON.
- `base64`: Para codificación y decodificación en Base64.
- `tokio`: Como runtime para la ejecución asíncrona.

## Instalación

Asegúrate de tener instalado Rust y Cargo. Luego, clona este repositorio y navega hasta la carpeta del proyecto.

```bash
git clone [URL del repositorio]
cd [nombre del proyecto]
```

## Uso

El script principal (`main.rs`) realiza las siguientes funciones:

1. **`read_bitcoin_cookie`**: Esta función lee el contenido del archivo `.cookie` del nodo Bitcoin, necesario para la autenticación. Asegúrate de especificar la ruta correcta al archivo `.cookie` en tu sistema. Has de 
modificar "/tu ruta al fichero de/" por la ruta donde se encuentre el 
archivo .cookie el cual debe contener "usuario:contraseña" (sin comillas)

2. **`bitcoin_rpc_request`**: Esta función asíncrona envía una solicitud RPC al nodo Bitcoin. Requiere dos parámetros:
   - `method`: El método RPC a invocar.
   - `params`: Los parámetros del método RPC en formato JSON.

3. **Punto de entrada `main`**: Un ejemplo de cómo utilizar la función `bitcoin_rpc_request` para obtener información sobre el blockchain de Bitcoin (`getblockchaininfo`).

### Ejemplo

Para ejecutar el cliente RPC y obtener información del blockchain, utiliza el comando:

```bash
cargo run
```

Esto enviará una solicitud RPC al nodo Bitcoin y mostrará la respuesta en la consola.

## Configuración

Asegúrate de configurar correctamente la URL y el puerto del servidor RPC en la función `bitcoin_rpc_request`, según tu configuración de nodo de Bitcoin. Así como modificar "/tu ruta al fichero de/" .cookie

## Contribuciones

Las contribuciones al proyecto son bienvenidas. Por favor, asegúrate de seguir las mejores prácticas de programación en Rust y de probar tu código antes de enviar un pull request.

## Licencia

[Incluir detalles de la licencia aquí, si corresponde]

---

Este README proporciona una visión general del cliente RPC en Rust para Bitcoin. Para más detalles, revisa el código fuente y los comentarios incluidos en él.