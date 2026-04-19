/*
od — volcado en hexadecimal

Ideas clave
buffer fijo [u8; N]
lectura binaria
match sobre read
muy fiel a programación de sistemas tradicional

*/

/// Programa que lee datos binarios desde la entrada estándar (stdin)
/// y los muestra en formato hexadecimal, 16 bytes por línea.
///
/// Esto es útil para inspeccionar el contenido crudo de archivos o flujos
/// de datos, similar a herramientas como `xxd` o `hexdump`.
use std::io::{self, Read};

fn main() {
    // Buffer de 16 bytes: cada iteración leerá hasta 16 bytes de stdin.
    // Se usa un arreglo de tamaño fijo para evitar asignaciones en el heap.
    let mut buffer = [0u8; 16];

    // Obtenemos un handle al flujo de entrada estándar (stdin).
    // Esto permite leer datos que se envían al programa mediante pipes
    // o redirección, por ejemplo: `cat archivo.bin | cargo run`
    let mut stdin = io::stdin();

    // Bucle principal: lee continuamente bloques de hasta 16 bytes
    // desde stdin hasta que se alcance el fin del archivo (EOF)
    // o se produzca un error de lectura.
    loop {
        // `read` intenta llenar el buffer y devuelve cuántos bytes
        // se leyeron realmente. Puede devolver menos de 16 si no hay
        // suficientes datos disponibles.
        match stdin.read(&mut buffer) {
            // Ok(0) indica que se alcanzó el EOF (fin del archivo),
            // es decir, no quedan más datos por leer. Salimos del bucle.
            Ok(0) => break,

            // Ok(n) indica que se leyeron `n` bytes exitosamente.
            // Iteramos sobre los primeros `n` bytes del buffer
            // y los imprimimos en formato hexadecimal de 2 dígitos.
            Ok(n) => {
                for i in 0..n {
                    // {:02x} formatea el byte como hexadecimal en minúsculas,
                    // rellenando con ceros a la izquierda hasta 2 caracteres.
                    // Ejemplo: el byte 10 se muestra como "0a", 255 como "ff".
                    print!("{:02x} ", buffer[i]);
                }
                // Salto de línea al final de cada bloque de hasta 16 bytes,
                // para que la salida sea legible y organizada en filas.
                println!();
            }

            // Si ocurre un error de lectura (por ejemplo, un problema con
            // el descriptor de archivo), lo mostramos por stderr y salimos.
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}