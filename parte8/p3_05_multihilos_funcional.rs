/*
Múltiples hilos — versión funcional (parte3/05_move_multihilos.rs)

Imperativo: for loop + push a Vec + for loop para join.
Funcional: (0..3).map() para crear hilos, .collect() para recopilar handles,
           .into_iter().for_each() para join.
*/

use std::thread;

fn main() {
    // (0..3).map() crea un iterador que lanza un hilo por cada valor.
    // .collect::<Vec<_>>() recopila los JoinHandles.
    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                println!("soy el hilo {i}");
            })
        })
        .collect();

    // .into_iter() consume el Vec (toma ownership de cada handle).
    // .for_each() llama join() en cada uno.
    handles
        .into_iter()
        .for_each(|h| h.join().unwrap());
}
