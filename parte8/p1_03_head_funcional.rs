/*
head — versión funcional

Compara con parte2/03_head.rs que usa un contador manual + if + break.

Imperativo (parte2):
    let mut count = 0;
    for linea in reader.lines() {
        if count >= n { break; }
        match linea {
            Ok(texto) => println!("{}", texto),
            Err(e) => eprintln!("Error: {}", e),
        }
        count += 1;
    }

Funcional (este archivo):
    reader.lines().take(n).for_each(|linea| { ... });

`.take(n)` reemplaza el contador manual y el `if + break`.
Es más declarativo: "toma las primeras n líneas" en lugar de
"lleva un contador, verifica si llegaste a n, y rompe el loop".
*/

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let n = 5;

    // `.lines()` devuelve un iterador de Result<String, io::Error>.
    // `.take(n)` limita el iterador a los primeros `n` elementos.
    //   No hay contador manual, no hay `break`.
    // `.for_each()` ejecuta el closure por cada elemento.
    reader.lines().take(n).for_each(|linea| {
        match linea {
            Ok(texto) => println!("{}", texto),
            Err(e) => eprintln!("Error: {}", e),
        }
    });

    // Alternativa aún más concisa usando .flatten():
    // `.flatten()` descarta los Err y desenvuelve los Ok automáticamente.
    // Útil cuando no te importa manejar errores de lectura individuales.
    //
    // stdin.lock().lines().flatten().take(n).for_each(|l| println!("{}", l));
}
