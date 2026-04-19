/*
Channels — versión funcional (parte5/04_channels.rs)

Imperativo: spawn + send + recv.
Funcional: el productor usa una cadena de iteradores para generar y enviar.
El consumidor usa .iter() sobre el receiver.
*/

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Productor: genera mensajes con una cadena funcional.
    thread::spawn(move || {
        (1..=5)
            .map(|i| format!("mensaje {}", i))
            .for_each(|msg| tx.send(msg).unwrap());
    });

    // Consumidor: rx.iter() es un iterador que termina cuando tx se destruye.
    // .enumerate() agrega índice.
    // .for_each() imprime.
    rx.iter()
        .enumerate()
        .for_each(|(i, msg)| println!("{}: {}", i, msg));
}
