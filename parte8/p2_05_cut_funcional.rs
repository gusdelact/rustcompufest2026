/*
cut — versión funcional (parte2/05_cut.rs)

Imperativo: for + match + split + if para verificar índice.
Funcional: .filter_map() combina transformación y filtrado en un paso.
*/

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let columna = 1;

    stdin
        .lock()
        .lines()
        .flatten()
        // .filter_map() aplica una función que retorna Option.
        // Si retorna Some(valor), lo incluye. Si retorna None, lo descarta.
        // Aquí: split por espacios, tomar la columna, retornar Some si existe.
        .filter_map(|linea| {
            linea
                .split_whitespace()
                .nth(columna) // .nth(n) toma el elemento n directamente
                .map(|s| s.to_string()) // convertir &str a String (porque linea se mueve)
        })
        .for_each(|col| println!("{}", col));
}
