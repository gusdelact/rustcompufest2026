/*
Cliente TCP básico

Este programa se conecta a un servidor TCP, le envía un mensaje,
y lee la respuesta. Es la contraparte de 02_servidor.rs.

En C, esto sería: socket() → connect() → write() → read() → close().
En Rust, TcpStream encapsula todo eso y se cierra automáticamente
al salir de scope (RAII).
*/

// Read: trait que proporciona el método `read()` para leer bytes.
// Write: trait que proporciona `write_all()` para escribir bytes.
use std::io::{Read, Write};
// TcpStream: representa una conexión TCP establecida.
// Implementa Read y Write, así que se puede leer y escribir como un archivo.
use std::net::TcpStream;

/// Punto de entrada del programa.
///
/// Se conecta al servidor en 127.0.0.1:7878, envía "Hola servidor\n",
/// lee la respuesta y la imprime.
///
/// # Errores
/// Retorna error si no puede conectarse o si falla la lectura/escritura.
fn main() -> std::io::Result<()> {
    // `TcpStream::connect()` establece una conexión TCP con el servidor.
    // Internamente hace socket() + connect() del sistema operativo.
    // `?` propaga el error si el servidor no está escuchando o la conexión falla.
    // `mut` porque vamos a leer y escribir sobre el stream.
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;

    // Enviamos bytes al servidor.
    // `b"Hola servidor\n"` es un byte string literal (&[u8]).
    // `write_all()` garantiza que se envíen TODOS los bytes.
    // A diferencia de `write()`, que puede enviar solo una parte,
    // `write_all()` reintenta hasta completar o falla con error.
    stream.write_all(b"Hola servidor\n")?;

    // Buffer de 1024 bytes para recibir la respuesta.
    // Inicializado en ceros. Es un array fijo en el stack.
    let mut buffer = [0u8; 1024];

    // `read()` lee bytes del stream al buffer.
    // Retorna `n`: el número de bytes leídos.
    // Puede leer menos de 1024 si el servidor envió menos datos.
    // Si `n == 0`, significa que el servidor cerró la conexión.
    let n = stream.read(&mut buffer)?;

    // Imprimimos solo los primeros `n` bytes del buffer (los que realmente se leyeron).
    // `&buffer[..n]` es un slice de los bytes leídos.
    // `String::from_utf8_lossy()` convierte bytes a texto, reemplazando
    // bytes inválidos con el carácter de reemplazo Unicode (�).
    println!("Respuesta: {}", String::from_utf8_lossy(&buffer[..n]));

    // `stream` sale de scope aquí y se cierra automáticamente (RAII).
    // En C, tendrías que llamar close(fd) manualmente.
    Ok(())
}
