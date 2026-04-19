/*
Crear un hilo sencillo

quí nacen dos flujos de ejecución: el hilo principal y el hilo nuevo. 
El JoinHandle representa el permiso de esperar al hilo con join(). Si lo dejas caer sin join, el hilo queda detached y ya no puedes esperar su terminación mediante ese handle.
*/

use std::thread;
use std::time::Duration;

/// Punto de entrada del programa.
/// Demuestra el uso básico de hilos (threads) en Rust.
///
/// Se crea un hilo secundario que imprime mensajes del 1 al 3,
/// mientras el hilo principal imprime mensajes del 1 al 2.
/// Ambos hilos se ejecutan de forma concurrente, y al final
/// el hilo principal espera a que el hilo secundario termine
/// antes de salir del programa.
fn main() {
    // `thread::spawn` crea un nuevo hilo de ejecución.
    // Recibe un closure (función anónima) con el código que ejecutará el hilo.
    // Devuelve un `JoinHandle` que permite esperar a que el hilo termine.
    let h = thread::spawn(|| {
        // El hilo secundario itera del 1 al 3 (inclusive gracias a `..=`).
        for i in 1..=3 {
            // Imprime un mensaje identificando que viene del hilo secundario.
            println!("hilo: {i}");

            // Pausa el hilo secundario durante 100 milisegundos.
            // Esto simula trabajo y permite observar la concurrencia
            // con el hilo principal, que duerme menos tiempo (80 ms).
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Mientras el hilo secundario se ejecuta en paralelo,
    // el hilo principal también realiza su propio trabajo.
    for i in 1..=2 {
        // Imprime un mensaje identificando que viene del hilo principal.
        println!("main: {i}");

        // Pausa el hilo principal durante 80 milisegundos.
        // Al dormir menos que el hilo secundario (80 ms vs 100 ms),
        // el hilo principal terminará su bucle antes.
        thread::sleep(Duration::from_millis(80));
    }

    // `join()` bloquea el hilo principal hasta que el hilo secundario termine.
    // Sin esta línea, el programa podría finalizar antes de que el hilo
    // secundario complete su ejecución, perdiendo sus últimos mensajes.
    // `unwrap()` desempaqueta el `Result`: si el hilo secundario entró
    // en pánico, propagará ese pánico aquí.
    h.join().unwrap();
}