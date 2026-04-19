/*
wc — versión funcional

Compara con parte2/07_wc.rs.
Ambas versiones son similares porque el original ya usaba iteradores.
Aquí llevamos el estilo funcional más lejos: una sola pasada con .fold()
para calcular las tres métricas a la vez.

Imperativo:
    let bytes = input.len();
    let lineas = input.lines().count();
    let palabras = input.split_whitespace().count();

Funcional (este archivo):
    input.lines().fold((0, 0, 0), |acc, linea| { ... });
*/

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Versión 1: directa con iteradores (igual que el original, ya es funcional).
    let bytes = input.len();
    let lineas = input.lines().count();
    let palabras = input.split_whitespace().count();
    println!("{} {} {}", lineas, palabras, bytes);

    // Versión 2: una sola pasada con .fold().
    // `.fold()` recorre el iterador acumulando un resultado.
    // Empezamos con la tupla (0 líneas, 0 palabras, 0 bytes).
    // Por cada línea, sumamos 1 línea, contamos sus palabras,
    // y sumamos sus bytes + 1 (por el '\n' que .lines() elimina).
    let (l, w, b) = input.lines().fold((0, 0, 0), |(l, w, b), linea| {
        (
            l + 1,                                    // una línea más
            w + linea.split_whitespace().count(),     // palabras en esta línea
            b + linea.len() + 1,                      // bytes + salto de línea
        )
    });
    println!("fold: {} {} {}", l, w, b);
}
