/*
Mutex con múltiples hilos — versión funcional (parte5/01_mutex.rs)

Imperativo: for loop + push + for loop para join.
Funcional: (0..10).map() para crear hilos, .collect() + .into_iter().for_each() para join.
*/

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let contador = Arc::new(Mutex::new(0));

    // (0..10).map() crea 10 hilos, cada uno con su Arc clonado.
    // .collect() recopila los JoinHandles.
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let contador = Arc::clone(&contador);
            thread::spawn(move || {
                *contador.lock().unwrap() += 1;
            })
        })
        .collect();

    // .into_iter().for_each() espera a que todos terminen.
    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("contador = {}", *contador.lock().unwrap());
}
