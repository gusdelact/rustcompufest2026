/*
Mutex<T>
Arc = referencia compartida segura entre threads
Mutex = candado
lock() = entrar a sección crítica
al salir del scope, se libera el candado
*/

use std::sync::{Arc, Mutex};
use std::thread;

/// Programa que demuestra el uso de concurrencia segura en Rust
/// utilizando `Arc` (Atomic Reference Counted) y `Mutex` (Mutual Exclusion).
///
/// Se crean 10 hilos que incrementan un contador compartido de forma segura.
/// - `Arc` permite compartir la propiedad del dato entre múltiples hilos.
/// - `Mutex` garantiza que solo un hilo acceda al dato a la vez, evitando condiciones de carrera.
fn main() {
    // Creamos un contador compartido protegido por un Mutex y envuelto en un Arc.
    // - `Mutex::new(0)`: crea un mutex que protege el valor inicial 0.
    // - `Arc::new(...)`: permite que múltiples hilos tengan propiedad compartida del Mutex.
    let contador = Arc::new(Mutex::new(0));

    // Vector para almacenar los handles (manejadores) de cada hilo creado,
    // lo que nos permitirá esperar a que todos terminen antes de continuar.
    let mut handles = vec![];

    // Lanzamos 10 hilos concurrentes
    for _ in 0..10 {
        // Clonamos el Arc para que cada hilo tenga su propia referencia al contador.
        // Esto incrementa el conteo de referencias atómico, no duplica el dato.
        let contador = Arc::clone(&contador);

        // Creamos un nuevo hilo con `thread::spawn`.
        // `move` transfiere la propiedad del Arc clonado al closure del hilo.
        let h = thread::spawn(move || {
            // `lock()` adquiere el mutex, bloqueando el hilo hasta que esté disponible.
            // `unwrap()` maneja el caso de un mutex "envenenado" (si otro hilo falló mientras lo tenía).
            // `n` es un MutexGuard que permite acceder al valor protegido.
            let mut n = contador.lock().unwrap();

            // Incrementamos el valor del contador en 1.
            // El MutexGuard se libera automáticamente al salir del scope (RAII).
            *n += 1;
        });

        // Guardamos el handle del hilo para poder esperar su finalización después.
        handles.push(h);
    }

    // Esperamos a que todos los hilos terminen su ejecución.
    // `join()` bloquea el hilo principal hasta que el hilo asociado finalice.
    for h in handles {
        h.join().unwrap();
    }

    // Imprimimos el valor final del contador.
    // Debería ser 10, ya que cada uno de los 10 hilos incrementó el valor en 1.
    println!("contador = {}", *contador.lock().unwrap());
}