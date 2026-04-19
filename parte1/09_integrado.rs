/*
Un mini ejemplo integrador para clase
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Representa los niveles de severidad posibles para una entrada de log.
enum NivelLog {
    /// Nivel informativo: mensajes generales del sistema.
    Info,
    /// Nivel de advertencia: situaciones que requieren atención pero no son críticas.
    Warn,
    /// Nivel de error: fallos o problemas críticos en el sistema.
    Error,
}

/// Determina el nivel de log a partir del contenido de una línea.
///
/// Analiza la línea buscando las palabras clave `"ERROR"` y `"WARN"`.
/// Si no se encuentra ninguna, se asume que es de nivel `Info`.
///
/// # Argumentos
///
/// * `linea` - Referencia a la cadena de texto que representa una línea del archivo de log.
///
/// # Retorna
///
/// Un valor de `NivelLog` correspondiente al nivel detectado en la línea.
fn nivel_desde_linea(linea: &str) -> NivelLog {
    if linea.contains("ERROR") {
        NivelLog::Error
    } else if linea.contains("WARN") {
        NivelLog::Warn
    } else {
        NivelLog::Info
    }
}

/// Punto de entrada del programa.
///
/// Lee el archivo `sistema.log` línea por línea, clasifica cada entrada
/// según su nivel de severidad (`INFO`, `WARN` o `ERROR`), imprime cada
/// línea con su etiqueta correspondiente y al finalizar muestra un resumen
/// con el conteo total de cada nivel.
fn main() {
    // Abre el archivo de log. Falla si el archivo no existe o no se puede leer.
    let archivo = File::open("sistema.log").unwrap();
    let reader = BufReader::new(archivo);

    // Contadores para cada nivel de log.
    let mut errores = 0;
    let mut warnings = 0;
    let mut infos = 0;

    // Itera sobre cada línea del archivo de log.
    for linea in reader.lines() {
        let linea = linea.unwrap();

        // Clasifica la línea según su nivel y actualiza el contador correspondiente.
        match nivel_desde_linea(&linea) {
            NivelLog::Info => {
                infos += 1;
                println!("[INFO ] {}", linea);
            }
            NivelLog::Warn => {
                warnings += 1;
                println!("[WARN ] {}", linea);
            }
            NivelLog::Error => {
                errores += 1;
                println!("[ERROR] {}", linea);
            }
        }
    }

    // Imprime el resumen final con la cantidad de entradas por nivel.
    println!("Resumen: info={}, warn={}, error={}", infos, warnings, errores);
}