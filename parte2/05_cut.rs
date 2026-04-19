/*
cut — seleccionar columnas simples

Versión: separar por espacios y tomar columna 1

Ideas clave
strings como slices (&str)
uso de Vec<&str>
if para evitar out-of-bounds

*/

/// Programa que lee líneas desde la entrada estándar (stdin) y extrae
/// la primera columna (índice 0) de cada línea, separando por espacios en blanco.
///
/// Ejemplo de uso:
///   echo "hola mundo" | 05_cut
///   # Salida: hola
///
/// Cada línea se divide en "partes" usando espacios/tabs como delimitadores,
/// y se imprime únicamente el elemento en la posición indicada por `columna`.

use std::io::{self, BufRead};

fn main() {
    // Obtenemos un handle al flujo de entrada estándar (stdin).
    let stdin = io::stdin();

    // Bloqueamos stdin para obtener un lector con buffer (BufRead),
    // lo cual permite leer línea por línea de forma eficiente.
    let reader = stdin.lock();

    // Índice de la columna que queremos extraer de cada línea.
    // 0 = primera columna, 1 = segunda, etc.
    let columna = 1;

    // Iteramos sobre cada línea del flujo de entrada.
    // `reader.lines()` devuelve un iterador de `Result<String, io::Error>`.
    for linea in reader.lines() {
        match linea {
            // Si la línea se leyó correctamente, procesamos el texto.
            Ok(texto) => {
                // Dividimos la línea en partes usando espacios en blanco como
                // separadores. `split_whitespace()` ignora espacios múltiples
                // y tabulaciones. Recopilamos los fragmentos en un Vec<&str>.
                let partes: Vec<&str> = texto.split_whitespace().collect();

                // Verificamos que la columna solicitada exista dentro de las
                // partes obtenidas, para evitar un acceso fuera de rango (panic).
                if columna < partes.len() {
                    // Imprimimos el valor de la columna correspondiente.
                    println!("{}", partes[columna]);
                }
                // Si la línea tiene menos columnas de las requeridas,
                // simplemente se omite sin producir error.
            }
            // Si ocurrió un error al leer la línea (por ejemplo, UTF-8 inválido),
            // lo reportamos por stderr sin detener el programa.
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}