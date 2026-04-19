/*
Versión sin unwrap: maneja errores explícitamente con match y ?

Este módulo lee un archivo de log llamado "sistema.log", clasifica cada línea
según su nivel de severidad (INFO, WARN o ERROR) y muestra un resumen con
el conteo de cada tipo al finalizar. Todos los errores de E/S se propagan
mediante el operador `?` en lugar de usar `unwrap`.
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Representa los niveles de severidad posibles para una entrada de log.
enum NivelLog {
    /// Mensaje informativo general.
    Info,
    /// Advertencia que no impide la ejecución pero requiere atención.
    Warn,
    /// Error crítico que indica un fallo en el sistema.
    Error,
}

/// Determina el nivel de log de una línea según su contenido.
///
/// Busca las palabras clave "ERROR" y "WARN" dentro de la línea.
/// Si no se encuentra ninguna, se asume que es de nivel `Info`.
///
/// # Argumentos
///
/// * `linea` - Referencia a la cadena de texto de la línea del log.
///
/// # Retorna
///
/// El `NivelLog` correspondiente a la línea analizada.
fn nivel_desde_linea(linea: &str) -> NivelLog {
    if linea.contains("ERROR") {
        NivelLog::Error
    } else if linea.contains("WARN") {
        NivelLog::Warn
    } else {
        NivelLog::Info
    }
}

/// Lee el archivo "sistema.log", clasifica cada línea por nivel de severidad,
/// la imprime con un prefijo indicativo y al final muestra un resumen con
/// la cantidad de líneas de cada tipo.
///
/// # Errores
///
/// Retorna `std::io::Error` si no se puede abrir el archivo o si ocurre
/// un error al leer alguna de sus líneas.
fn procesar_log() -> Result<(), std::io::Error> {
    // Abre el archivo de log; propaga el error si no existe o no se puede leer.
    let archivo = File::open("sistema.log")?;

    // Envuelve el archivo en un BufReader para leer línea por línea de forma eficiente.
    let reader = BufReader::new(archivo);

    // Contadores para cada nivel de severidad.
    let mut errores = 0;
    let mut warnings = 0;
    let mut infos = 0;

    // Itera sobre cada línea del archivo.
    for linea in reader.lines() {
        // Desenvuelve la línea; propaga el error de E/S si la lectura falla.
        let linea = linea?;

        // Clasifica la línea y actualiza el contador correspondiente.
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

    // Imprime el resumen final con los totales de cada nivel.
    println!("Resumen: info={}, warn={}, error={}", infos, warnings, errores);
    Ok(())
}

/// Punto de entrada del programa.
///
/// Invoca `procesar_log` y, en caso de error, imprime un mensaje descriptivo
/// en la salida de error estándar (stderr).
fn main() {
    match procesar_log() {
        Ok(()) => {}
        Err(e) => eprintln!("Error al procesar el log: {}", e),
    }
}
