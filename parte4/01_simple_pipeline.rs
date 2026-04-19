/*
Ejemplo que muestra como se escribe un pipe usando Rust


*/

use std::process::{Command, Stdio};

/// Programa principal que demuestra cómo encadenar dos procesos mediante pipes (tuberías)
/// en Rust, simulando el comando de shell: `echo hola | wc -c`
///
/// Flujo:
/// 1. Se crea un proceso hijo que ejecuta `echo hola`, redirigiendo su salida estándar a un pipe.
/// 2. Se crea un segundo proceso hijo que ejecuta `wc -c`, tomando como entrada estándar
///    la salida del primer proceso.
/// 3. Se captura la salida del segundo proceso y se imprime en pantalla.
///
/// El resultado esperado es `5`, que corresponde a los 5 bytes de "hola\n"
/// (4 caracteres + 1 salto de línea que `echo` añade automáticamente).
fn main() -> std::io::Result<()> {
    // Creamos el primer proceso hijo: `echo hola`
    // - `Command::new("echo")`: invoca el comando `echo` del sistema operativo.
    // - `.arg("hola")`: pasa "hola" como argumento al comando.
    // - `.stdout(Stdio::piped())`: redirige la salida estándar (stdout) del proceso
    //   a un pipe en lugar de imprimirla en la terminal. Esto nos permite conectarla
    //   como entrada de otro proceso.
    // - `.spawn()`: lanza el proceso de forma asíncrona (no espera a que termine).
    //   Devuelve un `Result<Child>`, donde `Child` representa el proceso en ejecución.
    let mut echo = Command::new("echo")
        .arg("hola")
        .stdout(Stdio::piped())
        .spawn()?;

    // Creamos el segundo proceso hijo: `wc -c`
    // - `Command::new("wc")`: invoca el comando `wc` (word count) del sistema.
    // - `.arg("-c")`: el flag `-c` indica a `wc` que cuente bytes en lugar de palabras.
    // - `echo.stdout.take().unwrap()`: extrae el extremo de lectura del pipe del primer
    //   proceso. `take()` devuelve un `Option<ChildStdout>` y transfiere la propiedad
    //   (deja `None` en su lugar para evitar doble uso). `unwrap()` extrae el valor
    //   asumiendo que existe (es seguro aquí porque configuramos `Stdio::piped()`).
    // - `.stdin(...)`: conecta la salida del primer proceso como entrada estándar del segundo,
    //   creando así la tubería entre ambos procesos.
    // - `.output()`: ejecuta el proceso y espera a que termine, capturando toda su salida
    //   estándar y de error en un struct `Output`.
    let output = Command::new("wc")
        .arg("-c")
        .stdin(echo.stdout.take().unwrap())
        .output()?;

    // Imprimimos el resultado del segundo proceso.
    // - `output.stdout`: contiene la salida estándar de `wc -c` como un `Vec<u8>` (bytes).
    // - `String::from_utf8_lossy(...)`: convierte los bytes a un `String` de forma segura.
    //   Si hay bytes inválidos en UTF-8, los reemplaza con el carácter de reemplazo Unicode (�)
    //   en lugar de fallar, de ahí el nombre "lossy" (con pérdida).
    // - `print!`: imprime sin salto de línea adicional (a diferencia de `println!`).
    print!("{}", String::from_utf8_lossy(&output.stdout));

    // Retornamos `Ok(())` indicando que todo se ejecutó correctamente.
    // El operador `?` usado anteriormente propaga automáticamente cualquier error de tipo
    // `std::io::Error` que ocurra durante la ejecución de los comandos.
    Ok(())
}

/*
Este programa **no va a funcionar en PowerShell de Windows** porque depende de dos comandos Unix:

- `echo` — en Windows existe, pero se comporta diferente. En PowerShell, `echo` es un alias de `Write-Output`, que es un cmdlet de PowerShell, no un ejecutable externo. `Command::new("echo")` busca un ejecutable binario, no un cmdlet. Dependiendo de la versión, podría encontrar `echo.exe` o fallar.

- `wc` — **no existe en Windows**. El programa va a fallar con un error de tipo `std::io::Error` (archivo no encontrado) al intentar hacer `Command::new("wc")`.

El error que verías sería algo como:

```
Error: Os { code: 2, kind: NotFound, message: "El sistema no puede encontrar el archivo especificado." }
```

Para que funcione en Windows tendrías algunas opciones:

1. Instalar herramientas Unix vía WSL, Git Bash o MSYS2 que incluyen `echo` y `wc`.

2. Usar los equivalentes de PowerShell invocando `cmd.exe` o `powershell.exe`:
```rust
// En lugar de "echo" y "wc", usar cmd.exe
Command::new("cmd")
    .args(["/C", "echo hola"])
    .stdout(Stdio::piped())
    .spawn()?;
```

3. Hacer la lógica en Rust puro sin depender de comandos externos — contar los bytes directamente en el programa en lugar de llamar a `wc`.

La opción 3 es la más portable. El programa actual está escrito asumiendo un entorno Unix/Linux/macOS.

*/

/*
Con Docker en Windows puedes levantar un contenedor con una imagen ligera como Alpine Linux (pesa ~5 MB), que ya trae echo, wc y todas las utilidades básicas de Unix.

Sería algo como:

# Compilar dentro del contenedor
docker run --rm -v ${PWD}:/code -w /code rust:alpine rustc 01_simple_pipeline.rs -o 01_simple_pipeline

# Ejecutar
docker run --rm -v ${PWD}:/code -w /code alpine ./01_simple_pipeline
O tener un Dockerfile mínimo con Rust + Alpine para trabajar todo el curso ahí dentro.

*/