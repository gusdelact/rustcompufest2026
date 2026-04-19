/*

Pipeline de tres etapas: cat archivo | grep foo | wc -l

*/


use std::process::{Command, Stdio};

/// Programa que simula el pipeline de shell: `cat archivo.txt | grep foo | wc -l`
///
/// Encadena tres procesos hijos conectando la salida estándar (stdout) de cada uno
/// con la entrada estándar (stdin) del siguiente, tal como lo haría una tubería (pipe)
/// en la terminal.
///
/// 1. `cat archivo.txt`  — lee el contenido del archivo.
/// 2. `grep foo`         — filtra solo las líneas que contienen "foo".
/// 3. `wc -l`            — cuenta cuántas líneas pasaron el filtro.
fn main() -> std::io::Result<()> {
    // --- Primer proceso: cat archivo.txt ---
    // Se lanza `cat` para leer el archivo "archivo.txt".
    // `stdout(Stdio::piped())` indica que la salida estándar del proceso
    // se redirige a un pipe, en lugar de imprimirse en la terminal,
    // para que otro proceso pueda consumirla.
    let mut cat = Command::new("cat")
        .arg("archivo.txt")       // Archivo de entrada que será leído
        .stdout(Stdio::piped())   // Redirige stdout a un pipe interno
        .spawn()?;                // Lanza el proceso hijo; `?` propaga errores

    // --- Segundo proceso: grep foo ---
    // Se lanza `grep` para filtrar las líneas que contengan la cadena "foo".
    // `cat.stdout.take().unwrap()` extrae el extremo de lectura del pipe de `cat`
    // y lo conecta como entrada estándar (stdin) de `grep`.
    // `.take()` devuelve un `Option` y deja `None` en su lugar, transfiriendo
    // la propiedad del recurso. `.unwrap()` lo desenvuelve asumiendo que existe.
    let mut grep = Command::new("grep")
        .arg("foo")                           // Patrón a buscar en cada línea
        .stdin(cat.stdout.take().unwrap())    // Conecta stdout de cat → stdin de grep
        .stdout(Stdio::piped())              // Redirige stdout de grep a otro pipe
        .spawn()?;                            // Lanza el proceso hijo

    // --- Tercer proceso: wc -l ---
    // Se lanza `wc -l` para contar el número de líneas recibidas.
    // Se usa `.output()` en lugar de `.spawn()` porque queremos esperar
    // a que el proceso termine y capturar su salida completa de una vez.
    let output = Command::new("wc")
        .arg("-l")                             // Flag para contar líneas
        .stdin(grep.stdout.take().unwrap())   // Conecta stdout de grep → stdin de wc
        .output()?;                            // Ejecuta, espera y captura la salida

    // Imprime el resultado final.
    // `output.stdout` es un `Vec<u8>` (bytes crudos), así que usamos
    // `String::from_utf8_lossy` para convertirlo a texto de forma segura,
    // reemplazando bytes inválidos con el carácter de reemplazo Unicode (�).
    print!("{}", String::from_utf8_lossy(&output.stdout));

    // Retorna Ok(()) indicando que todo se ejecutó sin errores de I/O.
    Ok(())
}