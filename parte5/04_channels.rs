/*
Channels

Esto es la contraparte moderna de “message passing”.

Puedes decir:

con Mutex, sincronizas acceso a estado compartido
con channel, sincronizas por flujo de mensajes

*/

// Importamos el módulo mpsc (multiple producer, single consumer) para comunicación entre hilos
// mpsc nos da un canal con un transmisor (tx) y un receptor (rx)
use std::sync::mpsc;
// Importamos el módulo thread para crear hilos de ejecución
use std::thread;

fn main() {
    // Creamos un canal de comunicación.
    // tx (transmisor) envía datos, rx (receptor) los recibe.
    // El canal es tipado: en este caso transmitirá valores de tipo String.
    let (tx, rx) = mpsc::channel();

    // Creamos un nuevo hilo (thread) con thread::spawn.
    // Usamos `move` para transferir la propiedad de `tx` al closure del hilo hijo,
    // ya que el hilo necesita ser dueño del transmisor para poder enviar datos.
    thread::spawn(move || {
        // Enviamos un mensaje de tipo String a través del canal.
        // send() devuelve un Result; usamos unwrap() para que falle con panic
        // si el receptor ya fue descartado (dropped).
        tx.send(String::from("hola desde thread hijo")).unwrap();
    });

    // recv() bloquea el hilo principal hasta que reciba un mensaje del canal.
    // Devuelve un Result: Ok(valor) si se recibió correctamente,
    // o Err si el transmisor se cerró sin enviar nada.
    let msg = rx.recv().unwrap();

    // Imprimimos el mensaje recibido desde el hilo hijo
    println!("recibido: {}", msg);
}