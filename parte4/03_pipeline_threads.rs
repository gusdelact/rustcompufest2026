/* 
Equivalente “tipo pipeline”, pero dentro del mismo programa

Aquí la idea es muy parecida a un pipeline:

una etapa produce,
otra transforma,
otra consume.

Pero ya no son procesos del sistema operativo, sino hilos conectados por canales. Eso es justamente el estilo de paso de mensajes que Rust documenta con mpsc

*/

// Importamos el módulo `mpsc` (Multiple Producer, Single Consumer) para crear canales de comunicación entre hilos.
// Un canal permite enviar datos de un hilo a otro de forma segura.
use std::sync::mpsc;
// Importamos el módulo `thread` para poder crear y gestionar hilos de ejecución.
use std::thread;

fn main() {
    // Creamos el primer canal de comunicación.
    // `tx1` es el transmisor (sender) y `rx1` es el receptor (receiver).
    // Este canal conectará el hilo productor con el hilo procesador.
    let (tx1, rx1) = mpsc::channel();

    // Creamos el segundo canal de comunicación.
    // `tx2` es el transmisor y `rx2` es el receptor.
    // Este canal conectará el hilo procesador con el hilo principal.
    let (tx2, rx2) = mpsc::channel();

    // --- Hilo 1: Productor ---
    // Creamos un nuevo hilo que actúa como productor de datos.
    // `move` transfiere la propiedad de `tx1` al hilo, ya que lo necesita para enviar mensajes.
    thread::spawn(move || {
        // Iteramos sobre un arreglo de cadenas en español: "uno", "dos", "tres".
        for s in ["uno", "dos", "tres"] {
            // Enviamos cada cadena (convertida a String) a través del canal `tx1`.
            // `unwrap()` hará que el programa entre en pánico si el receptor ya fue cerrado.
            tx1.send(s.to_string()).unwrap();
        }
        // Al terminar el bucle, `tx1` se destruye (sale del scope),
        // lo que cierra el extremo transmisor del canal y señala al receptor que no habrá más mensajes.
    });

    // --- Hilo 2: Procesador / Transformador ---
    // Creamos un segundo hilo que recibe datos del primer canal, los transforma y los reenvía.
    // `move` transfiere la propiedad de `rx1` (para recibir) y `tx2` (para enviar).
    thread::spawn(move || {
        // Iteramos sobre los mensajes recibidos de `rx1`.
        // Este bucle se bloquea esperando mensajes y termina automáticamente
        // cuando el transmisor `tx1` se cierra (es decir, cuando el hilo productor termina).
        for msg in rx1 {
            // Transformamos el mensaje convirtiéndolo a mayúsculas.
            // "uno" -> "UNO", "dos" -> "DOS", "tres" -> "TRES"
            let filtrado = msg.to_uppercase();
            // Enviamos el mensaje transformado a través del segundo canal `tx2`.
            tx2.send(filtrado).unwrap();
        }
        // Al terminar, `tx2` se destruye, cerrando el segundo canal.
    });

    // --- Hilo principal: Consumidor ---
    // El hilo principal actúa como consumidor final de la cadena de procesamiento.
    // Iteramos sobre los mensajes recibidos de `rx2`.
    // Este bucle se bloquea esperando mensajes y termina cuando `tx2` se cierra.
    for msg in rx2 {
        // Imprimimos cada mensaje ya transformado (en mayúsculas).
        // La salida será:
        //   UNO
        //   DOS
        //   TRES
        println!("{msg}");
    }

    // Resumen del flujo (pipeline de 3 etapas):
    // Hilo 1 (Productor) --tx1/rx1--> Hilo 2 (Procesador) --tx2/rx2--> Hilo principal (Consumidor)
    //   "uno", "dos", "tres"  →  to_uppercase()  →  "UNO", "DOS", "TRES" → println!
}