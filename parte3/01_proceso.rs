/*
  Proceso estilo Unix: lanzar un comando
  “En C/Unix clásico yo pensaba en fork() y luego exec().
En Rust, para empezar, pienso en Command::new(...).status() o spawn().”
*/

use std::process::Command;

/// Punto de entrada del programa.
///
/// Ejecuta el comando `ls -l /tmp` como un proceso hijo y muestra
/// el código de salida resultante en la consola.
///
/// # Errores
///
/// Retorna un `std::io::Error` si no se puede iniciar el proceso hijo
/// (por ejemplo, si el comando `ls` no existe en el sistema).
fn main() -> std::io::Result<()> {
    // Crea y ejecuta un proceso hijo con el comando `ls -l /tmp`.
    // - `Command::new("ls")`: define el programa a ejecutar.
    // - `.arg("-l")`: agrega el argumento de listado largo.
    // - `.arg("/tmp")`: especifica el directorio a listar.
    // - `.status()`: ejecuta el comando y espera a que termine,
    //   devolviendo un `ExitStatus` con el resultado.
    let status = Command::new("ls")
        .arg("-l")
        .arg("/tmp")
        .status()?;

    // Imprime el código de salida del proceso hijo.
    println!("terminó con: {status}");
    Ok(())
}