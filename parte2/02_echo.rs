/*
echo — imprimir argumentos
Ideas clave
manejo de argumentos
for con índice (enumerate)
control de formato con if
*/

/// Programa que replica el comportamiento del comando `echo` de Unix.
///
/// Toma todos los argumentos pasados por línea de comandos,
/// los concatena separados por un espacio, e imprime el resultado
/// seguido de un salto de línea.
///
/// # Ejemplos de uso
///
/// ```bash
/// cargo run -- hola mundo
/// # Salida: hola mundo
/// ```
///
/// ```bash
/// cargo run
/// # Salida: (línea vacía)
/// ```
use std::env;

fn main() {
    // Recopilamos los argumentos de la línea de comandos en un vector de Strings.
    // `env::args()` devuelve un iterador donde el primer elemento (índice 0)
    // es la ruta del propio ejecutable, por lo que usamos `.skip(1)` para
    // omitirlo y quedarnos solo con los argumentos reales del usuario.
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        // Si no se proporcionaron argumentos, simplemente imprimimos
        // una línea vacía (equivalente a ejecutar `echo` sin argumentos).
        println!();
    } else {
        // Recorremos cada argumento junto con su índice usando `enumerate()`.
        // `enumerate()` devuelve tuplas (índice, valor) para cada elemento.
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                // A partir del segundo argumento (índice > 0), imprimimos
                // un espacio antes del argumento para separarlos entre sí.
                // Esto evita un espacio extra al inicio de la salida.
                print!(" ");
            }
            // Imprimimos el argumento actual sin salto de línea al final.
            // `print!` (a diferencia de `println!`) no añade '\n'.
            print!("{}", arg);
        }
        // Finalmente, imprimimos un salto de línea para cerrar la salida,
        // tal como lo haría el comando `echo` en una terminal.
        println!();
    }
}