/*
Un ejemplo mínimo de move con varios hilos
*/

// Importamos el módulo `thread` de la biblioteca estándar,
// que nos permite crear y gestionar hilos de ejecución.
use std::thread;

fn main() {
    // Creamos un vector mutable para almacenar los "handles" (manejadores)
    // de cada hilo. Estos handles nos permiten esperar a que cada hilo termine.
    let mut handles = Vec::new();

    // Iteramos 3 veces (i = 0, 1, 2) para crear 3 hilos concurrentes.
    for i in 0..3 {
        // `thread::spawn` crea un nuevo hilo de ejecución.
        // Usamos `move` para transferir la propiedad de `i` al closure,
        // ya que el hilo podría vivir más que el scope donde se creó `i`.
        let h = thread::spawn(move || {
            // Cada hilo imprime su identificador.
            println!("soy el hilo {i}");
        });

        // Guardamos el handle del hilo en el vector para poder
        // esperar su finalización más adelante.
        handles.push(h);
    }

    // Recorremos todos los handles y llamamos a `join()` en cada uno.
    // `join()` bloquea el hilo principal hasta que el hilo asociado termine.
    // `unwrap()` propaga cualquier panic que haya ocurrido dentro del hilo.
    for h in handles {
        h.join().unwrap();
    }
}