// Importamos los tipos necesarios para manejar operaciones atómicas entre hilos
use std::sync::atomic::{AtomicBool, Ordering};
// Arc permite compartir datos de forma segura entre múltiples hilos
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Punto de entrada del programa.
///
/// Este programa demuestra cómo capturar la señal SIGINT (Ctrl+C) de forma
/// segura usando un booleano atómico compartido entre el hilo principal
/// y el manejador de señales.
///
/// Flujo:
/// 1. Se crea un `AtomicBool` envuelto en `Arc` para compartirlo entre hilos.
/// 2. Se registra un manejador de Ctrl+C que cambia el valor a `false`.
/// 3. El hilo principal espera en un bucle hasta que el valor cambie.
/// 4. Al presionar Ctrl+C, el manejador se ejecuta, el bucle termina
///    y el programa finaliza de forma limpia.
fn main() {
    // Creamos un booleano atómico inicializado en `true`.
    // `Arc` (Atomic Reference Counted) nos permite compartir este valor
    // de forma segura entre el hilo principal y el closure del manejador.
    let running = Arc::new(AtomicBool::new(true));

    // Clonamos el Arc para moverlo dentro del closure del manejador.
    // Esto incrementa el contador de referencias, no duplica el dato.
    let r = running.clone();

    // Registramos un manejador para la señal SIGINT (Ctrl+C).
    // El closure captura `r` por movimiento (move) para poder
    // modificar el AtomicBool desde el contexto del manejador de señales.
    ctrlc::set_handler(move || {
        println!("Recibí SIGINT");
        // Cambiamos el valor a `false` con ordenamiento `SeqCst`
        // (Sequentially Consistent), que garantiza que todos los hilos
        // vean este cambio de forma inmediata y en el orden correcto.
        r.store(false, Ordering::SeqCst);
    }).unwrap();

    // Bucle principal: se ejecuta mientras `running` sea `true`.
    // Cada iteración duerme 100ms para no consumir CPU innecesariamente.
    // Cuando el manejador de Ctrl+C cambia el valor a `false`,
    // la condición del while deja de cumplirse y el programa termina.
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }
}
