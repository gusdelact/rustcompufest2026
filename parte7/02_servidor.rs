/*
Servidor TCP secuencial (un cliente a la vez)

Este servidor escucha en un puerto, acepta conexiones una por una,
lee lo que el cliente envía, y responde. Es secuencial: mientras
atiende a un cliente, los demás esperan en la cola del sistema operativo.

En C: socket() → bind() → listen() → accept() → read() → write() → close().
En Rust: TcpListener::bind() → .incoming() → read/write sobre TcpStream.
*/

// Read: trait para leer bytes (método `read()`).
// Write: trait para escribir bytes (método `write_all()`).
use std::io::{Read, Write};
// TcpListener: escucha conexiones entrantes en un puerto.
// TcpStream: representa una conexión TCP individual con un cliente.
use std::net::{TcpListener, TcpStream};

/// Atiende a un cliente individual.
///
/// Lee los datos que el cliente envía, los imprime en consola,
/// y responde con "Hola cliente\n".
///
/// # Argumentos
/// * `stream` - La conexión TCP con el cliente. Se toma por valor (`mut`)
///   porque necesitamos leer y escribir sobre ella.
///
/// # Errores
/// Retorna error si falla la lectura o escritura.
fn manejar_cliente(mut stream: TcpStream) -> std::io::Result<()> {
    // Buffer de 1024 bytes para recibir datos del cliente.
    let mut buffer = [0u8; 1024];

    // Leemos lo que el cliente envió.
    // `n` es el número de bytes leídos.
    let n = stream.read(&mut buffer)?;

    // Imprimimos los datos recibidos, convertidos a texto.
    // Solo usamos los primeros `n` bytes (los que realmente se leyeron).
    println!("Recibido: {}", String::from_utf8_lossy(&buffer[..n]));

    // Enviamos una respuesta al cliente.
    stream.write_all(b"Hola cliente\n")?;

    // `stream` se cierra automáticamente al salir de la función (RAII).
    Ok(())
}

/// Punto de entrada del programa.
///
/// Crea un servidor TCP que escucha en 127.0.0.1:7878 y atiende
/// clientes de forma secuencial (uno a la vez).
///
/// # Errores
/// Retorna error si no puede hacer bind al puerto (por ejemplo,
/// si otro proceso ya lo está usando).
fn main() -> std::io::Result<()> {
    // `TcpListener::bind()` crea un socket, lo asocia al puerto 7878,
    // y empieza a escuchar conexiones entrantes.
    // Internamente hace socket() + bind() + listen() del SO.
    // `?` propaga el error si el puerto ya está en uso.
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Servidor escuchando en 127.0.0.1:7878");

    // `listener.incoming()` devuelve un iterador infinito de conexiones.
    // Cada iteración bloquea hasta que un cliente se conecte.
    // Cada `stream` es un `Result<TcpStream, io::Error>`.
    for stream in listener.incoming() {
        match stream {
            // Si la conexión se aceptó correctamente, atendemos al cliente.
            Ok(stream) => {
                // `if let Err(e)` captura solo el caso de error.
                // Si `manejar_cliente` falla, imprimimos el error
                // pero seguimos aceptando nuevas conexiones.
                if let Err(e) = manejar_cliente(stream) {
                    eprintln!("Error atendiendo cliente: {}", e);
                }
            }
            // Si hubo un error al aceptar la conexión (raro, pero posible),
            // lo reportamos y seguimos escuchando.
            Err(e) => eprintln!("Error aceptando conexión: {}", e),
        }
    }

    // Este Ok(()) nunca se alcanza porque el loop es infinito.
    // El servidor se detiene con Ctrl+C.
    Ok(())
}
