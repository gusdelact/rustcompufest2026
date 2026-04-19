/*
Pipeline con hilos — versión funcional

Compara con parte4/03_pipeline_threads.rs.
El patrón es el mismo (productor → transformador → consumidor),
pero dentro de cada hilo usamos estilo funcional.

Imperativo (parte4):
    thread::spawn(move || {
        for s in ["uno", "dos", "tres"] {
            tx1.send(s.to_string()).unwrap();
        }
    });

Funcional (este archivo):
    thread::spawn(move || {
        ["uno", "dos", "tres"]
            .iter()
            .map(|s| s.to_string())
            .for_each(|s| tx1.send(s).unwrap());
    });

También mostramos cómo el transformador puede usar .map() y .filter()
para crear un pipeline más expresivo.
*/

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    // Etapa 1: Productor
    // En lugar de un `for` loop, usamos una cadena de iteradores.
    // `.iter()` itera sobre el array.
    // `.map()` convierte cada &str en String.
    // `.for_each()` envía cada String por el canal.
    thread::spawn(move || {
        ["uno", "dos", "tres", "cuatro", "cinco"]
            .iter()
            .map(|s| s.to_string())
            .for_each(|s| tx1.send(s).unwrap());
    });

    // Etapa 2: Transformador con filtro
    // Recibe del canal, transforma a mayúsculas, filtra los que tienen
    // más de 3 caracteres, y reenvía.
    // `rx1.iter()` convierte el receiver en un iterador (equivale a `for msg in rx1`).
    // `.map()` transforma cada mensaje.
    // `.filter()` descarta los que no cumplen el predicado.
    // `.for_each()` envía los que pasaron el filtro.
    thread::spawn(move || {
        rx1.iter()
            .map(|msg| msg.to_uppercase())
            .filter(|msg| msg.len() > 3)
            .for_each(|msg| tx2.send(msg).unwrap());
    });

    // Etapa 3: Consumidor
    // `.iter()` sobre el receiver, `.enumerate()` para numerar,
    // `.for_each()` para imprimir.
    rx2.iter()
        .enumerate()
        .for_each(|(i, msg)| println!("{}: {}", i + 1, msg));

    // Salida esperada (solo los de más de 3 caracteres, en mayúsculas):
    //   1: TRES
    //   2: CUATRO
    //   3: CINCO
}
