/*
tail — versión funcional (parte2/04_tail.rs)

Imperativo: for + match + push + remove(0) como buffer FIFO.
Funcional: recolectar todas las líneas y usar un slice del final.

La versión funcional es más simple pero usa más memoria
(guarda todas las líneas, no solo las últimas N).
Para archivos enormes, la versión imperativa con VecDeque es mejor.
*/

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let n = 5;

    // .lines() → Iterator<Result<String>>
    // .flatten() → descarta errores, desenvuelve Ok
    // .collect() → recopila TODAS las líneas en un Vec
    let lineas: Vec<String> = stdin.lock().lines().flatten().collect();

    // .len().saturating_sub(n) calcula el índice de inicio.
    // saturating_sub evita underflow: si hay menos de n líneas, empieza en 0.
    // &lineas[inicio..] es un slice con las últimas n líneas.
    // .iter().for_each() las imprime.
    let inicio = lineas.len().saturating_sub(n);
    lineas[inicio..].iter().for_each(|l| println!("{}", l));
}
