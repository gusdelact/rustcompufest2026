/*
RwLock<T>

Cuando muchos leen y pocos escriben.
*/

use std::sync::{Arc, RwLock};
use std::thread;

/// Punto de entrada del programa.
///
/// Demuestra el uso de `Arc<RwLock<T>>` para compartir datos de forma segura
/// entre múltiples hilos (threads). `Arc` (Atomic Reference Counted) permite
/// tener múltiples propietarios del mismo dato entre hilos, mientras que
/// `RwLock` (Read-Write Lock) permite acceso concurrente de lectura, pero
/// exclusivo de escritura, garantizando seguridad en tiempo de compilación.
fn main() {
    // Creamos un dato compartido protegido por un RwLock y envuelto en un Arc.
    // - `RwLock::new(10)`: crea un candado de lectura/escritura con el valor inicial 10.
    // - `Arc::new(...)`: envuelve el RwLock en un contador de referencias atómico,
    //   lo que permite clonarlo y compartirlo entre hilos de forma segura.
    let dato = Arc::new(RwLock::new(10));

    // --- Hilo lector ---
    // Este hilo solo necesita leer el valor compartido.
    let lector = {
        // Clonamos el Arc para que este hilo tenga su propia referencia al dato.
        // Esto incrementa el contador de referencias atómico, no copia el dato interno.
        let dato = Arc::clone(&dato);

        // `thread::spawn` crea un nuevo hilo del sistema operativo.
        // El closure usa `move` para transferir la propiedad del `dato` clonado al hilo.
        thread::spawn(move || {
            // `dato.read()` adquiere un bloqueo de lectura (shared lock).
            // Múltiples lectores pueden mantener este bloqueo simultáneamente.
            // `.unwrap()` desenvuelve el Result; paniquea si el lock está envenenado
            // (esto ocurre si otro hilo entró en pánico mientras tenía el lock).
            let n = dato.read().unwrap();

            // `*n` desreferencia el guard `RwLockReadGuard<i32>` para acceder al valor.
            // El bloqueo de lectura se libera automáticamente cuando `n` sale del scope.
            println!("lector ve {}", *n);
        })
    };

    // --- Hilo escritor ---
    // Este hilo necesita modificar el valor compartido.
    let escritor = {
        // Nuevamente clonamos el Arc para este segundo hilo.
        let dato = Arc::clone(&dato);

        thread::spawn(move || {
            // `dato.write()` adquiere un bloqueo de escritura (exclusive lock).
            // Solo un escritor puede mantener este bloqueo a la vez, y ningún lector
            // puede leer mientras el escritor tiene el lock. Esto previene data races.
            // `.unwrap()` desenvuelve el Result, igual que en el caso del lector.
            let mut n = dato.write().unwrap();

            // `*n += 5` modifica el valor interno a través del guard `RwLockWriteGuard<i32>`.
            // El bloqueo de escritura se libera automáticamente cuando `n` sale del scope.
            *n += 5;
        })
    };

        // --- Hilo lector ---
    // Este hilo solo necesita leer el valor compartido.
    let lector2 = {
        // Clonamos el Arc para que este hilo tenga su propia referencia al dato.
        // Esto incrementa el contador de referencias atómico, no copia el dato interno.
        let dato = Arc::clone(&dato);

        // `thread::spawn` crea un nuevo hilo del sistema operativo.
        // El closure usa `move` para transferir la propiedad del `dato` clonado al hilo.
        thread::spawn(move || {
            // `dato.read()` adquiere un bloqueo de lectura (shared lock).
            // Múltiples lectores pueden mantener este bloqueo simultáneamente.
            // `.unwrap()` desenvuelve el Result; paniquea si el lock está envenenado
            // (esto ocurre si otro hilo entró en pánico mientras tenía el lock).
            let n = dato.read().unwrap();

            // `*n` desreferencia el guard `RwLockReadGuard<i32>` para acceder al valor.
            // El bloqueo de lectura se libera automáticamente cuando `n` sale del scope.
            println!("lector ve {}", *n);
        })
    };

    // `join()` bloquea el hilo principal hasta que el hilo correspondiente termine.
    // Esto garantiza que ambos hilos completen su ejecución antes de que el programa finalice.
    // `.unwrap()` propaga cualquier pánico que haya ocurrido dentro del hilo.
    lector.join().unwrap();
    escritor.join().unwrap();
    lector2.join().unwrap();

    println!("{:?}",dato);
    // Nota: el orden de ejecución de los hilos no es determinista. El lector podría
    // ver el valor 10 (si lee antes de que el escritor escriba) o 15 (si lee después).
    // Para garantizar un orden específico se necesitarían mecanismos adicionales de
    // sincronización como barreras (`Barrier`) o canales (`mpsc::channel`).
}