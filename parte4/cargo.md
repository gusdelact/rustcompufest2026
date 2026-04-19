# Cargo — El gestor de proyectos de Rust

## ¿Qué es Cargo?

Cargo es la herramienta que gestiona todo en un proyecto Rust: compilación, dependencias, testing, y distribución. Es el equivalente a `npm` en JavaScript, `pip` en Python, o `maven` en Java — pero viene incluido con Rust desde la instalación.

Hasta ahora compilábamos con `rustc` directamente:

```bash
rustc mi_programa.rs -o bin/mi_programa
```

Esto funciona mientras solo uses la biblioteca estándar. Pero en cuanto necesitas un crate externo (como `ctrlc` o `signal-hook`), `rustc` no sabe de dónde descargarlo. Ahí entra Cargo.

---

## ¿Por qué lo necesitamos?

En la parte 4, `04_signals.rs` usa el crate `ctrlc` para capturar Ctrl+C. Este crate no viene con Rust — hay que descargarlo de [crates.io](https://crates.io), el registro público de paquetes de Rust.

Sin Cargo, tendrías que descargar el código fuente manualmente, compilar sus dependencias, y pasarle todo a `rustc` con flags. Cargo hace todo eso por ti.

---

## Cargo.toml — El archivo de configuración

Cada proyecto Cargo tiene un `Cargo.toml` en la raíz. Es el equivalente a `package.json` en Node o `pom.xml` en Maven.

Nuestro `Cargo.toml` de la parte 4:

```toml
[package]
name = "parte4"
version = "0.1.0"
edition = "2021"

[dependencies]
ctrlc = "3.4"

[[bin]]
name = "01_simple_pipeline"
path = "01_simple_pipeline.rs"

[[bin]]
name = "02_tres_pipeline"
path = "02_tres_pipeline.rs"

[[bin]]
name = "03_pipeline_threads"
path = "03_pipeline_threads.rs"

[[bin]]
name = "04_signals"
path = "04_signals.rs"
```

### Secciones

**`[package]`** — Metadatos del proyecto:
- `name` — nombre del proyecto
- `version` — versión semántica (major.minor.patch)
- `edition` — edición de Rust a usar (2021 es la actual)

**`[dependencies]`** — Crates externos que necesita el proyecto:
- `ctrlc = "3.4"` significa "usa la versión 3.4.x más reciente compatible"
- Cargo descarga automáticamente el crate y todas sus dependencias transitivas

**`[[bin]]`** — Define múltiples binarios en el mismo proyecto:
- Cada `[[bin]]` es un ejecutable independiente
- `name` es el nombre del binario resultante
- `path` apunta al archivo `.rs` con el `fn main()`

---

## Comandos esenciales

### Crear un proyecto nuevo

```bash
cargo new mi_proyecto        # crea proyecto con src/main.rs
cargo init                   # inicializa en el directorio actual
```

### Compilar

```bash
cargo build                  # compilación debug (rápida, sin optimizar)
cargo build --release        # compilación release (optimizada)
```

Los binarios quedan en:
- `target/debug/` para build normal
- `target/release/` para build con `--release`

### Compilar un binario específico

```bash
cargo build --release --bin 04_signals
```

### Ejecutar directamente

```bash
cargo run --bin 04_signals   # compila y ejecuta en un solo paso
```

### Agregar dependencias

```bash
cargo add ctrlc              # agrega la última versión al Cargo.toml
cargo add signal-hook@0.3    # agrega una versión específica
```

### Otros comandos útiles

```bash
cargo check                  # verifica que compile sin generar binario (más rápido)
cargo test                   # ejecuta tests
cargo doc --open             # genera documentación y la abre en el navegador
cargo update                 # actualiza dependencias a las últimas versiones compatibles
cargo clean                  # borra el directorio target/
```

---

## Cargo.lock

La primera vez que compilas, Cargo genera un archivo `Cargo.lock` que fija las versiones exactas de todas las dependencias. Esto garantiza que todos los que compilen el proyecto usen exactamente las mismas versiones.

- `Cargo.toml` dice "quiero ctrlc 3.4.x"
- `Cargo.lock` dice "uso ctrlc 3.4.2 exactamente"

---

## crates.io

[crates.io](https://crates.io) es el registro público de crates de Rust. Cualquiera puede publicar un crate ahí. Cuando escribes `ctrlc = "3.4"` en tu `Cargo.toml`, Cargo lo descarga de crates.io automáticamente.

Puedes buscar crates en:
- https://crates.io — el registro oficial
- https://lib.rs — interfaz alternativa con mejor búsqueda y categorización

---

## rustc vs Cargo

| | `rustc` | `cargo` |
|---|---|---|
| Compila | Un archivo `.rs` | Un proyecto completo |
| Dependencias | Solo biblioteca estándar | Descarga crates de crates.io |
| Configuración | Flags en la línea de comandos | `Cargo.toml` |
| Múltiples binarios | Compilar uno por uno | `[[bin]]` en Cargo.toml |
| Tests | Manual | `cargo test` |
| Cuándo usarlo | Archivos simples sin dependencias | Cualquier proyecto real |

`rustc` es el compilador. Cargo es la herramienta que orquesta al compilador, las dependencias, y todo lo demás. En la práctica, casi siempre usas Cargo.

---

## Estructura típica de un proyecto Cargo

```
mi_proyecto/
├── Cargo.toml          # configuración del proyecto
├── Cargo.lock          # versiones exactas (generado automáticamente)
├── src/
│   └── main.rs         # punto de entrada por defecto
└── target/             # binarios compilados (generado automáticamente)
    ├── debug/
    └── release/
```

En nuestro caso, la parte 4 no usa `src/` — cada `.rs` está en la raíz y se declara explícitamente como `[[bin]]` en el `Cargo.toml`. Ambos enfoques son válidos.
