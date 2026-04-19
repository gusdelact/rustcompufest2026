/*
Productor-Consumidor

🧩 El problema
Un productor genera datos.
Un consumidor los procesa.
Hay un buffer en medio.

⚠️ Problemas típicos en C
- El productor escribe cuando el buffer está lleno.
- El consumidor lee cuando está vacío.
- Condiciones de carrera si ambos acceden al buffer sin sincronización.

En C, esto requiere un buffer compartido protegido con mutex y dos semáforos
(uno para "hay espacio" y otro para "hay datos"). En Rust, mpsc::channel()
resuelve todo: actúa como buffer, bloquea al consumidor cuando está vacío,
y el dato se mueve (no se comparte) eliminando condiciones de carrera.
*/

// Importamos mpsc (Multiple Producer, Single Consumer): un canal para pasar
// mensajes entre hilos. `tx` es el transmisor (sender), `rx` es el receptor (receiver).
use std::sync::mpsc;
// Importamos thread para crear hilos de ejecución.
use std::thread;

/// Punto de entrada del programa.
///
/// Crea un hilo productor que genera 5 valores (0..5) y los envía
/// por un canal mpsc. El hilo principal actúa como consumidor,
/// recibiendo e imprimiendo cada valor.
fn main() {
    // Creamos un canal de comunicación.
    // `tx` (transmitter) envía datos, `rx` (receiver) los recibe.
    // El canal es tipado: Rust infiere que transmitirá valores `i32`
    // a partir del primer `send()`.
    let (tx, rx) = mpsc::channel();

    // Lanzamos el hilo productor.
    // `move` transfiere la propiedad de `tx` al closure del hilo.
    // El hilo principal se queda con `rx`.
    thread::spawn(move || {
        // El productor genera 5 valores (0, 1, 2, 3, 4).
        for i in 0..5 {
            // `send(i)` mueve el valor `i` al canal.
            // Después de enviarlo, el productor ya no puede usarlo.
            // `unwrap()` hace panic si el receptor fue destruido
            // (lo cual no pasa aquí porque el consumidor sigue vivo).
            tx.send(i).unwrap();
        }
        // Al terminar el loop, `tx` sale de scope y se destruye.
        // Esto cierra el extremo transmisor del canal, lo que le indica
        // al receptor que no llegarán más mensajes.
    });

    // El hilo principal actúa como consumidor.
    // `for recibido in rx` itera sobre los mensajes del canal.
    // Se bloquea esperando cuando no hay mensajes disponibles.
    // Termina automáticamente cuando `tx` se destruye (el canal se cierra).
    for recibido in rx {
        // Imprimimos cada valor recibido.
        // La salida será:
        //   Consumido: 0
        //   Consumido: 1
        //   Consumido: 2
        //   Consumido: 3
        //   Consumido: 4
        println!("Consumido: {}", recibido);
    }
}
