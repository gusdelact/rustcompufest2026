/* Control de entrada con if y match, estilo utilidad Unix
Este ejemplo tiene mucho valor porque junta varias piezas:

if decide la fuente de datos.
for recorre las líneas.
match separa éxito y error.
aparece un recurso del sistema real: File.
*/

// Importamos las estructuras y traits necesarios para manejar archivos y entrada/salida
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Función genérica que procesa líneas de cualquier fuente que implemente BufRead
fn procesar_lineas<T: BufRead>(reader: T) {
    // Iteramos sobre cada línea del lector
    for linea in reader.lines() {
        // Verificamos si la lectura de la línea fue exitosa o falló
        match linea {
            // Si la línea se leyó correctamente
            Ok(texto) => {
                // Comprobamos si la línea contiene la palabra "error"
                if texto.contains("error") {
                    // Imprimimos la línea con la etiqueta [ERROR]
                    println!("[ERROR] {}", texto);
                } else {
                    // Si no contiene "error", la imprimimos con la etiqueta [OK]
                    println!("[OK] {}", texto);
                }
            }
            // Si hubo un error al leer la línea, mostramos el mensaje de fallo
            Err(e) => {
                println!("Fallo al leer línea: {}", e);
            }
        }
    }
}

// Función principal del programa
fn main() {
    // Variable que determina si leemos desde la entrada estándar o desde un archivo
    let usar_stdin = true;

    if usar_stdin {
        // Obtenemos la entrada estándar y la bloqueamos para lectura eficiente
        let stdin = io::stdin();
        let reader = stdin.lock(); //stdin.lock() bloquea el acceso a la entrada estándar para el hilo actual, devolviendo un StdinLock.
        //Con stdin.lock() adquieres el lock una sola vez y lo mantienes mientras exista reader. Todas las lecturas posteriores van directo sin costo de sincronización.
        // Procesamos las líneas desde la entrada estándar
        procesar_lineas(reader);
    } else {
        // Abrimos el archivo "log.txt" (falla si no existe)
        let archivo = File::open("log.txt").unwrap();
        // Envolvemos el archivo en un BufReader para lectura con búfer
        let reader = BufReader::new(archivo);
        // Procesamos las líneas desde el archivo
        procesar_lineas(reader);
    }
}