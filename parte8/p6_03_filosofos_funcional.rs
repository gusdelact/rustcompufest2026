/*
Filósofos Comensales — versión funcional (parte6/03_filosofos_comensales.rs)

Imperativo: Vec con map manual + for loop para join.
Funcional: (0..5).map() para crear tenedores y hilos, .collect() + .into_iter().for_each() para join.

⚠️ Misma advertencia de deadlock que el original.
*/

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Crear tenedores con iterador funcional.
    let forks: Vec<_> = (0..5)
        .map(|_| Arc::new(Mutex::new(())))
        .collect();

    // Crear hilos de filósofos con .map() y .collect().
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let left = Arc::clone(&forks[i]);
            let right = Arc::clone(&forks[(i + 1) % 5]);

            thread::spawn(move || {
                let (_l, _r) = (left.lock().unwrap(), right.lock().unwrap());
                println!("Filósofo {} comiendo", i);
            })
        })
        .collect();

    // Esperar a todos con .into_iter().for_each().
    handles.into_iter().for_each(|h| h.join().unwrap());
}
