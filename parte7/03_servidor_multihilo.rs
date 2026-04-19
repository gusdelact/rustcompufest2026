/*
Servidor TCP concurrente (un hilo por cliente)

Evolución del servidor secuencial: cada cliente se atiende en su propio hilo.
Esto permite atender múltiples clientes simultáneamente.

El patrón es el clásico "thread-per-connection":
- El hilo principal acepta conexiones.
- Cada conexión se delega a un hilo nuevo.
- Los hilos corren en paralelo.

⚠️ Limitación: crear un hilo por conexión no escala bien con miles de clientes.
Para eso se usan thread pools o async (tokio).
*/

// Read: trait para leer bytes.
// Write: trait para escribir bytes.
use std::io::{Read, Write};
// TcpListener: escucha conexiones entrantes.
// TcpStream: conexión TCP individual.
use std::net::{TcpListener, TcpStream};
// thread: para crear hilos de ejecución.
use std::thread;

/// Atiende a un cliente individual en su propio hilo.
///
/// Lee los datos del cliente, imprime qué hilo los recibió,
/// y responde con "OK\n".
///
/// # Argumentos
/// * `stream` - La conexión TCP con el cliente.
fn manejar_cliente(mut stream: TcpStream) {
    // Buffer de 1024 bytes para recibir datos.
    let mut buffer = [0u8; 1024];

    match stream.read(&mut buffer) {
        Ok(n) => {
            // Imprimimos el id del hilo que está atendiendo esta conexión.
            // `thread::current().id()` devuelve un identificador único del hilo.
            // Esto demuestra que cada cliente se atiende en un hilo diferente.
            println!(
                "Thread {:?} recibió: {}",
                thread::current().id(),
                String::from_utf8_lossy(&buffer[..n])
            );

            // Enviamos respuesta al cliente.
            // Usamos `if let Err(e)` para manejar el error sin hacer panic.
            if let Err(e) = stream.write_all(b"OK\n") {
                eprintln!("Error enviando respuesta: {}", e);
            }
        }
        // Si falla la lectura, reportamos el error.
        Err(e) => eprintln!("Error leyendo: {}", e),
    }
    // `stream` se cierra automáticamente al salir de la función.
}

/// Punto de entrada del programa.
///
/// Crea un servidor TCP concurrente que escucha en 127.0.0.1:7878.
/// Cada conexión entrante se atiende en un hilo separado.
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Servidor concurrente en 127.0.0.1:7878");

    // Iteramos sobre las conexiones entrantes.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // `thread::spawn` crea un nuevo hilo para cada conexión.
                // `move` transfiere la propiedad del `stream` al hilo.
                // El hilo principal queda libre para aceptar la siguiente conexión
                // inmediatamente, sin esperar a que este cliente termine.
                thread::spawn(move || {
                    manejar_cliente(stream);
                });
            }
            Err(e) => eprintln!("Error aceptando conexión: {}", e),
        }
    }

    Ok(())
}
