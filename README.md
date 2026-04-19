# Programación de sistemas desde la perspectiva de Rust

## ¿Qué es la programación de sistemas?

La programación de sistemas es escribir software que interactúa directamente con el hardware y el sistema operativo. No es hacer una app con botones ni una página web — es construir lo que está debajo: kernels, drivers, bases de datos, servidores de red, compiladores, sistemas embebidos.

El programador de sistemas trabaja con lo que el sistema operativo expone: bytes, descriptores de archivo, memoria, procesos, señales, sockets. No hay capas de abstracción que te protejan. Si algo sale mal, no hay excepción bonita — hay un segfault, corrupción de memoria o un bug silencioso que aparece tres meses después.

Históricamente, este territorio ha sido de C y C++. Rust entra como alternativa con una propuesta clara: darte el mismo nivel de control, pero con garantías que antes no existían.

Este documento describe qué es la programación de sistemas a través de lo que realmente se construye en este curso: desde buffers de bytes hasta servidores HTTP, pasando por utilidades Unix, procesos, hilos, sincronización y problemas clásicos de concurrencia.

---

## Las piezas fundamentales

Casi todo programa de sistemas se construye combinando estos elementos:

### 1. Buffers de bytes

El sistema operativo no entiende de strings, JSON ni objetos. Entiende bytes. Cuando lees un archivo, recibes bytes. Cuando envías datos por un socket, envías bytes. Cuando un dispositivo te manda información, llegan bytes.

Un buffer es simplemente un bloque de memoria donde almacenas esos bytes temporalmente mientras los procesas.

En C esto es un `unsigned char[]` o un `void*` con un tamaño. El problema: nada te impide leer más allá del final del buffer, escribir donde no debes, o usar el buffer después de liberarlo.

En Rust, un buffer puede ser:

- `Vec<u8>` — dinámico, crece según necesites, el compilador sabe cuándo liberarlo.
- `[u8; N]` — fijo, tamaño conocido en compilación, vive en el stack.
- `&[u8]` — un slice, una vista a bytes que viven en otro lado, con tamaño conocido.

```rust
// Buffer dinámico — recorre bytes y decide si son imprimibles
let buffer: Vec<u8> = vec![72, 101, 108, 108, 111, 10, 0, 255];
for byte in &buffer {
    if *byte >= 32 && *byte <= 126 {
        println!("ASCII imprimible: {}", *byte as char);
    }
}

// Buffer fijo — llena de a bloques, como harías en C
let mut buf = [0u8; 4];
let mut usados = 0;
```

En el curso, los buffers fijos aparecen desde el primer ejemplo (`01_buffer.rs`, `02_buffer_fijo.rs`) y reaparecen constantemente: en la lectura binaria hexadecimal de `od` (`06_od.rs`), en la recepción de datos por socket (`01_cliente.rs`, `02_servidor.rs`), y en el servidor echo (`04_net_echo.rs`) donde un `[0u8; 512]` recibe y reenvía bytes en un loop.

Rust garantiza en compilación que no puedes acceder fuera de los límites de un slice, que no puedes usar un buffer después de moverlo, y que no puedes tener dos partes del código modificándolo al mismo tiempo.

### 2. Texto

Muchos de esos bytes resultan ser texto: líneas de log, rutas de archivo, comandos, configuraciones, respuestas HTTP. El programa necesita interpretar bytes como texto, manipularlo y producir texto nuevo.

En C, un string es un `char*` terminado en null. No sabes su longitud sin recorrerlo, no sabes si es UTF-8 válido, y funciones como `sprintf` y `strcat` son fuente constante de vulnerabilidades.

Rust separa el texto en dos tipos:

- `&str` — texto prestado, inmutable, garantizado UTF-8. Puede apuntar a un literal en el binario o a parte de un `String`.
- `String` — texto con dueño, vive en el heap, puede crecer y modificarse.

```rust
let modulo: &str = "net";                      // prestado, vive en el binario
let log = format!("[INFO] modulo={} fd={}", modulo, 3); // String, construido en runtime
```

No existe el null terminator. La longitud siempre es conocida. Y si intentas crear un `&str` con bytes que no son UTF-8, el compilador o el runtime te lo impiden.

En el curso, el texto aparece en el logging (`03_logging_str.rs`), en las utilidades Unix que procesan líneas (`cat`, `head`, `tail`, `wc`, `cut`), en el análisis de logs (`09_integrado.rs`), y en el parseo de peticiones HTTP del servidor web (`httpd.rs`) donde se separan método, ruta y versión con `split_whitespace()`.

### 3. Estado y manejo de errores

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

`match` te obliga a manejar todos los casos posibles. Si agregas una variante nueva, el código no compila hasta que la atiendas en cada `match`. No hay fall-through como en `switch` de C — cada brazo es independiente. Y `match` puede extraer datos de las variantes (destructuring), algo que `switch` no puede hacer.

Para operaciones que pueden fallar, Rust tiene `Result<T, E>` (éxito o error) y `Option<T>` (valor presente o ausente). No hay `-1` mágico ni `NULL` — el tipo mismo te dice que algo puede no estar ahí.

```rust
fn buscar_byte(buffer: &[u8], buscado: u8) -> Option<usize> {
    for (i, b) in buffer.iter().enumerate() {
        if *b == buscado { return Some(i); }
    }
    None
}
```

En el curso, `match` y `Result` aparecen en prácticamente todos los programas. El ejemplo integrador (`09_integrado.rs`) clasifica líneas de log con un `enum NivelLog` y `match`. La versión sin `unwrap` (`09_integrado_sin_unwrap.rs`) muestra cómo propagar errores con `?` en lugar de hacer panic. Las utilidades Unix usan `match` sobre `reader.lines()` para separar lectura exitosa de error. Y el servidor HTTP (`httpd.rs`) usa `match` para despachar métodos HTTP y manejar cada posible fallo.

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

En el curso, RAII aparece con archivos (`06_buffer_archivo.rs`), con conexiones TCP (`TcpStream` en los servidores y clientes de la parte 7), con procesos hijos (`Command::spawn()` en la parte 3), y con mutex guards (`MutexGuard` se libera al salir del scope, desbloqueando el candado automáticamente).

---

## Interacción con el sistema operativo

La programación de sistemas no es solo manipular datos en memoria — es hablar con el sistema operativo. El curso cubre las interfaces fundamentales que todo sistema Unix expone.

### Utilidades de línea de comandos

Las herramientas clásicas de Unix (`cat`, `head`, `tail`, `wc`, `cut`, `echo`, `od`) son programas de sistemas en su forma más pura: leen bytes de stdin o de un archivo, los procesan, y escriben el resultado en stdout. Son el ejemplo perfecto de cómo un programa interactúa con el sistema operativo a través de flujos de datos.

En el curso se implementan todas estas utilidades en Rust:

- `cat` — lee archivo o stdin línea por línea, usando generics (`T: BufRead`) para abstraer la fuente.
- `head` — lee las primeras N líneas con un contador y `break`.
- `tail` — mantiene las últimas N líneas en un `Vec<String>` como buffer FIFO.
- `wc` — cuenta líneas, palabras y bytes usando iteradores (`lines()`, `split_whitespace()`, `len()`).
- `cut` — extrae columnas separando por espacios con `split_whitespace()`.
- `od` — volcado hexadecimal con lectura binaria en bloques de 16 bytes.
- `echo` — imprime argumentos de línea de comandos.

Cada una de estas utilidades demuestra un patrón diferente de programación de sistemas: lectura con buffer, procesamiento línea por línea, manejo de argumentos, lectura binaria cruda, y la decisión entre stdin y archivo como fuente de datos.

### Procesos

En Unix, un proceso es la unidad fundamental de ejecución. Crear procesos hijos, conectarlos con pipes, y esperar su terminación es pan de cada día en programación de sistemas.

En C, esto es `fork()` + `exec()` + `wait()` + `pipe()`. En Rust, `Command` encapsula todo eso con una API más segura:

```rust
// Lanzar un proceso y esperar su resultado
let status = Command::new("ls").arg("-l").arg("/tmp").status()?;

// Conectar stdin/stdout con pipes
let mut child = Command::new("wc")
    .arg("-c")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;
child.stdin.as_mut().unwrap().write_all(b"hola rust\n")?;
let output = child.wait_with_output()?;
```

En el curso, los procesos aparecen desde la parte 3 (`01_proceso.rs`, `02_proceso_con_pipe.rs`) y se extienden en la parte 4 con pipelines de múltiples etapas: `echo hola | wc -c` y `cat archivo | grep foo | wc -l`. Estos pipelines son exactamente lo que hace el shell cuando escribes comandos encadenados con `|`.

### Señales

Las señales son el mecanismo que tiene Unix para notificar eventos a los procesos: Ctrl+C envía SIGINT, `kill` envía SIGTERM, un segfault genera SIGSEGV. Un programa de sistemas necesita capturar estas señales para hacer un apagado limpio (graceful shutdown) en lugar de morir abruptamente.

En el curso se muestran dos enfoques:

```rust
// Con AtomicBool: un flag compartido entre el manejador y el loop principal
let running = Arc::new(AtomicBool::new(true));
ctrlc::set_handler(move || { r.store(false, Ordering::SeqCst); }).unwrap();
while running.load(Ordering::SeqCst) {
    thread::sleep(Duration::from_millis(100));
}

// Con signal_hook + canal: el manejador envía un mensaje y el main espera
let (tx, rx) = mpsc::channel();
// hilo que escucha SIGINT y SIGTERM y envía () por el canal
rx.recv().unwrap(); // bloquea hasta recibir la señal
println!("Shutdown limpio");
```

Ambos patrones son fundamentales en servidores, daemons y cualquier programa que necesite liberar recursos antes de terminar.

---

## Concurrencia: hilos y sincronización

La concurrencia es donde la programación de sistemas se pone seria. Múltiples flujos de ejecución accediendo a los mismos datos es la fuente de los bugs más difíciles de encontrar y reproducir: data races, deadlocks, starvation.

### Hilos (threads)

Un hilo es un flujo de ejecución dentro del mismo proceso. Comparten memoria, lo cual es eficiente pero peligroso.

```rust
let h = thread::spawn(|| {
    println!("hilo secundario");
});
h.join().unwrap(); // esperar a que termine
```

La palabra clave `move` transfiere la propiedad de las variables capturadas al hilo. Esto es fundamental: el hilo puede vivir más que el scope donde nació, así que Rust exige que posea sus datos en vez de colgar de referencias potencialmente inválidas.

```rust
let saludo = String::from("hola desde el padre");
let h = thread::spawn(move || {
    println!("{saludo}"); // saludo ahora pertenece a este hilo
});
// println!("{saludo}"); // ERROR: saludo fue movido
```

En el curso, los hilos aparecen desde la parte 3 y se usan extensivamente: en el pipeline por canales (`03_pipeline_threads.rs`), en los servidores concurrentes (un hilo por conexión en `03_servidor_multihilo.rs` y `04_net_echo.rs`), y en todos los problemas clásicos de concurrencia de la parte 6.

### Primitivas de sincronización

Cuando múltiples hilos necesitan acceder al mismo dato, se necesitan mecanismos de sincronización. El curso cubre los cuatro fundamentales:

**Mutex** — exclusión mutua. Solo un hilo accede al dato a la vez. `lock()` adquiere el candado, y el `MutexGuard` lo libera automáticamente al salir del scope.

```rust
let contador = Arc::new(Mutex::new(0));
// en cada hilo:
let mut n = contador.lock().unwrap();
*n += 1;
// el candado se libera aquí (RAII)
```

**RwLock** — candado de lectura/escritura. Múltiples lectores simultáneos O un solo escritor exclusivo. Ideal cuando muchos leen y pocos escriben.

```rust
let dato = Arc::new(RwLock::new(10));
// lectores: dato.read().unwrap()  — pueden ser varios a la vez
// escritor: dato.write().unwrap() — exclusivo
```

**Condvar** — variable de condición. Permite que un hilo duerma sin consumir CPU hasta que otro lo despierte. Se usa siempre junto con un Mutex.

```rust
// consumidor espera:
while !*listo {
    listo = cvar.wait(listo).unwrap(); // libera el lock, duerme, re-adquiere al despertar
}
// productor señaliza:
*listo = true;
cvar.notify_one();
```

**Canales (mpsc)** — paso de mensajes. En lugar de compartir estado protegido por candados, los hilos se comunican enviando datos por un canal. El dato se mueve, no se comparte.

```rust
let (tx, rx) = mpsc::channel();
thread::spawn(move || { tx.send("hola").unwrap(); });
let msg = rx.recv().unwrap(); // bloquea hasta recibir
```

En el curso, los canales se usan para construir pipelines internos (`03_pipeline_threads.rs`), para el patrón productor-consumidor (`01_productor_consumidor.rs`), y para recibir señales del sistema operativo (`05_signals_hilos.rs`).

### Problemas clásicos de concurrencia

La parte 6 del curso implementa los problemas clásicos que todo programador de sistemas debe conocer:

**Productor-Consumidor** — un hilo genera datos, otro los procesa, con un buffer en medio. En C requiere mutex + dos semáforos. En Rust, `mpsc::channel()` resuelve todo: actúa como buffer, bloquea al consumidor cuando está vacío, y el dato se mueve eliminando condiciones de carrera.

**Lectores-Escritores** — muchos hilos leen un dato compartido, ocasionalmente uno lo modifica. `RwLock` encapsula toda la lógica que en C requiere mutex + contadores + semáforos.

**Filósofos Comensales** — 5 filósofos, 5 tenedores, cada uno necesita dos para comer. Demuestra el riesgo de deadlock: si todos toman el tenedor izquierdo al mismo tiempo, nadie puede tomar el derecho. La implementación del curso usa `Mutex` por tenedor y muestra explícitamente dónde puede ocurrir el deadlock.

**Barbero Dormilón** — una barbería con capacidad limitada, un barbero que duerme cuando no hay clientes, y clientes que se van si no hay silla. Combina `Mutex`, `Condvar` y `VecDeque` como cola FIFO. Es el ejemplo más complejo del curso y demuestra cómo coordinar múltiples condiciones: despertar al barbero, verificar capacidad, cerrar la barbería limpiamente.

---

## Programación de red

La red es donde todo lo anterior converge. Un servidor de red necesita buffers para recibir y enviar datos, texto para parsear protocolos, estado para manejar conexiones, handles para gestionar sockets, hilos para atender múltiples clientes, y sincronización para coordinar el acceso a recursos compartidos.

### Cliente y servidor TCP

En C: `socket()` → `bind()` → `listen()` → `accept()` → `read()` → `write()` → `close()`. En Rust, `TcpListener` y `TcpStream` encapsulan todo eso:

```rust
// Servidor: escuchar y aceptar conexiones
let listener = TcpListener::bind("127.0.0.1:7878")?;
for stream in listener.incoming() {
    // manejar cada conexión
}

// Cliente: conectar, enviar, recibir
let mut stream = TcpStream::connect("127.0.0.1:7878")?;
stream.write_all(b"Hola servidor\n")?;
let n = stream.read(&mut buffer)?;
```

El curso progresa desde un servidor secuencial (`02_servidor.rs`) que atiende un cliente a la vez, hasta un servidor concurrente (`03_servidor_multihilo.rs`) con un hilo por conexión, un servidor echo (`04_net_echo.rs`) que devuelve todo lo que recibe, y finalmente un servidor HTTP (`httpd.rs`) que parsea peticiones, resuelve rutas de forma segura contra directory traversal, y sirve archivos HTML estáticos.

### El servidor HTTP como ejemplo integrador

El servidor HTTP del curso (`httpd.rs`) es el ejemplo más completo de programación de sistemas porque integra todo:

- **Buffers**: `BufReader` para leer la petición línea por línea.
- **Texto**: parseo de la request line (`GET /index.html HTTP/1.1`) con `split_whitespace()`.
- **Estado**: `match` sobre el método HTTP para despachar GET, rechazar POST/PUT/DELETE, y manejar errores 400, 404, 403, 500, 501.
- **Handles**: `TcpStream` se cierra automáticamente, `File` para leer archivos del disco.
- **Seguridad**: validación de rutas contra directory traversal rechazando componentes `..`, rutas absolutas y prefijos de unidad.
- **Concurrencia**: un hilo por conexión con `thread::spawn`.

---

## Lo que Rust cambia respecto a C

Rust no cambia qué haces — sigues trabajando con bytes, texto, estado, recursos del sistema, procesos, hilos y sockets. Lo que cambia es cómo lo haces:

| Concepto | C | Rust |
|---|---|---|
| Buffer | `char buf[1024]`, sin verificación de límites | `[u8; 1024]` o `Vec<u8>`, verificado en compilación |
| Texto | `char*` con null terminator | `&str` / `String`, UTF-8 garantizado, longitud conocida |
| Estado | Enteros, `#define`, convenciones | `enum`, `Option`, `Result`, verificado por el compilador |
| Recursos | File descriptors enteros, `close()` manual | Tipos con ownership, liberación automática (RAII) |
| Errores | `-1`, `NULL`, `errno` | `Result<T, E>`, `Option<T>`, el compilador te obliga a manejarlos |
| Memoria | `malloc`/`free`, sin verificación | Ownership + borrowing, verificado en compilación |
| Procesos | `fork()` + `exec()` + `wait()` | `Command::new().spawn()`, pipes con `Stdio::piped()` |
| Hilos | `pthread_create`, data races posibles | `thread::spawn` + `move`, data races imposibles en compilación |
| Sincronización | `pthread_mutex_lock/unlock` manual | `Mutex`, `RwLock`, `Condvar` con RAII (se desbloquean solos) |
| Red | `socket()` + `bind()` + `listen()` + `close()` | `TcpListener::bind()`, `TcpStream`, cierre automático |

---

## Ownership y borrowing: la idea central

Todo lo anterior funciona gracias a una idea que no existe en C ni en la mayoría de lenguajes: cada valor en Rust tiene exactamente un dueño.

- Cuando el dueño sale de scope, el valor se libera.
- Puedes prestar el valor con `&` (referencia inmutable) o `&mut` (referencia mutable).
- No puedes tener una referencia mutable y otra inmutable al mismo tiempo.
- No puedes usar un valor después de moverlo.

Estas reglas se verifican en compilación. No hay garbage collector, no hay runtime. El costo es cero en ejecución. El precio es que el compilador te rechaza código que en C compilaría sin quejarse — pero que en C tendría bugs.

```rust
let saludo = String::from("hola");
let h = thread::spawn(move || {
    println!("{saludo}"); // saludo fue movido aquí
});
// println!("{saludo}"); // ERROR: ya no es tuyo
```

Este mecanismo es lo que hace posible que Rust prevenga data races en tiempo de compilación. Si un hilo es dueño de un dato, ningún otro hilo puede acceder a él. Si necesitas compartir, usas `Arc` (propiedad compartida atómica) + `Mutex` o `RwLock` (acceso sincronizado). El compilador te obliga a elegir explícitamente cómo compartir, y verifica que lo hagas correctamente.

---

## El recorrido del curso

El curso construye conocimiento de forma progresiva, cada parte apoyándose en la anterior:

1. **Fundamentos** — buffers, texto, enums, match, Option, Result, archivos, paths. Las piezas básicas con las que se construye todo lo demás.
2. **Utilidades Unix** — cat, echo, head, tail, cut, od, wc. Programas reales que procesan flujos de datos, leen argumentos y manejan errores.
3. **Procesos e hilos** — lanzar comandos, conectar pipes, crear hilos, transferir ownership con `move`.
4. **Pipelines y señales** — encadenar procesos con tuberías, construir pipelines internos con canales, capturar SIGINT/SIGTERM para apagado limpio.
5. **Sincronización** — Mutex, RwLock, Condvar, canales. Las herramientas para que múltiples hilos cooperen sin corromperse.
6. **Problemas clásicos** — productor-consumidor, lectores-escritores, filósofos comensales, barbero dormilón. Los problemas que todo programador de sistemas debe saber resolver.
7. **Red** — cliente TCP, servidor secuencial, servidor concurrente, servidor echo, servidor HTTP. La culminación donde todo converge.

---

## ¿Para quién es la programación de sistemas en Rust?

Para quien necesita control sobre la máquina sin sacrificar seguridad. Rust no es más fácil que C — en muchos sentidos es más exigente, porque el compilador te obliga a pensar en cosas que en C simplemente ignorabas. Pero los bugs que previene (use-after-free, buffer overflow, data races, deadlocks por RAII) son exactamente los que causan las vulnerabilidades más graves y los bugs más difíciles de encontrar en software de sistemas.

Rust te deja pensar a bajo nivel — bytes, memoria, procesos, hilos, sockets — pero con un compilador que actúa como un revisor de código implacable que nunca se cansa.
