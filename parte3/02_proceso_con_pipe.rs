/* Proceso con pipe: parecido a redirigir stdin/stdout


Aquí ya sale el sabor Unix de siempre: un pipe entre padre e hijo. Stdio::piped() pide precisamente que se arregle una tubería para ese stream. Y conviene remarcar una advertencia útil: el handle Child no mata ni espera automáticamente al proceso al salir de scope; si no lo gestionas, el hijo puede seguir ejecutándose. Por eso aquí usamos wait_with_output().
*/

// Importamos el trait Write para poder usar write_all() sobre el stdin del proceso hijo
use std::io::Write;
// Importamos Command para ejecutar procesos externos y Stdio para configurar sus flujos de E/S
use std::process::{Command, Stdio};

/// Programa que demuestra la comunicación entre procesos (IPC) usando pipes en Rust.
///
/// Crea un proceso hijo ejecutando el comando `wc -c` (cuenta bytes de la entrada),
/// le envía una cadena de texto a través de su stdin mediante un pipe,
/// y luego lee el resultado desde su stdout.
fn main() -> std::io::Result<()> {
    // Creamos un proceso hijo que ejecuta `wc -c` (cuenta el número de bytes)
    // - stdin(Stdio::piped()): redirige el stdin para que podamos escribir en él desde Rust
    // - stdout(Stdio::piped()): redirige el stdout para que podamos leer su salida
    // - spawn(): lanza el proceso de forma asíncrona (no bloquea hasta que termine)
    let mut child = Command::new("wc")
        .arg("-c")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // Obtenemos una referencia mutable al stdin del proceso hijo con as_mut().
    // unwrap() es seguro aquí porque configuramos stdin como piped arriba.
    // Escribimos "hola rust\n" (10 bytes) en el stdin del proceso hijo.
    // Al terminar este bloque, el stdin se cierra implícitamente (drop),
    // lo que le indica a `wc` que no habrá más entrada y puede producir su resultado.
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"hola rust\n")?;

    // wait_with_output() hace dos cosas:
    // 1. Cierra el stdin del proceso hijo (si aún no se cerró)
    // 2. Espera a que el proceso termine y captura todo su stdout y stderr
    let output = child.wait_with_output()?;

    print!("salida de wc: {}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
