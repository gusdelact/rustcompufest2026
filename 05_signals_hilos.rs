/// - SIGINT: señal de interrupción (Ctrl+C)
/// - SIGTERM: señal de terminación (enviada por `kill` u orquestadores)
use signal_hook::consts::{SIGINT, SIGTERM};

/// `Signals` nos permite suscribirnos a señales del sistema operativo
/// y consumirlas de forma iterativa, como si fueran un stream.
use signal_hook::iterator::Signals;

/// `mpsc` (multiple producer, single consumer) es un canal de comunicación
/// entre hilos. Lo usamos para notificar al hilo principal cuando se
/// recibe una señal.
use std::sync::mpsc;

/// `thread` nos permite crear hilos de ejecución adicionales.
use std::thread;

/// Punto de entrada del programa.
///
/// Este programa demuestra cómo capturar señales del sistema operativo
/// (SIGINT y SIGTERM) de forma segura en Rust, permitiendo realizar
/// un apagado limpio (graceful shutdown) en lugar de terminar abruptamente.
///
/// ## Flujo general
///
/// 1. Se crea un canal `mpsc` para comunicación entre hilos.
/// 2. Se lanza un hilo secundario que escucha señales del SO.
/// 3. El hilo principal se bloquea esperando un mensaje del canal.
/// 4. Cuando el hilo secundario detecta SIGINT o SIGTERM, envía un
///    mensaje a través del canal, desbloqueando al hilo principal.
/// 5. El hilo principal ejecuta la lógica de limpieza y termina.
fn main() -> std::io::Result<()> {
    // Creamos un canal de comunicación:
    // - `tx` (transmitter): el extremo que envía mensajes, se mueve al hilo secundario.
    // - `rx` (receiver): el extremo que recibe mensajes, se queda en el hilo principal.
    let (tx, rx) = mpsc::channel();

    // Lanzamos un hilo dedicado exclusivamente a escuchar señales del SO.
    // Usamos `move` para transferir la propiedad de `tx` al closure del hilo.
    thread::spawn(move || {
        // Registramos las señales que queremos capturar.
        // `Signals::new` configura los handlers del SO para SIGINT y SIGTERM.
        // Si falla el registro (por ejemplo, por permisos), el `unwrap` provocará un panic.
        let mut signals = Signals::new(&[SIGINT, SIGTERM]).unwrap();

        // `signals.forever()` devuelve un iterador bloqueante que produce
        // un valor cada vez que llega una de las señales registradas.
        // Usamos `_` porque no nos importa cuál señal específica llegó,
        // solo nos interesa saber que ocurrió alguna.
        for _ in signals.forever() {
            // Enviamos un mensaje unitario `()` al hilo principal para
            // notificarle que debe iniciar el proceso de apagado.
            // Si el receptor ya fue destruido (el hilo principal terminó),
            // `send` fallará y el `unwrap` provocará un panic en este hilo,
            // lo cual es aceptable porque el programa ya está terminando.
            tx.send(()).unwrap();
        }
    });

    // Informamos al usuario que el programa está activo y esperando.
    println!("Esperando señal...");

    // `rx.recv()` bloquea el hilo principal hasta que se reciba un mensaje
    // del canal. Esto mantiene el programa vivo sin consumir CPU (busy-wait).
    // Cuando el hilo de señales envía `()`, esta línea se desbloquea.
    rx.recv().unwrap();

    // Llegamos aquí solo después de recibir SIGINT o SIGTERM.
    // Este es el lugar ideal para ejecutar lógica de limpieza:
    // cerrar conexiones, guardar estado, liberar recursos, etc.
    println!("Shutdown limpio");

    Ok(())
}