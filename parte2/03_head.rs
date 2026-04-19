/*
head — primeras N líneas
Ideas clave
contador como variable de control clásica
if + break
patrón muy típico de sistemas
*/

use std::io::{self, BufRead};

/// Programa que lee e imprime las primeras N líneas de la entrada estándar (stdin).
/// Funciona de manera similar al comando `head -n 5` en Unix/Linux.
fn main() {
    // Obtenemos un handle a la entrada estándar del proceso
    let stdin = io::stdin();

    // Bloqueamos stdin para obtener un lector con buffer (BufRead),
    // lo cual permite leer línea por línea de forma eficiente
    let reader = stdin.lock();

    // Número máximo de líneas que queremos leer
    let n = 5;

    // Contador para rastrear cuántas líneas hemos procesado
    let mut count = 0;

    // Iteramos sobre cada línea de la entrada estándar.
    // `reader.lines()` devuelve un iterador de `Result<String, io::Error>`,
    // ya que la lectura puede fallar en cualquier momento.
    for linea in reader.lines() {
        // Si ya leímos las N líneas deseadas, salimos del bucle
        if count >= n {
            break;
        }

        // Usamos `match` para manejar el Result de cada línea:
        match linea {
            // Si la lectura fue exitosa, imprimimos el texto de la línea
            Ok(texto) => println!("{}", texto),
            // Si ocurrió un error de E/S, lo mostramos por stderr
            Err(e) => eprintln!("Error: {}", e),
        }

        // Incrementamos el contador después de procesar cada línea
        count += 1;
    }
}