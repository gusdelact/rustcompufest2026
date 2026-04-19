/*
Pipeline con hilos — versión funcional (parte4/03_pipeline_threads.rs)

Mismo patrón productor → transformador → consumidor,
pero cada etapa usa cadenas de iteradores en lugar de for loops.
(Este es el mismo que 05_pipeline_funcional.rs pero con nombres de parte4)
*/

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    // Productor: array → iterador → map → for_each(send)
    thread::spawn(move || {
        ["uno", "dos", "tres"]
            .iter()
            .map(|s| s.to_string())
            .for_each(|s| tx1.send(s).unwrap());
    });

    // Transformador: rx.iter() → map(uppercase) → for_each(send)
    thread::spawn(move || {
        rx1.iter()
            .map(|msg| msg.to_uppercase())
            .for_each(|msg| tx2.send(msg).unwrap());
    });

    // Consumidor: rx.iter() → enumerate → for_each(print)
    rx2.iter()
        .enumerate()
        .for_each(|(i, msg)| println!("{}: {}", i + 1, msg));
}
