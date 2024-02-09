// Macros de Rocket para habilitar características específicas, como el routing y el manejo de requests.
#[macro_use] extern crate rocket;
// Importación de las crates `serde` y `serde_json` para la serialización y deserialización de datos en formato JSON.
extern crate serde;
extern crate serde_json;

// Importaciones de Rocket y Serde para el manejo de datos serializables y respuestas HTTP.
use rocket::serde::{Deserialize, Serialize};
use rocket::{response::{self, Responder, Response}, http::ContentType};

// Importaciones del crate `bitcoincore_rpc` para interactuar con la API JSON-RPC de un nodo de Bitcoin Core.
use bitcoincore_rpc::{Auth, Client};
use bitcoincore_rpc::RpcApi;

// Importaciones estándar de Rust para el manejo de concurrencia y sincronización de datos.
use std::thread;
use std::sync::{Arc, Mutex};
use rocket::State;

// Constantes para la autenticación con el nodo Bitcoin Core.
const USER: &str = "tu usuario";
const PWS: &str  = "tu contraseña";

// Definición de una estructura para almacenar transacciones del mempool y su implementación.
#[derive(Serialize, Deserialize)]
struct MempoolTx {
    txid: Vec<String>,
}

impl MempoolTx {
    fn new() -> MempoolTx {
        MempoolTx {
            txid: Vec::new(),
        }
    }
}

// Definición de una ruta en el servidor web para obtener datos del mempool.
#[get("/mempool")]
fn get_mempool(get_raw_mempool: &State<Arc<Mutex<MempoolTx>>>,) -> JsonResponse {
    // Bloqueo y acceso a los datos del mempool, luego se serializan a JSON.
    let get_raw_mempool = get_raw_mempool.lock().unwrap();
    let stringified_json = serde_json::to_string(&get_raw_mempool.txid).unwrap();

    JsonResponse(stringified_json)
}

// Definición de una estructura para enviar respuestas JSON personalizadas.
pub struct JsonResponse(String);

// Implementación del trait `Responder` para `JsonResponse`, permitiendo su uso como respuesta HTTP.
impl<'r> Responder<'r, 'static> for JsonResponse {
    fn respond_to(self, _: &'r rocket::Request) -> response::Result<'static> {
        Response::build()
            .header(ContentType::JSON) // Establece el tipo de contenido de la respuesta como JSON.
            .sized_body(self.0.len(), std::io::Cursor::new(self.0)) // Añade el cuerpo de la respuesta JSON.
            .ok()
    }
}

// Punto de entrada para la aplicación Rocket, configurando el servidor web.
#[launch]
fn rocket() -> _ {
    // Inicialización y gestión del estado compartido para almacenar datos del mempool.
    let get_raw_mempool = Arc::new(Mutex::new(MempoolTx::new()));
    let get_raw_mempool_clone = Arc::clone(&get_raw_mempool);

    // Configuración del cliente RPC para interactuar con Bitcoin Core.
    let rpc_url  = "http://localhost:8332";
    let rpc_auth = Auth::UserPass(USER.to_string(), PWS.to_string());
    let client = Client::new(rpc_url, rpc_auth).expect("Error to connect Bitcoin Core");
    
    // Creación de un hilo para actualizar los datos del mempool de forma asíncrona.
    thread::spawn(move || {
        let get_raw_mempool = client.call("getrawmempool", &[serde_json::Value::Bool(false)]).unwrap();
        let mut mempool_tx = get_raw_mempool_clone.lock().unwrap();
        mempool_tx.txid = get_raw_mempool;
    }); 

    // Construcción y lanzamiento del servidor web Rocket, montando la ruta y gestionando el estado.
    rocket::build()
        .mount("/", routes![get_mempool])
        .manage(get_raw_mempool)
}
