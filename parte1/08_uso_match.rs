/*
match como reemplazo de switch

*/

/// Clasifica un byte según su valor y muestra su categoría por consola.
///
/// # Categorías
/// - `0`: Byte nulo (NUL)
/// - `10` o `13`: Caracteres de fin de línea (LF y CR)
/// - `32..=126`: Caracteres ASCII imprimibles
/// - Cualquier otro valor: Byte no clasificado
///
/// # Argumentos
/// * `b` - El byte a clasificar
///
/// # Ejemplo
/// ```
/// clasificar_byte(65); // Imprime: "ASCII imprimible"
/// ```
fn clasificar_byte(b: u8) {
    match b {
        0 => println!("NUL"),
        10 | 13 => println!("Fin de línea"),
        32..=126 => println!("ASCII imprimible"),
        _ => println!("Otro byte"),
    }
}

/// Punto de entrada del programa.
///
/// Recorre una lista de bytes de ejemplo y clasifica cada uno
/// utilizando la función [`clasificar_byte`].
fn main() {
    for b in [0u8, 10, 65, 200,13] {
        clasificar_byte(b);
    }
}