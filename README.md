# Fundamentos de programación de sistemas con Rust


## Instalación Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

# Programación de sistemas desde la perspectiva de Rust

## ¿Qué es la programación de sistemas?

La programación de sistemas es escribir software que interactúa directamente con el hardware y el sistema operativo. No es hacer una app con botones ni una página web — es construir lo que está debajo: kernels, drivers, bases de datos, servidores de red, compiladores, sistemas embebidos.

El programador de sistemas trabaja con lo que el sistema operativo expone: bytes, descriptores de archivo, memoria, procesos, señales. No hay capas de abstracción que te protejan. Si algo sale mal, no hay excepción bonita — hay un segfault, corrupción de memoria o un bug silencioso que aparece tres meses después.

Históricamente, este territorio ha sido de C y C++. Rust entra como alternativa con una propuesta clara: darte el mismo nivel de control, pero con garantías que antes no existían.

---

## Las cuatro piezas fundamentales

Casi todo programa de sistemas se construye combinando cuatro elementos:

### 1. Buffers de bytes

El sistema operativo no entiende de strings, JSON ni objetos. Entiende bytes. Cuando lees un archivo, recibes bytes. Cuando envías datos por un socket, envías bytes. Cuando un dispositivo te manda información, llegan bytes.

Un buffer es simplemente un bloque de memoria donde almacenas esos bytes temporalmente mientras los procesas.

En C esto es un `unsigned char[]` o un `void*` con un tamaño. El problema: nada te impide leer más allá del final del buffer, escribir donde no debes, o usar el buffer después de liberarlo.

En Rust, un buffer puede ser:

- `Vec<u8>` — dinámico, crece según necesites, el compilador sabe cuándo liberarlo.
- `[u8; N]` — fijo, tamaño conocido en compilación, vive en el stack.
- `&[u8]` — un slice, una vista a bytes que viven en otro lado, con tamaño conocido.

```rust
// Buffer dinámico
let buffer: Vec<u8> = vec![72, 101, 108, 108, 111];

// Buffer fijo
let buf = [0u8; 1024];

// Slice: vista parcial sin copiar
let primeros = &buf[..10];
```

Rust garantiza en compilación que no puedes acceder fuera de los límites de un slice, que no puedes usar un buffer después de moverlo, y que no puedes tener dos partes del código modificándolo al mismo tiempo.

### 2. Texto

Muchos de esos bytes resultan ser texto: líneas de log, rutas de archivo, comandos, configuraciones, respuestas HTTP. El programa necesita interpretar bytes como texto, manipularlo y producir texto nuevo.

En C, un string es un `char*` terminado en null. No sabes su longitud sin recorrerlo, no sabes si es UTF-8 válido, y funciones como `sprintf` y `strcat` son fuente constante de vulnerabilidades.

Rust separa el texto en dos tipos:

- `&str` — texto prestado, inmutable, garantizado UTF-8. Puede apuntar a un literal en el binario o a parte de un `String`.
- `String` — texto con dueño, vive en el heap, puede crecer y modificarse.

```rust
let modulo: &str = "net";                    // prestado, vive en el binario
let log = format!("[INFO] modulo={}", modulo); // String, construido en runtime
```

No existe el null terminator. La longitud siempre es conocida. Y si intentas crear un `&str` con bytes que no son UTF-8, el compilador o el runtime te lo impiden.

### 3. Estado

Un programa de sistemas no es una función pura que recibe datos y devuelve un resultado. Tiene estado: ¿el socket está abierto o cerrado? ¿La operación anterior falló? ¿En qué fase del protocolo estamos?

En C, el estado se maneja con enteros, `#define`, banderas de bits, o convenciones informales. Un file descriptor es un `int`. Un error es `-1`. Un puntero inválido es `NULL`. Nada en el tipo te dice qué valores son válidos ni qué significan.

Rust usa el sistema de tipos para hacer el estado explícito:

```rust
enum EstadoFd {
    Abierto,
    Cerrado,
    Error(i32),   // el código de error viaja dentro del tipo
}
```

`match` te obliga a manejar todos los casos posibles. Si agregas una variante nueva, el código no compila hasta que la atiendas en cada `match`.

Para operaciones que pueden fallar, Rust tiene `Result<T, E>` (éxito o error) y `Option<T>` (valor presente o ausente). No hay `-1` mágico ni `NULL` — el tipo mismo te dice que algo puede no estar ahí.

```rust
fn buscar_byte(buffer: &[u8], buscado: u8) -> Option<usize> {
    for (i, b) in buffer.iter().enumerate() {
        if *b == buscado {
            return Some(i);
        }
    }
    None
}
```

### 4. Handles de recurso

El programa no vive solo en memoria. Abre archivos, crea sockets, lanza procesos, mapea memoria. Cada uno de estos es un recurso del sistema operativo que debe abrirse, usarse y cerrarse correctamente.

En C, un recurso es un entero (file descriptor) o un puntero opaco. Olvidar cerrarlo es un leak. Usarlo después de cerrarlo es undefined behavior. Cerrarlo dos veces es otro bug.

En Rust, los recursos son valores con dueño. Cuando el valor sale de scope, el recurso se libera automáticamente (RAII). No necesitas `close()` explícito — el compilador lo hace por ti.

```rust
use std::fs::File;
use std::path::Path;

let ruta = Path::new("datos.txt");
match File::open(ruta) {
    Ok(archivo) => { /* usar archivo */ }
    Err(e) => println!("Error: {}", e),
}
// archivo se cierra automáticamente aquí
```

`File` no es un entero — es un tipo que encapsula el recurso, garantiza su limpieza, y no te deja usarlo después de cerrarlo porque el sistema de ownership lo impide.

---

## Lo que Rust cambia respecto a C

Rust no cambia qué haces — sigues trabajando con bytes, texto, estado y recursos del sistema. Lo que cambia es cómo lo haces:

| Concepto | C | Rust |
|---|---|---|
| Buffer | `char buf[1024]`, sin verificación de límites | `[u8; 1024]` o `Vec<u8>`, verificado en compilación |
| Texto | `char*` con null terminator | `&str` / `String`, UTF-8 garantizado, longitud conocida |
| Estado | Enteros, `#define`, convenciones | `enum`, `Option`, `Result`, verificado por el compilador |
| Recursos | File descriptors enteros, `close()` manual | Tipos con ownership, liberación automática (RAII) |
| Errores | `-1`, `NULL`, `errno` | `Result<T, E>`, `Option<T>`, el compilador te obliga a manejarlos |
| Memoria | `malloc`/`free`, sin verificación | Ownership + borrowing, verificado en compilación |

---

## Ownership y borrowing: la idea central

Todo lo anterior funciona gracias a una idea que no existe en C ni en la mayoría de lenguajes: cada valor en Rust tiene exactamente un dueño.

- Cuando el dueño sale de scope, el valor se libera.
- Puedes prestar el valor con `&` (referencia inmutable) o `&mut` (referencia mutable).
- No puedes tener una referencia mutable y otra inmutable al mismo tiempo.
- No puedes usar un valor después de moverlo.

Estas reglas se verifican en compilación. No hay garbage collector, no hay runtime. El costo es cero en ejecución. El precio es que el compilador te rechaza código que en C compilaría sin quejarse — pero que en C tendría bugs.

```rust
let buffer = vec![1, 2, 3];

// Préstamo: la función lee el buffer sin consumirlo
for byte in &buffer {
    println!("{}", byte);
}

// buffer sigue siendo válido aquí
println!("{:?}", buffer);
```

---

## ¿Para quién es la programación de sistemas en Rust?

Para quien necesita control sobre la máquina sin sacrificar seguridad. Rust no es más fácil que C — en muchos sentidos es más exigente, porque el compilador te obliga a pensar en cosas que en C simplemente ignorabas. Pero los bugs que previene (use-after-free, buffer overflow, data races) son exactamente los que causan las vulnerabilidades más graves en software de sistemas.

Rust te deja pensar a bajo nivel — bytes, memoria, recursos del sistema — pero con un compilador que actúa como un revisor de código implacable que nunca se cansa.

