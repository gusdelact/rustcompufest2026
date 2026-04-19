/*
Servidor Echo — devuelve todo lo que recibe

El servidor echo es el "Hello World" de la programación de red.
Cada byte que el cliente envía, el servidor lo devuelve tal cual.
Es útil para probar conectividad y entender el flujo de datos TCP.

Se puede probar con:
  telnet 127.0.0.1 9000
  nc 127.0.0.1 9000

Cada línea que escribas se devuelve inmediatamente.
*/

// Read: trait para leer bytes (método `read()`).
// Write: trait para escribir bytes (método `write_all()`).
use std::io::{Read, Write};
// TcpListener: escucha conexiones entrantes.
// TcpStream: conexión TCP individual.
use std::net::{TcpListener, TcpStream};
// thread: para crear un hilo por conexión.
use std::thread;

/// Implementa el protocolo echo para una conexión individual.
///
/// Lee datos del cliente en un loop y los reenvía de vuelta.
/// El loop termina cuando el cliente cierra la conexión (`n == 0`).
///
/// # Argumentos
/// * `stream` - La conexión TCP con el cliente.
///
/// # Errores
/// Retorna error si falla una lectura o escritura.
fn echo(mut stream: TcpStream) -> std::io::Result<()> {
    // Buffer de 512 bytes para recibir datos.
    // Cada iteración del loop lee hasta 512 bytes.
    let mut buffer = [0u8; 512];

    // Loop infinito: seguimos leyendo mientras el cliente envíe datos.
    loop {
        // `read()` bloquea hasta que lleguen datos o el cliente cierre.
        // Retorna `n`: el número de bytes leídos.
        let n = stream.read(&mut buffer)?;

        // Si `n == 0`, el cliente cerró su extremo de la conexión (EOF).
        // En TCP, esto significa que el cliente hizo shutdown o close.
        // Salimos del loop para terminar este hilo.
        if n == 0 {
            break;
        }

        // Reenviamos exactamente los mismos bytes que recibimos.
        // `&buffer[..n]` es un slice con solo los bytes leídos.
        // `write_all()` garantiza que se envíen todos los bytes.
        // Si el cliente envió "hola\n", le devolvemos "hola\n".
        stream.write_all(&buffer[..n])?;
    }

    // El cliente cerró la conexión. `stream` se cierra al salir (RAII).
    Ok(())
}

/// Punto de entrada del programa.
///
/// Crea un servidor echo concurrente que escucha en 127.0.0.1:9000.
/// Cada conexión se atiende en un hilo separado.
/// El servidor devuelve todo lo que cada cliente le envía.
fn main() -> std::io::Result<()> {
    // Escuchamos en el puerto 9000.
    // Usamos un puerto diferente al de los otros servidores (7878)
    // para poder tener ambos corriendo al mismo tiempo.
    let listener = TcpListener::bind("127.0.0.1:9000")?;

    // Iteramos sobre las conexiones entrantes (loop infinito).
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Cada conexión se atiende en su propio hilo.
                // `move` transfiere el `stream` al hilo.
                thread::spawn(move || {
                    // Si `echo()` retorna error, lo imprimimos.
                    // `if let Err(e)` solo captura el caso de error.
                    if let Err(e) = echo(stream) {
                        eprintln!("Error: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("No se pudo aceptar conexión: {}", e),
        }
    }

    Ok(())
}
