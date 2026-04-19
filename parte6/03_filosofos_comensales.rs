/*
Filósofos Comensales (Dining Philosophers)

🧩 El problema
5 filósofos sentados en una mesa circular.
Entre cada par hay un tenedor (5 tenedores en total).
Para comer, cada filósofo necesita los 2 tenedores adyacentes.

⚠️ Problemas típicos
- Deadlock: todos toman el tenedor izquierdo al mismo tiempo
  y se quedan esperando el derecho para siempre.
- Starvation: un filósofo nunca consigue ambos tenedores.

⚠️ NOTA: Esta implementación puede hacer deadlock.
Si los 5 filósofos toman su tenedor izquierdo simultáneamente,
todos se quedan bloqueados esperando el derecho.
Soluciones clásicas:
- Que un filósofo tome los tenedores en orden inverso.
- Limitar a 4 filósofos intentando comer a la vez (semáforo).
- Usar try_lock() y soltar si no se consigue el segundo.
*/

// Arc: compartir ownership entre hilos.
// Mutex: exclusión mutua — solo un hilo accede al recurso a la vez.
use std::sync::{Arc, Mutex};
use std::thread;

/// Punto de entrada del programa.
///
/// Crea 5 tenedores (cada uno un `Mutex`) y 5 filósofos (cada uno un hilo).
/// Cada filósofo intenta tomar su tenedor izquierdo y luego el derecho.
/// Si lo logra, come e imprime un mensaje.
fn main() {
    // Creamos 5 tenedores, cada uno representado como un `Arc<Mutex<()>>`.
    // El valor dentro del Mutex es `()` (unit) porque no nos importa el dato,
    // solo el mecanismo de lock/unlock. Es como un semáforo binario.
    // `(0..5).map(...)` crea un iterador de 0 a 4 y transforma cada número
    // en un `Arc<Mutex<()>>`. `.collect()` lo recopila en un `Vec`.
    let forks: Vec<_> = (0..5).map(|_| Arc::new(Mutex::new(()))).collect();

    // Creamos 5 hilos, uno por filósofo.
    // `(0..5).map(|i| ...)` itera con `i` = 0, 1, 2, 3, 4.
    let handles: Vec<_> = (0..5).map(|i| {
        // Cada filósofo necesita dos tenedores:
        // - El izquierdo: tenedor[i]
        // - El derecho: tenedor[(i+1) % 5]
        // El `% 5` hace que el filósofo 4 use el tenedor 0 como derecho (mesa circular).
        let left = Arc::clone(&forks[i]);
        let right = Arc::clone(&forks[(i + 1) % 5]);

        // Lanzamos un hilo para este filósofo.
        // `move` transfiere `left`, `right` e `i` al closure.
        thread::spawn(move || {
            // Paso 1: tomar el tenedor izquierdo.
            // `left.lock()` bloquea hasta que el mutex esté disponible.
            // `_l` es un `MutexGuard` — mantiene el lock mientras exista.
            // El prefijo `_` indica que no usamos el valor, solo el lock.
            let _l = left.lock().unwrap();

            // Paso 2: tomar el tenedor derecho.
            // Si otro filósofo ya tiene este tenedor, nos bloqueamos aquí.
            // ⚠️ AQUÍ PUEDE OCURRIR DEADLOCK si todos los filósofos
            // tomaron su tenedor izquierdo simultáneamente.
            let _r = right.lock().unwrap();

            // Si llegamos aquí, tenemos ambos tenedores.
            println!("Filósofo {} comiendo", i);

            // Al salir del scope, `_r` y `_l` se destruyen (RAII),
            // lo que libera ambos mutex automáticamente.
            // No necesitamos `unlock()` explícito como en C.
        })
    }).collect();

    // Esperamos a que todos los filósofos terminen.
    for h in handles {
        h.join().unwrap();
    }
}
