/*
Pipeline simple — versión funcional (parte4/01_simple_pipeline.rs)

El original encadena echo | wc -c con procesos.
Aquí hacemos lo mismo pero la lógica de captura y conversión
usa combinadores funcionales en lugar de variables intermedias.
*/

use std::process::{Command, Stdio};

fn main() -> std::io::Result<()> {
    let mut echo = Command::new("echo")
        .arg("hola")
        .stdout(Stdio::piped())
        .spawn()?;

    // Encadenamiento funcional: take() → map() → unwrap()
    // en lugar de asignar a variable intermedia.
    let output = echo
        .stdout
        .take()
        .map(|stdout| {
            Command::new("wc")
                .arg("-c")
                .stdin(stdout)
                .output()
        })
        .unwrap()?;

    // Conversión funcional de bytes a string.
    String::from_utf8(output.stdout)
        .map(|s| print!("{}", s.trim()))
        .unwrap_or_else(|_| print!("(salida no UTF-8)"));

    println!();
    Ok(())
}
