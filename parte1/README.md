# Parte 1 — Las cuatro piezas de la programación de sistemas en Rust

En la programación de sistemas, casi todo se construye con cuatro piezas fundamentales:

1. **Buffers de bytes** — el sistema operativo entrega y recibe datos como bytes.
2. **Texto** — muchas veces esos bytes representan líneas, comandos, rutas o logs.
3. **Estado** — el programa necesita recordar en qué situación está y qué ha pasado.
4. **Handles de recurso** — el programa abre archivos, usa sockets, lanza procesos y trabaja con recursos del sistema operativo.

Rust no inventa esas cuatro piezas; lo que hace es volverlas más explícitas, más seguras y más claras.

---

## Programas

### 01_buffer.rs — Buffer dinámico de bytes
Crea un `Vec<u8>` con bytes crudos y recorre cada uno con `for`. Usa `if` para decidir si un byte es ASCII imprimible o un carácter de control.

**Pieza:** buffers de bytes.
**Conceptos:** `Vec<u8>`, `&buffer` (préstamo), `*byte` (desreferencia), rango ASCII imprimible.

---

### 02_buffer_fijo.rs — Buffer fijo con variable de control
Simula la lectura de bytes en un buffer de tamaño fijo `[u8; 4]`, llenándolo y vaciándolo manualmente con un contador `usados`. Muy cercano a cómo se trabaja en C.

**Pieza:** buffers de bytes.
**Conceptos:** `b"..."` (byte string literal), `[0u8; 4]` (array fijo), slices (`&buf[..usados]`), `String::from_utf8_lossy`.

---

### 03_logging_str.rs — Logging con texto
Construye una línea de log usando `format!` a partir de `&str` y la imprime si supera una longitud mínima.

**Pieza:** texto.
**Conceptos:** `&str` (texto prestado, vive en el binario), `String` (texto con dueño, vive en el heap), `format!`, `.len()`.

---

### 04_enum_estado.rs — Estado del sistema con enum y match
Define un enum `EstadoFd` con variantes `Abierto`, `Cerrado` y `Error(i32)` para representar el estado de un file descriptor. Usa `match` para manejar cada caso.

**Pieza:** estado.
**Conceptos:** `enum` con datos asociados, `match` exhaustivo, destructuring (`Error(codigo)`), diferencias con `switch`.

---

### 05_option_resultado.rs — Búsqueda con Option
Busca un byte dentro de un buffer y retorna `Option<usize>`: `Some(pos)` si lo encuentra, `None` si no. Reemplaza la convención de C de retornar `-1` o `NULL`.

**Pieza:** estado + buffers de bytes.
**Conceptos:** `Option<T>`, `Some`/`None`, `.iter().enumerate()`, `match` sobre `Option`.

---

### 06_buffer_archivo.rs — Lectura desde archivo o stdin
Lee líneas desde la entrada estándar o desde un archivo, clasificándolas con `match`. Usa una función genérica con el trait `BufRead` para aceptar ambas fuentes.

**Pieza:** handles de recurso + texto.
**Conceptos:** `File::open`, `BufReader`, `stdin.lock()`, generics con traits (`T: BufRead`), `.lines()`, `match` sobre `Result`.

---

### 07_uso_path.rs — Apertura de archivo con Path
Abre un archivo usando `Path` en lugar de un string crudo. Maneja el resultado con `match` para distinguir éxito de error.

**Pieza:** handles de recurso.
**Conceptos:** `Path::new`, `File::open`, `match` sobre `Result<File, Error>`.

---

### 08_uso_match.rs — Clasificación de bytes con match
Clasifica bytes individuales en categorías (NUL, fin de línea, ASCII imprimible, otro) usando `match` con patrones de rango y alternativas.

**Pieza:** buffers de bytes + estado.
**Conceptos:** `match` con rangos (`32..=126`), alternativas (`10 | 13`), comodín (`_`).

---

### 09_integrado.rs — Ejemplo integrador: analizador de logs
Lee `sistema.log` línea por línea, clasifica cada entrada por nivel de severidad (`INFO`, `WARN`, `ERROR`) usando un enum y `match`, y muestra un resumen con conteos al final.

**Pieza:** las cuatro juntas — buffers, texto, estado y handles de recurso.
**Conceptos:** `enum`, `match`, `File::open`, `BufReader`, `.lines()`, contadores mutables.

---

### 09_integrado_sin_unwrap.rs — Integrador sin unwrap
Misma lógica que `09_integrado.rs`, pero reemplaza los `unwrap()` por el operador `?` para propagar errores de forma segura. La lógica se mueve a una función que retorna `Result`.

**Pieza:** las cuatro juntas + manejo de errores idiomático.
**Conceptos:** `Result<(), std::io::Error>`, operador `?`, `eprintln!`, separación entre lógica y `main`.

---

## Cómo compilar y ejecutar

```bash
# Compilar un programa individual
rustc 01_buffer.rs -o bin/01_buffer

# Ejecutar
./bin/01_buffer

# Para 09_integrado, asegúrate de tener sistema.log en el mismo directorio
rustc 09_integrado.rs -o bin/09_integrado
./bin/09_integrado
```
