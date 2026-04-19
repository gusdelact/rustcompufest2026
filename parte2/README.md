# Parte 2 — Comandos Unix/Linux en Rust

El objetivo de esta parte es ilustrar, con ejemplos simples, cómo se construyen utilidades clásicas de línea de comandos al estilo Unix/Linux usando Rust.

Cada programa reimplementa un comando conocido en su forma más básica. La idea no es replicar todas las opciones del comando original, sino mostrar los patrones de programación de sistemas que hay detrás: leer stdin, procesar líneas, manejar argumentos, trabajar con bytes crudos.

---

## Programas

### 01_cat.rs — Imprimir archivo o stdin
Replica `cat`. Si recibe un archivo como argumento, imprime su contenido. Si no, lee desde stdin. Permite encadenar con pipes.

```bash
./01_cat archivo.txt
echo "hola" | ./01_cat
```

**Conceptos:** `env::args()`, `File::open`, `BufReader`, generics con `BufRead`, `match` sobre `Result`.

---

### 02_echo.rs — Imprimir argumentos
Replica `echo`. Toma los argumentos de la línea de comandos, los une con espacios y los imprime.

```bash
./02_echo hola mundo
# Salida: hola mundo
```

**Conceptos:** `env::args().skip(1)`, `enumerate()`, `print!` vs `println!`, control de formato con `if`.

---

### 03_head.rs — Primeras N líneas
Replica `head -n 5`. Lee desde stdin e imprime solo las primeras 5 líneas.

```bash
cat archivo.txt | ./03_head
```

**Conceptos:** contador como variable de control, `if` + `break`, patrón clásico de lectura limitada.

---

### 04_tail.rs — Últimas N líneas
Replica `tail -n 5`. Lee toda la entrada y muestra solo las últimas 5 líneas usando un buffer tipo FIFO.

```bash
cat archivo.txt | ./04_tail
```

**Conceptos:** `Vec<String>` como buffer de ventana deslizante, `push` + `remove(0)` para comportamiento FIFO.

---

### 05_cut.rs — Seleccionar columnas
Replica `cut` simplificado. Divide cada línea por espacios en blanco y extrae la columna indicada.

```bash
echo "hola mundo cruel" | ./05_cut
# Salida: mundo  (columna 1, índice base 0)
```

**Conceptos:** `split_whitespace()`, `Vec<&str>`, verificación de límites antes de acceder por índice.

---

### 06_od.rs — Volcado hexadecimal
Replica `od -x` / `hexdump`. Lee bytes crudos desde stdin y los muestra en formato hexadecimal, 16 por línea.

```bash
echo "Hola" | ./06_od
# Salida: 48 6f 6c 61 0a
```

**Conceptos:** buffer fijo `[u8; 16]`, lectura binaria con `Read::read`, formato `{:02x}`, loop con `match` sobre `Ok(0)` para detectar EOF.

---

### 07_wc.rs — Contar líneas, palabras y bytes
Replica `wc`. Lee toda la entrada y muestra el conteo de líneas, palabras y bytes.

```bash
cat archivo.txt | ./07_wc
# Salida: 10 42 256
```

**Conceptos:** `read_to_string`, `.lines().count()`, `.split_whitespace().count()`, `.len()` para bytes.

---

## Cómo compilar y ejecutar

```bash
# Compilar
rustc 01_cat.rs -o bin/01_cat

# Ejecutar con archivo
./bin/01_cat archivo.txt

# Ejecutar con pipe
echo "hola mundo" | ./bin/01_cat
```

## Patrón común

Todos los programas siguen la misma estructura que las utilidades Unix reales:

1. Leer desde stdin o un archivo
2. Procesar línea por línea (o bloque por bloque)
3. Escribir el resultado en stdout
4. Reportar errores en stderr

Esto permite encadenarlos con pipes, exactamente como sus equivalentes en un shell real.
