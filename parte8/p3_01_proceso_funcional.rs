/*
Proceso — versión funcional (parte3/01_proceso.rs)

Imperativo: Command con .arg() encadenado.
Funcional: usar un array de argumentos con .args() y .map() sobre el resultado.

El original ya es bastante limpio. Aquí mostramos cómo encadenar
múltiples comandos de forma funcional usando arrays y closures.
*/

use std::process::Command;

fn main() {
    // Definimos los comandos como datos (array de tuplas).
    // Cada tupla es (comando, argumentos).
    let comandos: Vec<(&str, Vec<&str>)> = vec![
        ("echo", vec!["hola", "desde", "funcional"]),
        ("ls", vec!["-l", "/tmp"]),
    ];

    // .iter() recorre los comandos.
    // .for_each() ejecuta cada uno.
    // .map() sobre el Result transforma el ExitStatus.
    comandos.iter().for_each(|(cmd, args)| {
        Command::new(cmd)
            .args(args)
            .status()
            .map(|status| println!("{} terminó con: {}", cmd, status))
            .unwrap_or_else(|e| eprintln!("Error ejecutando {}: {}", cmd, e));
    });
}
