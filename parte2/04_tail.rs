/*
tail — últimas N líneas (simple con buffer)

Ideas clave
Vec<String> como buffer circular sencillo
lógica de ventana deslizante
muy cercano a cómo se haría en C, pero más seguro

*/


/// Programa que implementa el comando `tail` básico de Unix.
/// Lee todas las líneas de la entrada estándar (stdin) y muestra
/// únicamente las últimas N líneas, similar a `tail -n 5`.
///
/// Utiliza un buffer de tamaño fijo que actúa como una cola FIFO
/// (First In, First Out): cuando se supera la capacidad máxima,
/// se elimina la línea más antigua para hacer espacio a la nueva.

use std::io::{self, BufRead};

fn main() {
    // Obtenemos un handle al flujo de entrada estándar (stdin).
    let stdin = io::stdin();

    // Bloqueamos stdin para obtener un lector con buffer (BufReader),
    // lo cual es más eficiente que leer byte a byte.
    let reader = stdin.lock();

    // Número máximo de líneas que queremos conservar (las últimas N).
    let n = 5;

    // Buffer donde almacenamos temporalmente las líneas leídas.
    // Al final del programa, contendrá como máximo `n` líneas:
    // las últimas que se leyeron de stdin.
    let mut buffer: Vec<String> = Vec::new();

    // Iteramos sobre cada línea de la entrada estándar.
    // `reader.lines()` devuelve un iterador de `Result<String, io::Error>`,
    // ya que la lectura puede fallar en cualquier momento.
    for linea in reader.lines() {
        match linea {
            Ok(texto) => {
                // Agregamos la línea leída al final del buffer.
                buffer.push(texto);

                // Si el buffer excede el tamaño máximo permitido,
                // eliminamos el primer elemento (el más antiguo).
                // Esto garantiza que siempre tengamos como máximo
                // las últimas `n` líneas en memoria.
                // Nota: `remove(0)` tiene costo O(n) porque desplaza
                // todos los elementos. Para mayor eficiencia se podría
                // usar `VecDeque` que permite eliminar del frente en O(1).
                if buffer.len() > n {
                    buffer.remove(0); // comportamiento FIFO
                }
            }
            // Si ocurre un error al leer una línea, lo reportamos
            // por stderr sin interrumpir la lectura de las demás líneas.
            Err(e) => eprintln!("Error al leer línea: {}", e),
        }
    }

    // Una vez consumida toda la entrada, imprimimos las últimas N líneas
    // que quedaron almacenadas en el buffer.
    for linea in buffer {
        println!("{}", linea);
    }
}