/*
Productor-Consumidor — versión funcional (parte6/01_productor_consumidor.rs)

Imperativo: for i in 0..5 { tx.send(i) } + for recibido in rx { println }
Funcional: (0..5).for_each(send) + rx.iter().for_each(print)
*/

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        (0..5).for_each(|i| tx.send(i).unwrap());
    });

    rx.iter().for_each(|recibido| println!("Consumido: {}", recibido));
}
