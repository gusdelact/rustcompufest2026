/*
od — versión funcional (parte2/06_od.rs)

Imperativo: loop + match + for sobre bytes.
Funcional: leer todos los bytes, .chunks(16) para agrupar, .for_each() para imprimir.

Nota: la versión imperativa con buffer fijo es mejor para archivos grandes
porque no carga todo en memoria. Esta versión funcional es más clara
pero asume que la entrada cabe en memoria.
*/

use std::io::{self, Read};

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    // .chunks(16) divide el Vec<u8> en slices de hasta 16 bytes.
    // Cada chunk es un &[u8].
    // .for_each() procesa cada grupo de 16 bytes.
    input.chunks(16).for_each(|chunk| {
        // Dentro de cada chunk, .iter() recorre los bytes.
        // .for_each() imprime cada byte en hexadecimal.
        chunk.iter().for_each(|b| print!("{:02x} ", b));
        println!();
    });
}
