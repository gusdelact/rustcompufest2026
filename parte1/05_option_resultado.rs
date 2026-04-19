/* Resultado de operación tipo sistema: match con Option
Aquí match reemplaza elegantemente una lógica muy común en C:

antes: regresar -1, o NULL, o un código especial.
ahora: regresar Some(pos) o None.
*/

/// Busca un byte específico dentro de un buffer de bytes.
///
/// # Argumentos
///
/// * `buffer` - Un slice de bytes donde se realizará la búsqueda.
/// * `buscado` - El byte que se desea encontrar.
///
/// # Retorno
///
/// Retorna `Some(usize)` con la posición del primer byte encontrado,
/// o `None` si el byte no se encuentra en el buffer.
fn buscar_byte(buffer: &[u8], buscado: u8) -> Option<usize> {
    // Recorremos el buffer con su índice
    /* Son dos métodos encadenados:
buffer.iter() → crea un iterador que recorre cada elemento del slice por referencia (&u8).
.enumerate() → envuelve ese iterador para que además de cada elemento, te dé su índice.
*/
    for (i, b) in buffer.iter().enumerate() {
        // Si el byte actual coincide con el buscado, retornamos su posición
        if *b == buscado {
            return Some(i);
        }
    }
    // Si no se encontró el byte, retornamos None
    None
}

/// Punto de entrada del programa.
///
/// Busca un salto de línea (`\n`) dentro de un buffer de bytes
/// e imprime un mensaje indicando si se encontró y en qué posición.
fn main() {
    // Definimos un buffer de bytes con el contenido "rust\n"
    /*
El prefijo b antes del string literal le dice a Rust que lo trate como 
una secuencia de bytes en lugar de texto.
    */
    let buffer = b"rust\n";

    // Buscamos el byte de salto de línea en el buffer y mostramos el resultado
    match buscar_byte(buffer, b'\n') {
        Some(pos) => println!("Se encontró salto de línea en posición {}", pos),
        None => println!("No se encontró salto de línea"),
    }
}