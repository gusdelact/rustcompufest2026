/*
Cliente TCP — versión funcional (parte7/01_cliente.rs)

Imperativo: connect → write_all → read → println.
Funcional: connect().and_then(enviar).and_then(recibir).map(imprimir).

Se encadenan las operaciones de I/O con combinadores sobre Result,
eliminando las variables intermedias y el flujo secuencial explícito.
*/

use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // Encadenamiento funcional:
    // connect() → Result<TcpStream>
    // .and_then() → envía el mensaje y devuelve el stream
    // .and_then() → lee la respuesta y devuelve (buffer, n)
    // .map() → imprime la respuesta
    TcpStream::connect("127.0.0.1:7878")
        .and_then(|mut stream| {
            // write_all devuelve Result<()>; con .map() preservamos el stream
            stream.write_all(b"Hola servidor\n").map(|_| stream)
        })
        .and_then(|mut stream| {
            let mut buffer = [0u8; 1024];
            // read devuelve Result<usize>; con .map() empaquetamos buffer y n
            stream.read(&mut buffer).map(|n| (buffer, n))
        })
        .map(|(buffer, n)| {
            // Solo imprimimos los bytes efectivamente leídos
            println!("Respuesta: {}", String::from_utf8_lossy(&buffer[..n]));
        })
}
