#[macro_use] extern crate rocket;
extern crate serde;
extern crate serde_json;
use rocket::serde::{ Deserialize, Serialize};
use rocket::{ response::{self, Responder, Response}, http::ContentType};


#[derive(Serialize, Deserialize)]
struct MempoolTx {
    txid: String,
}

// Estructura para manejar la respuesta JSON
pub struct JsonResponse(String);
impl<'r> Responder<'r, 'static> for JsonResponse {
    fn respond_to(self, _: &'r rocket::Request) -> response::Result<'static> {
        Response::build()
            .header(ContentType::JSON)
            .sized_body(self.0.len(), std::io::Cursor::new(self.0))
            .ok()
    }
}

#[get("/mempool")]
fn get_mempool() -> JsonResponse {
    // Aquí deberías insertar la lógica para obtener la mempool real desde tu nodo de Bitcoin.
    // Este ejemplo solo devuelve datos ficticios.
    let mock_data = vec![
        MempoolTx { txid: "txid1".to_string() },
        MempoolTx { txid: "txid2".to_string() },
    ];

    let ascen_descen_for_txid_string = serde_json::to_string(&mock_data).unwrap();
    JsonResponse(ascen_descen_for_txid_string)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_mempool])
}
