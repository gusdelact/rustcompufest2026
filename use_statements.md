# Inventario de sentencias `use` (biblioteca estándar)

Este documento recopila todas las sentencias `use` de la biblioteca estándar de Rust (`std::`) encontradas en los ejemplos de las partes 1 a 8, agrupadas por módulo y con su propósito. Se excluyen los `use` que provienen de crates externos (por ejemplo, `signal_hook::*` en `parte4/05_signals_hilos.rs`).

---

## Resumen por módulo

| Módulo `std::` | Elementos importados | Para qué sirve |
|---|---|---|
| `std::env` | — | Acceso a variables de entorno y argumentos de la línea de comandos (`env::args()`). |
| `std::fs` | — | Operaciones de sistema de archivos (leer/escribir/listar). En `httpd.rs` se usa para `fs::read`. |
| `std::fs::File` | `File` | Abrir, crear y manipular archivos regulares. |
| `std::io` | `self` | Acceso al submódulo `io` para tipos como `io::stdin()`, `io::Result`, `io::Error`. |
| `std::io::BufRead` | `BufRead` | Trait que aporta lectura línea a línea (`.lines()`, `.read_line()`) sobre lectores con buffer. |
| `std::io::BufReader` | `BufReader` | Envolvente que añade buffering a cualquier `Read`, mejorando el rendimiento. |
| `std::io::Read` | `Read` | Trait base para leer bytes (`read`, `read_to_string`, `read_to_end`). |
| `std::io::Write` | `Write` | Trait base para escribir bytes (`write`, `write_all`, `flush`). |
| `std::net::TcpListener` | `TcpListener` | Servidor TCP: escucha conexiones entrantes en un puerto. |
| `std::net::TcpStream` | `TcpStream` | Conexión TCP ya establecida (implementa `Read` + `Write`). |
| `std::path::Path` | `Path` | Representación de rutas del sistema de archivos en forma prestada (`&Path`). |
| `std::path::PathBuf` | `PathBuf` | Ruta en propiedad (owned), mutable y constructible (`push`, `join`). |
| `std::path::Component` | `Component` | Segmentos de una ruta (normalización y validación contra *path traversal*). |
| `std::process::Command` | `Command` | Constructor y lanzador de procesos externos (equivalente a `fork`+`exec`). |
| `std::process::Stdio` | `Stdio` | Configuración de stdin/stdout/stderr de un proceso hijo (heredar, pipe, null). |
| `std::sync::Arc` | `Arc` | *Atomic Reference Counted*: compartir propiedad de datos entre hilos de forma segura. |
| `std::sync::Mutex` | `Mutex` | Exclusión mutua: un único hilo accede al recurso a la vez. |
| `std::sync::RwLock` | `RwLock` | Candado lectura/escritura: muchos lectores O un escritor exclusivo. |
| `std::sync::Condvar` | `Condvar` | Variable de condición: un hilo espera hasta que otro le avise. |
| `std::sync::mpsc` | — | *Multiple Producer, Single Consumer*: canales para enviar mensajes entre hilos. |
| `std::sync::atomic::AtomicBool` | `AtomicBool` | Booleano atómico, seguro entre hilos sin necesidad de `Mutex`. |
| `std::sync::atomic::Ordering` | `Ordering` | Nivel de garantías de memoria para operaciones atómicas (`Relaxed`, `SeqCst`, etc.). |
| `std::thread` | — | Crear y gestionar hilos (`thread::spawn`, `JoinHandle`). |
| `std::time::Duration` | `Duration` | Representar intervalos de tiempo (usado con `thread::sleep`). |
| `std::collections::VecDeque` | `VecDeque` | Cola doble-extremo usada como FIFO eficiente. |

---

## Detalle por archivo

### parte1 — Fundamentos de E/S

**parte1/06_buffer_archivo.rs**
```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader};
```
- `File`: abrir el archivo de entrada.
- `io`: acceso al módulo (para `io::Result`, errores).
- `BufRead`: trait que habilita `.lines()` sobre el `BufReader`.
- `BufReader`: envolver el `File` y leer por bloques/líneas eficientemente.

**parte1/09_integrado.rs** y **parte1/09_integrado_sin_unwrap.rs**
```rust
use std::fs::File;
use std::io::{BufRead, BufReader};
```
- Igual que el anterior, para leer un archivo de log línea a línea.

> Los archivos 01–05, 07 y 08 de `parte1` no importan nada: trabajan con tipos del preludio (`Vec`, `String`, `&str`, `enum`, `match`, `Option`).

---

### parte2 — Reimplementación de utilidades Unix

**parte2/01_cat.rs**
```rust
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
```
- `env`: leer los argumentos (`env::args()`) para saber qué archivo(s) imprimir.
- `File` + `BufReader` + `BufRead`: abrir y recorrer archivo por líneas.
- `io`: para `io::stdin()` cuando no se pasa archivo.

**parte2/02_echo.rs**
```rust
use std::env;
```
- `env`: recibir los argumentos a imprimir.

**parte2/03_head.rs**, **parte2/04_tail.rs**, **parte2/05_cut.rs**
```rust
use std::io::{self, BufRead};
```
- `io`: acceso a `io::stdin()`.
- `BufRead`: leer línea por línea desde stdin (tras envolver con `.lock()` o `BufReader`).

**parte2/06_od.rs**, **parte2/07_wc.rs**
```rust
use std::io::{self, Read};
```
- `Read`: permite `read_to_string` / `read_to_end` para consumir stdin como bytes crudos (necesario en `od` para el volcado hexadecimal y en `wc` para contar bytes).

---

### parte3 — Procesos e hilos

**parte3/01_proceso.rs**
```rust
use std::process::Command;
```
- `Command`: lanzar un proceso externo (`ls`, `echo`, etc.).

**parte3/02_proceso_con_pipe.rs**
```rust
use std::io::Write;
use std::process::{Command, Stdio};
```
- `Write`: `write_all` sobre el `stdin` del hijo.
- `Command`: crear el proceso.
- `Stdio`: configurar `stdin`/`stdout` como `piped()` para IPC.

**parte3/03_thread.rs**
```rust
use std::thread;
use std::time::Duration;
```
- `thread`: `thread::spawn` para crear hilos.
- `Duration`: medir/pausar tiempo con `thread::sleep`.

**parte3/04_move.rs**, **parte3/05_move_multihilos.rs**
```rust
use std::thread;
```
- Crear hilos que toman ownership con `move`.

---

### parte4 — Pipelines y señales

**parte4/03_pipeline_threads.rs**
```rust
use std::sync::mpsc;
use std::thread;
```
- `mpsc`: canal productor/consumidor entre hilos.
- `thread`: creación de hilos.

**parte4/04_signals.rs**
```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
```
- `AtomicBool` + `Ordering`: bandera compartida de terminación sin `Mutex`.
- `Arc`: compartir la bandera entre el hilo principal y el handler.
- `thread` y `Duration`: hilos + pausas.

**parte4/05_signals_hilos.rs** (solo los `std::`; `signal_hook::*` es un crate externo y se omite)
```rust
use std::sync::mpsc;
use std::thread;
```
- Canal `mpsc` para que el hilo que escucha señales avise al hilo principal.
- `thread`: hilo dedicado a recibir señales.

---

### parte5 — Primitivas de sincronización

**parte5/01_mutex.rs**
```rust
use std::sync::{Arc, Mutex};
use std::thread;
```
- `Arc<Mutex<T>>`: patrón clásico para compartir estado mutable entre hilos.

**parte5/02_rw_lock.rs**
```rust
use std::sync::{Arc, RwLock};
use std::thread;
```
- `RwLock`: muchos lectores concurrentes o un escritor exclusivo.

**parte5/03_condvar.rs**
```rust
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
```
- `Condvar`: espera/notificación; acompaña siempre a un `Mutex`.

**parte5/04_channels.rs**
```rust
use std::sync::mpsc;
use std::thread;
```
- Comunicación por paso de mensajes (alternativa a memoria compartida).

---

### parte6 — Problemas clásicos de concurrencia

**parte6/01_productor_consumidor.rs**
```rust
use std::sync::mpsc;
use std::thread;
```
- Canal entre productor y consumidor.

**parte6/02_lectores_escritores.rs**
```rust
use std::sync::{Arc, RwLock};
use std::thread;
```
- `RwLock` para modelar múltiples lectores y un escritor.

**parte6/03_filosofos_comensales.rs**
```rust
use std::sync::{Arc, Mutex};
use std::thread;
```
- `Mutex` para cada tenedor, `Arc` para compartirlos.

**parte6/04_barberia.rs**
```rust
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
```
- `VecDeque`: cola FIFO de clientes.
- `Mutex` + `Condvar`: dormir al barbero y despertarlo al llegar un cliente.
- `Arc`: compartir el estado entre los hilos de clientes y barbero.

---

### parte7 — Redes (TCP)

**parte7/01_cliente.rs**
```rust
use std::io::{Read, Write};
use std::net::TcpStream;
```
- `TcpStream`: conectar al servidor.
- `Read`/`Write`: leer/escribir sobre el socket.

**parte7/02_servidor.rs**
```rust
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
```
- `TcpListener`: aceptar conexiones.
- `TcpStream`: comunicarse con el cliente.

**parte7/03_servidor_multihilo.rs**, **parte7/04_net_echo.rs**
```rust
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
```
- Igual que el anterior, más `thread` para atender a cada cliente en su propio hilo.

**parte7/simple_http/httpd.rs**
```rust
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
```
- `fs`: leer archivos estáticos del sistema (`fs::read`).
- `BufReader` + `BufRead`: parsear la petición HTTP línea por línea.
- `Write`: enviar la respuesta al cliente.
- `TcpListener`/`TcpStream`: servidor TCP.
- `Path`, `PathBuf`, `Component`: construir y validar rutas de forma segura (evitar *path traversal* con `..`).

---

### parte8 — Versiones funcionales

Reutilizan los mismos tipos que sus contrapartes imperativas:

| Archivo | `use` std | Motivo |
|---|---|---|
| `p1_02_wc_funcional.rs` | `io::{self, Read}` | Leer stdin como bytes. |
| `p1_03_head_funcional.rs` | `io::{self, BufRead}` | Leer stdin por líneas. |
| `p1_04_log_funcional.rs` | `fs::File`, `io::{BufRead, BufReader}` | Leer archivo de log. |
| `p1_05_pipeline_funcional.rs` | `sync::mpsc`, `thread` | Pipeline con canales e hilos. |
| `p2_01_cat_funcional.rs` | `env`, `fs::File`, `io::{self, BufRead, BufReader}` | `cat` estilo funcional. |
| `p2_02_echo_funcional.rs` | `env` | Argumentos de CLI. |
| `p2_04_tail_funcional.rs` | `io::{self, BufRead}` | Lectura por líneas. |
| `p2_05_cut_funcional.rs` | `io::{self, BufRead}` | Lectura por líneas. |
| `p2_06_od_funcional.rs` | `io::{self, Read}` | Lectura de bytes. |
| `p3_01_proceso_funcional.rs` | `process::Command` | Lanzar proceso externo. |
| `p3_05_multihilos_funcional.rs` | `thread` | Crear hilos. |
| `p4_01_pipeline_funcional.rs` | `process::{Command, Stdio}` | Pipeline con procesos. |
| `p4_03_pipeline_threads_funcional.rs` | `sync::mpsc`, `thread` | Pipeline con hilos. |
| `p5_01_mutex_funcional.rs` | `sync::{Arc, Mutex}`, `thread` | Estado compartido. |
| `p5_04_channels_funcional.rs` | `sync::mpsc`, `thread` | Paso de mensajes. |
| `p6_01_productor_consumidor_funcional.rs` | `sync::mpsc`, `thread` | Prod/consumidor. |
| `p6_03_filosofos_funcional.rs` | `sync::{Arc, Mutex}`, `thread` | Tenedores compartidos. |
| `p7_01_cliente_funcional.rs` | `io::{Read, Write}`, `net::TcpStream` | Cliente TCP. |
| `p7_03_servidor_multihilo_funcional.rs` | `io::{Read, Write}`, `net::{TcpListener, TcpStream}`, `thread` | Servidor TCP multihilo. |
| `p7_04_echo_funcional.rs` | `io::{Read, Write}`, `net::{TcpListener, TcpStream}`, `thread` | Servidor eco multihilo. |

> `p1_01_buffer_funcional.rs` no tiene `use`: opera con `Vec<u8>` y combinadores del preludio (`iter`, `for_each`, `filter`, etc.).

---

## Nota sobre crates externos (excluidos)

En `parte4/05_signals_hilos.rs` se importan también:

```rust
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;
```

Pertenecen al crate `signal_hook` (declarado en `parte4/Cargo.toml`), por lo que quedan fuera del alcance de este inventario, enfocado en la biblioteca estándar.
