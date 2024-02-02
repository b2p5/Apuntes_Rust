// Importaciones de módulos y bibliotecas
use reqwest; // Biblioteca para realizar solicitudes HTTP
use serde_json::json; // Funcionalidad de serde_json para trabajar con JSON
use std::{fs, io}; // Módulos estándar para manejo de archivos e I/O
extern crate base64; // Biblioteca para codificación y decodificación Base64

// Función para leer el archivo .cookie de Bitcoin
fn read_bitcoin_cookie() -> io::Result<String> {
    // Asegúrate de especificar la ruta correcta al archivo .cookie
    // Esta se encuentra en bitcoin.conf en la carpeta de datos de Bitcoin
    let cookie_content = fs::read_to_string("/ ¿? tu ruta al fichero de/.cookie")?;
    // Trimear y convertir el contenido a String
    Ok(cookie_content. ¿? )
}

// Función asíncrona para realizar solicitudes RPC a Bitcoin
async fn bitcoin_rpc_request(method: &str, params: serde_json::Value) -> Result<serde_json::Value, reqwest::Error> {
    // Leer el archivo de cookies y manejar posibles errores
    let cookie = ¿? .expect("Error leyendo el archivo de cookies");

    // URL del servidor RPC
    let rpc_url = "http://127.0.0.1:8332";

    // Codificación Base64 de las credenciales contenidas en el archivo de cookies
    let encoded_cookie = ¿?;

    // Crear un cliente HTTP para enviar la solicitud
    let client = reqwest::Client::new();
    let res = client.post(rpc_url)
        .header("Authorization", format!(¿?, ¿?))
        .json(&json!({
            // Construir el cuerpo de la solicitud con los parámetros adecuados
            "jsonrpc": "2.0",
            "id": "rust-bitcoin-rpc",
            "method": method,
            "params": params
        }))
        .send()
        .await?; // Enviar la solicitud y esperar la respuesta

    // Parsear la respuesta a JSON
    let response_json = res.json::<serde_json::Value>().await?;
    Ok(response_json)
}

// Punto de entrada principal para programas asíncronos en Rust
#[tokio::main]
async fn main() {
    // Ejemplo de cómo llamar a la función bitcoin_rpc_request
    let response = bitcoin_rpc_request("getblockchaininfo", json!([])).await.expect("Error en la solicitud RPC");
    // Imprimir la respuesta en un formato legible
    println!("{:#?}", response);

}
