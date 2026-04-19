/*
Servidor multihilo — versión funcional (parte7/03_servidor_multihilo.rs)

Imperativo: for + match + spawn.
Funcional: .incoming().flatten().for_each(spawn).
*/

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn manejar_cliente(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];

    // Encadenamiento funcional: read → map para procesar → unwrap_or_else para error.
    stream
        .read(&mut buffer)
        .map(|n| {
            println!(
                "Thread {:?} recibió: {}",
                thread::current().id(),
                String::from_utf8_lossy(&buffer[..n])
            );
            stream.write_all(b"OK\n").unwrap_or_else(|e| {
                eprintln!("Error enviando: {}", e);
            });
        })
        .unwrap_or_else(|e| eprintln!("Error leyendo: {}", e));
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Servidor concurrente funcional en 127.0.0.1:7878");

    // .incoming() → Iterator<Result<TcpStream>>
    // .flatten() → descarta errores de accept
    // .for_each() → lanza un hilo por conexión
    listener
        .incoming()
        .flatten()
        .for_each(|stream| {
            thread::spawn(move || manejar_cliente(stream));
        });

    Ok(())
}
