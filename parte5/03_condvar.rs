/*
Condvar
Ejemplo productor/consumidor sencillo:

esperar una condición
dormir proceso/thread
despertar cuando alguien señaliza
*/

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

/// Programa que demuestra la sincronización entre hilos usando Mutex y Condvar.
///
/// Patrón productor-consumidor:
/// - El productor espera 500ms y luego señaliza que un evento está listo.
/// - El consumidor espera bloqueado hasta recibir la señal del productor.
///
/// Flujo:
/// 1. Se crea un estado compartido (bool protegido por Mutex + Condvar).
/// 2. El productor duerme 500ms, adquiere el lock, cambia el estado a `true` y notifica.
/// 3. El consumidor adquiere el lock, y si el estado aún es `false`, se bloquea
///    en `cvar.wait()` hasta ser notificado.
/// 4. Al despertar y verificar que `listo == true`, imprime el mensaje.
fn main() {
    // Estado compartido entre hilos:
    // - Mutex<bool>: indica si el evento fue producido (`true`) o no (`false`).
    // - Condvar: variable de condición para notificar al consumidor.
    // Arc permite compartir ownership entre múltiples hilos de forma segura.
    let compartido = Arc::new((Mutex::new(false), Condvar::new()));

    // Hilo productor: simula trabajo (500ms) y luego señaliza el evento.
    let productor = {
        let compartido = Arc::clone(&compartido);
        thread::spawn(move || {
            // Simula una tarea que tarda 500ms en completarse.
            thread::sleep(Duration::from_millis(500));

            // Desestructura la tupla para obtener el Mutex y la Condvar.
            let (lock, cvar) = &*compartido;

            // Adquiere el lock del Mutex. `listo` es un MutexGuard<bool>.
            let mut listo = lock.lock().unwrap();

            // Cambia el estado a `true`, indicando que el evento fue producido.
            *listo = true;

            // Notifica a un hilo en espera (el consumidor) de que el estado cambió.
            cvar.notify_one();
        })
    };

    // Hilo consumidor: espera hasta que el productor señalice el evento.
    let consumidor = {
        let compartido = Arc::clone(&compartido);
        thread::spawn(move || {
            let (lock, cvar) = &*compartido;

            // Adquiere el lock del Mutex.
            let mut listo = lock.lock().unwrap();

            // Bucle de espera: protege contra "spurious wakeups" (despertares espurios).
            // Si `listo` es `false`, se bloquea en `cvar.wait()`, que:
            //   1. Libera el lock del Mutex automáticamente.
            //   2. Duerme el hilo hasta recibir una notificación.
            //   3. Re-adquiere el lock al despertar y devuelve el nuevo MutexGuard.
            while !*listo {
                listo = cvar.wait(listo).unwrap();
            }

            // Solo llega aquí cuando `listo == true`, es decir, el productor señalizó.
            println!("Evento recibido");
        })
    };

    // Espera a que ambos hilos terminen antes de salir del programa.
    productor.join().unwrap();
    consumidor.join().unwrap();
}