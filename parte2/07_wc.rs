/*
wc — contar líneas, palabras, bytes

Ideas clave
procesamiento completo en memoria (simple)
uso de iteradores
muy claro para enseñar métricas

*/

/// Programa que replica el comportamiento básico del comando `wc` (word count) de Unix.
/// Lee toda la entrada estándar (stdin) y muestra:
///   - Número de líneas
///   - Número de palabras
///   - Número de bytes
///
/// Uso típico:
///   echo "hola mundo" | 07_wc
///   cat archivo.txt | 07_wc

// Importamos los traits necesarios del módulo de entrada/salida estándar:
// - `io` nos da acceso a stdin (entrada estándar)
// - `Read` es un trait que proporciona el método `read_to_string`,
//   necesario para leer todo el contenido de stdin de una sola vez.
use std::io::{self, Read};

fn main() {
    // Creamos un String vacío que servirá como buffer para almacenar
    // todo el contenido leído desde la entrada estándar.
    let mut input = String::new();

    // Leemos TODO el contenido de stdin hasta encontrar EOF (End Of File).
    // `read_to_string` escribe los datos en `input` y devuelve un Result.
    // `.unwrap()` extrae el valor Ok o hace panic si hay un error de lectura.
    io::stdin().read_to_string(&mut input).unwrap();

    // Contamos los bytes del string.
    // `.len()` devuelve la longitud en bytes (no en caracteres Unicode).
    // Esto coincide con el comportamiento de `wc -c`.
    let bytes = input.len();

    // Contamos las líneas usando `.lines()`, que devuelve un iterador
    // sobre cada línea del texto (separadas por '\n' o '\r\n').
    // `.count()` consume el iterador y devuelve el total de elementos.
    let lineas = input.lines().count();

    // Contamos las palabras usando `.split_whitespace()`, que divide el texto
    // por cualquier tipo de espacio en blanco (espacios, tabs, saltos de línea, etc.)
    // y omite secuencias vacías. Esto replica el conteo de palabras de `wc -w`.
    let palabras = input.split_whitespace().count();

    // Imprimimos el resultado en el mismo formato que `wc`:
    //   líneas  palabras  bytes
    println!("{} {} {}", lineas, palabras, bytes);
}