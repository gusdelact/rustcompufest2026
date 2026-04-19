/*
Analizador de logs — versión funcional

Compara con parte1/09_integrado.rs que usa variables mutables + for + match.

Imperativo (parte1):
    let mut errores = 0;
    let mut warnings = 0;
    let mut infos = 0;
    for linea in reader.lines() {
        let linea = linea.unwrap();
        match nivel_desde_linea(&linea) {
            NivelLog::Info => { infos += 1; println!("[INFO ] {}", linea); }
            ...
        }
    }

Funcional (este archivo):
    reader.lines()
        .flatten()
        .map(|l| (clasificar(&l), l))
        .inspect(|(nivel, linea)| println!("[{}] {}", nivel, linea))
        .fold((0, 0, 0), |acc, (nivel, _)| { ... });

Usamos:
- .flatten() para descartar errores de lectura
- .map() para clasificar cada línea
- .inspect() para imprimir sin consumir (efecto secundario en la cadena)
- .fold() para acumular los conteos en una sola pasada
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Clasifica una línea y devuelve su etiqueta como &str.
/// Usamos &str en lugar de enum para simplificar el estilo funcional.
fn clasificar(linea: &str) -> &'static str {
    if linea.contains("ERROR") {
        "ERROR"
    } else if linea.contains("WARN") {
        "WARN"
    } else {
        "INFO"
    }
}

fn main() {
    // Abrimos el archivo. Usamos .expect() para un mensaje de error claro.
    let archivo = File::open("sistema.log").expect("No se pudo abrir sistema.log");
    let reader = BufReader::new(archivo);

    // Cadena funcional completa:
    let (infos, warns, errores) = reader
        .lines()
        // .flatten() convierte Iterator<Result<String>> en Iterator<String>,
        // descartando silenciosamente las líneas con error de lectura.
        .flatten()
        // .map() transforma cada línea en una tupla (nivel, línea).
        // Clasificamos la línea y la mantenemos para imprimirla después.
        .map(|linea| {
            let nivel = clasificar(&linea);
            (nivel, linea)
        })
        // .inspect() permite ejecutar un efecto secundario (imprimir)
        // sin consumir el elemento. La tupla sigue fluyendo al siguiente paso.
        .inspect(|(nivel, linea)| {
            println!("[{:<5}] {}", nivel, linea);
        })
        // .fold() acumula los conteos en una tupla (infos, warns, errores).
        // Empezamos en (0, 0, 0) y por cada elemento sumamos al contador correcto.
        .fold((0, 0, 0), |(i, w, e), (nivel, _)| match nivel {
            "INFO" => (i + 1, w, e),
            "WARN" => (i, w + 1, e),
            "ERROR" => (i, w, e + 1),
            _ => (i, w, e),
        });

    println!("Resumen: info={}, warn={}, error={}", infos, warns, errores);
}
