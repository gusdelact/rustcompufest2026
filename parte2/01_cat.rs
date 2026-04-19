/*
cat — imprimir archivo o stdin
Ideas clave
if: decide archivo vs stdin
for: recorre líneas
match: maneja Result
*/

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Lee líneas de cualquier fuente que implemente `BufRead` y las imprime
/// por la salida estándar. Si ocurre un error al leer alguna línea,
/// se muestra un mensaje de error por la salida de errores estándar.
///
/// # Parámetros
/// - `reader`: cualquier tipo que implemente el trait `BufRead`, lo que
///   permite aceptar tanto un archivo como la entrada estándar (stdin).
fn procesar<T: BufRead>(reader: T) {
    // Itera sobre cada línea del reader. `lines()` devuelve un iterador
    // de `Result<String, io::Error>`, manejando automáticamente los
    // saltos de línea.
    for linea in reader.lines() {
        match linea {
            // Si la línea se leyó correctamente, la imprime en stdout
            Ok(texto) => println!("{}", texto),
            // Si hubo un error de lectura (por ejemplo, encoding inválido),
            // lo reporta en stderr sin detener la ejecución
            Err(e) => eprintln!("Error leyendo línea: {}", e),
        }
    }
}

/// Punto de entrada del programa.
///
/// Comportamiento:
/// - Si se pasa un argumento por línea de comandos, lo interpreta como
///   la ruta de un archivo y lee su contenido línea por línea.
/// - Si no se pasan argumentos, lee desde la entrada estándar (stdin),
///   lo que permite usarlo con pipes, por ejemplo: `echo "hola" | ./programa`
fn main() {
    // Recoge los argumentos de la línea de comandos en un vector.
    // `args[0]` es el nombre del propio ejecutable, `args[1]` sería
    // el primer argumento real proporcionado por el usuario.
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Modo archivo: abre el archivo indicado en el primer argumento.
        // `unwrap()` hará que el programa termine con panic si el archivo
        // no existe o no se puede abrir.
        let archivo = File::open(&args[1]).unwrap();
        // Envuelve el archivo en un `BufReader` para leer de forma
        // eficiente con un buffer interno, evitando lecturas byte a byte.
        let reader = BufReader::new(archivo);
        procesar(reader);
    } else {
        // Modo stdin: lee desde la entrada estándar.
        let stdin = io::stdin();
        // `lock()` obtiene un handle con lock exclusivo sobre stdin,
        // lo que mejora el rendimiento al evitar bloqueos repetidos
        // en cada operación de lectura.
        let reader = stdin.lock();
        procesar(reader);
    }
}