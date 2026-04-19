/*
cat — versión funcional (parte2/01_cat.rs)

Imperativo: for + match sobre cada línea.
Funcional: .flatten() descarta errores, .for_each() imprime.
*/

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn procesar<T: BufRead>(reader: T) {
    // .lines() → Iterator<Result<String>>
    // .flatten() → desenvuelve Ok, descarta Err silenciosamente
    // .for_each() → imprime cada línea
    reader.lines().flatten().for_each(|linea| println!("{}", linea));
}

fn main() {
    // .nth(1) toma el segundo argumento (índice 1) directamente,
    // sin recolectar todo en un Vec. Más eficiente que .collect().
    match env::args().nth(1) {
        Some(ruta) => {
            let archivo = File::open(&ruta).unwrap();
            procesar(BufReader::new(archivo));
        }
        None => {
            let stdin = io::stdin();
            procesar(stdin.lock());
        }
    }
}
