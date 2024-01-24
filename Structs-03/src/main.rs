// Importación de la biblioteca bitcoincore_rpc para interactuar con Bitcoin Core
use bitcoincore_rpc::{Auth, Client, RpcApi};

// Constantes para la autenticación en el nodo de Bitcoin Core
const USER:&str = "tu_usuario";
const PWS:&str  = "tu_contraseña";

// Estructura que representa una transacción en el Mempool
#[derive(Debug)]
struct MempoolTransaction {
    txid: String, // Identificador de la transacción (TXID)
    // Campos adicionales pueden ser añadidos aquí (por ejemplo, tamaño, tarifas, tiempo de espera)
}

// Estructura para almacenar el Mempool completo
#[derive(Debug)]
struct RawMempool {
    transactions: Vec<MempoolTransaction>, // Vector para almacenar las transacciones
}

// Implementación de métodos para la estructura RawMempool
impl RawMempool {
    // Método para añadir una transacción al Mempool
    fn add_transaction(&mut self, tx: MempoolTransaction) {
        self.transactions.push(tx);
    }

    // Método para obtener el número de transacciones en el Mempool
    fn get_transactions_count(&self) -> usize {
        self.transactions.len()
    }

    // Método para obtener una transacción específica del Mempool por TXID
    fn get_transaction(&self, txid: &str) -> Option<&MempoolTransaction> {
        self.transactions.iter().find(|tx| tx.txid == txid)
    }

    // Aquí pueden implementarse otros métodos útiles (eliminar transacciones, buscar por otros criterios, etc.)
}

fn main() {
    // Configuración de la conexión con el nodo Bitcoin Core
    let rpc_url  = "http://localhost:8332"; // URL del nodo
    let rpc_auth = Auth::UserPass(USER.to_string(), PWS.to_string()); // Autenticación
    let client = Client::new(rpc_url, rpc_auth).expect("Error al conectar con Bitcoin Core");

    // Obtener datos del Mempool del nodo Bitcoin Core
    let raw_mempool_data = client.get_raw_mempool().expect("Error al obtener información del Mempool");

    // Creación de una instancia de RawMempool
    let mut raw_mempool = RawMempool { transactions: Vec::new() };
    for txid in raw_mempool_data {
        // Crear y añadir transacciones al Mempool
        let tx = MempoolTransaction { txid: txid.to_string() };
        raw_mempool.add_transaction(tx);
    }

    // Mostrar las transacciones en el Mempool
    println!("Las transacciones en el Mempool son: {:#?}", raw_mempool);

    // Mostrar el número de transacciones en el Mempool
    println!("Hay {} transacciones en el Mempool \n", raw_mempool.get_transactions_count());
    
    // Buscar y mostrar una transacción específica en el Mempool por TXID
    let mi_txid = "6458c77a3970961323fe961c49eceef47f024aa0735268ff142b7d0eab6514d5";
    let tx = raw_mempool.get_transaction(mi_txid);
    println!("La transacción con el Txid {} es: {:#?}", mi_txid, tx);

    // Imprime la transacción número 10 del Mempool
    // Asegúrate de que haya al menos 10 transacciones en el Mempool para evitar un error de índice
    if raw_mempool.transactions.len() > 10 {
        let tx = raw_mempool.get_transaction(&raw_mempool.transactions[10].txid);
        println!("La transacción número 10 del Mempool es: {:#?}", tx);
    } else {
        println!("No hay suficientes transacciones en el Mempool para mostrar la décima transacción.");
    }
}
