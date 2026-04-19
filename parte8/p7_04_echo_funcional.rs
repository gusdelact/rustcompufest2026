/*
Servidor Echo — versión funcional (parte7/04_net_echo.rs)

Imperativo: for stream in listener.incoming() { match { spawn } }
Funcional: listener.incoming().flatten().for_each(|stream| spawn)

La función echo interna usa .read() en un loop que no se puede
reemplazar fácilmente con iteradores (no hay iterador de read()),
pero el manejo de conexiones sí se beneficia del estilo funcional.
*/

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn echo(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0u8; 512];
    loop {
        let n = stream.read(&mut buffer)?;
        if n == 0 { break; }
        stream.write_all(&buffer[..n])?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9000")?;

    // .incoming() devuelve Iterator<Result<TcpStream>>.
    // .flatten() descarta errores de conexión.
    // .for_each() lanza un hilo por cada conexión válida.
    listener
        .incoming()
        .flatten()
        .for_each(|stream| {
            thread::spawn(move || {
                if let Err(e) = echo(stream) {
                    eprintln!("Error: {}", e);
                }
            });
        });

    Ok(())
}
