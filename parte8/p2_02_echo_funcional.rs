/*
echo — versión funcional (parte2/02_echo.rs)

Imperativo: for + enumerate + if para separar con espacios.
Funcional: .skip(1).collect::<Vec<_>>().join(" ") une todo en una línea.
*/

use std::env;

fn main() {
    // .skip(1) salta el nombre del ejecutable.
    // .collect::<Vec<String>>() recopila los argumentos.
    // .join(" ") los une con espacios.
    // Una sola línea reemplaza el loop con enumerate + if.
    let salida: String = env::args().skip(1).collect::<Vec<String>>().join(" ");
    println!("{}", salida);
}
