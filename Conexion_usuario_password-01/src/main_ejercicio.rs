// Importación de las librerías necesarias desde el crate bitcoincore_rpc.
// Auth y Client son utilizados para la autenticación y conexión con el nodo Bitcoin Core.
// RpcApi proporciona los métodos para interactuar con la API RPC del nodo.
// Ver el fichero Cargo.toml para más información sobre la dependencia de bitcoincore_rpc.
use bitcoincore_rpc::{Auth, Client, RpcApi};

// Constantes para la autenticación. USER y PWS son el usuario y la contraseña respectivamente.
// Estos valores deben ser reemplazados con las credenciales reales del nodo Bitcoin Core.
const USER:&str = "tu usuario";
const PWS:&str  = "tu contraseña";

// Función principal del programa.
fn main() {

    // Definición de la URL para la conexión con el nodo Bitcoin Core.
    // Esta URL debe apuntar a la dirección donde el nodo está escuchando.
    // Por defecto, el puerto 8332 es utilizado para la interfaz RPC.
    let rpc_url  =  

    // Creación del objeto de autenticación usando el usuario y contraseña.
    // Estos son necesarios para realizar solicitudes autenticadas al nodo.
    let rpc_auth =  

    // Creación de un cliente para interactuar con el nodo Bitcoin Core.
    // Este cliente utilizará la URL y las credenciales de autenticación definidas anteriormente.
    // Si la conexión falla, el programa mostrará un mensaje de error y terminará.
    let _client =  

    // Solicitar y mostrar la información del blockchain del nodo Bitcoin Core.
    // Esta llamada devuelve datos como la altura actual del blockchain, la dificultad, etc.
    // Los datos se imprimen en un formato legible gracias a {:#?}.
    println!( );

}
