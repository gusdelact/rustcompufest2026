# Comunicación entre procesos en Unix (IPC)

## El problema

En Unix, cada proceso tiene su propio espacio de memoria aislado. Un proceso no puede leer ni escribir la memoria de otro directamente. Esto es bueno para seguridad y estabilidad — si un proceso se muere, no corrompe a los demás.

Pero los procesos necesitan comunicarse. Un servidor web necesita hablar con la base de datos. Un shell necesita conectar la salida de `grep` con la entrada de `wc`. Un daemon necesita saber que le mandaron una señal de shutdown.

Unix ofrece varios mecanismos de IPC (Inter-Process Communication), cada uno con diferentes características.

---

## Los mecanismos

```mermaid
flowchart TB
    IPC["IPC en Unix"] --> DATOS["Transferencia de datos"]
    IPC --> SYNC["Sincronización"]
    IPC --> SIGNAL["Notificación"]

    DATOS --> PIPE["Pipes"]
    DATOS --> FIFO["Named Pipes\n(FIFO)"]
    DATOS --> SOCK["Sockets"]
    DATOS --> SHM["Memoria\ncompartida"]
    DATOS --> MQ["Colas de\nmensajes"]

    SYNC --> SEM["Semáforos"]
    SYNC --> FLOCK["File locks"]

    SIGNAL --> SIG["Señales"]

    style DATOS fill:#4a9eff,color:#fff
    style SYNC fill:#f39c12,color:#fff
    style SIGNAL fill:#e74c3c,color:#fff
```

---

## 1. Pipes (tuberías)

El mecanismo más simple. Un pipe es un buffer en el kernel con dos extremos: uno de escritura y uno de lectura. Los datos fluyen en una sola dirección.

```mermaid
flowchart LR
    P1["Proceso A\n(escribe)"] -->|"fd[1] → bytes"| KERNEL["Buffer del kernel\n(pipe)"]
    KERNEL -->|"bytes → fd[0]"| P2["Proceso B\n(lee)"]

    style KERNEL fill:#f39c12,color:#fff
```

Características:
- Unidireccional (un extremo escribe, el otro lee)
- Solo funciona entre procesos relacionados (padre-hijo)
- Los datos son un flujo de bytes sin estructura
- Se cierra automáticamente cuando el escritor termina (EOF)
- Capacidad limitada (~64KB en Linux) — si se llena, el escritor se bloquea

En la terminal:
```bash
cat archivo.txt | grep error | wc -l
```

En Rust:
```rust
use std::process::{Command, Stdio};

let mut cat = Command::new("cat")
    .arg("archivo.txt")
    .stdout(Stdio::piped())
    .spawn()?;

let output = Command::new("grep")
    .arg("error")
    .stdin(cat.stdout.take().unwrap())
    .output()?;
```

---

## 2. Named Pipes (FIFO)

Como un pipe, pero con nombre en el sistema de archivos. Cualquier proceso puede abrirlo, no solo procesos relacionados.

```bash
# Crear el FIFO
mkfifo /tmp/mi_canal

# Terminal 1: escribir
echo "hola" > /tmp/mi_canal

# Terminal 2: leer
cat /tmp/mi_canal
```

```mermaid
flowchart LR
    P1["Proceso A"] -->|"escribe"| FIFO["/tmp/mi_canal\n(archivo FIFO)"]
    FIFO -->|"lee"| P2["Proceso B"]

    style FIFO fill:#f39c12,color:#fff
```

Características:
- Persiste en el sistema de archivos (tiene una ruta)
- Cualquier proceso puede abrirlo (no necesitan ser padre-hijo)
- Sigue siendo unidireccional
- Se bloquea hasta que ambos extremos estén abiertos

---

## 3. Señales

Notificaciones asíncronas del kernel a un proceso. No llevan datos (solo un número de señal). Son el mecanismo más primitivo.

```mermaid
sequenceDiagram
    participant U as Usuario / Otro proceso
    participant K as Kernel
    participant P as Proceso

    U->>K: kill(pid, SIGTERM)
    K->>P: Entrega SIGTERM
    P->>P: Ejecuta handler
```

| Señal | Número | Significado |
|---|---|---|
| SIGINT | 2 | Ctrl+C — interrumpir |
| SIGTERM | 15 | Terminar limpiamente |
| SIGKILL | 9 | Matar (no se puede capturar) |
| SIGHUP | 1 | Terminal cerrada |
| SIGPIPE | 13 | Escribir a pipe sin lector |
| SIGUSR1/2 | 10/12 | Definidas por el usuario |

Limitaciones:
- No llevan datos (solo el número de señal)
- Pueden perderse si llegan varias del mismo tipo seguidas
- El handler se ejecuta en un contexto restringido (no puedes hacer mucho dentro)

---

## 4. Sockets Unix (Unix Domain Sockets)

Como sockets de red, pero locales. Bidireccionales, con soporte para streams y datagramas. El mecanismo más versátil para IPC local.

```mermaid
flowchart LR
    P1["Proceso A"] <-->|"bidireccional"| SOCK["/tmp/mi_socket\n(socket Unix)"]
    SOCK <-->|"bidireccional"| P2["Proceso B"]

    style SOCK fill:#2ecc71,color:#fff
```

Características:
- Bidireccional
- Soporta streams (como TCP) y datagramas (como UDP)
- Más rápido que sockets TCP/IP (no pasa por la pila de red)
- Tiene una ruta en el sistema de archivos
- Soporta pasar file descriptors entre procesos

Muchos servicios los usan: Docker (`/var/run/docker.sock`), PostgreSQL, systemd.

---

## 5. Memoria compartida

Dos o más procesos mapean la misma región de memoria física. Es el mecanismo más rápido porque no hay copia de datos — ambos procesos leen y escriben directamente en la misma memoria.

```mermaid
flowchart TB
    subgraph PA["Proceso A"]
        VA["Memoria virtual A"]
    end
    subgraph PB["Proceso B"]
        VB["Memoria virtual B"]
    end

    VA --> SHM["Memoria física\ncompartida"]
    VB --> SHM

    style SHM fill:#e74c3c,color:#fff
```

Características:
- El más rápido (cero copias)
- Requiere sincronización manual (semáforos o mutex)
- Peligroso si no se sincroniza correctamente
- Se crea con `shmget`/`shmat` (System V) o `shm_open`/`mmap` (POSIX)

---

## 6. Colas de mensajes

El kernel mantiene una cola donde los procesos depositan y retiran mensajes con estructura (tipo + datos).

```mermaid
flowchart LR
    P1["Proceso A"] -->|"msgsnd()"| COLA["Cola de mensajes\n(kernel)"]
    P2["Proceso B"] -->|"msgsnd()"| COLA
    COLA -->|"msgrcv()"| P3["Proceso C"]

    style COLA fill:#9b59b6,color:#fff
```

Características:
- Los mensajes tienen tipo y cuerpo
- Múltiples escritores y lectores
- Persisten hasta que se eliminan explícitamente
- Dos APIs: System V (`msgget`/`msgsnd`) y POSIX (`mq_open`/`mq_send`)

---

## 7. Semáforos

No transfieren datos — solo sincronizan. Un semáforo es un contador que los procesos incrementan y decrementan para coordinar acceso a recursos compartidos.

```mermaid
flowchart LR
    P1["Proceso A\nwait()"] --> SEM["Semáforo\n(contador)"]
    SEM --> P2["Proceso B\npost()"]

    style SEM fill:#f39c12,color:#fff
```

---

## Comparación

| Mecanismo | Dirección | Datos | Velocidad | Complejidad | Procesos relacionados |
|---|---|---|---|---|---|
| Pipe | Unidireccional | Stream de bytes | Rápido | Baja | Sí (padre-hijo) |
| Named Pipe | Unidireccional | Stream de bytes | Rápido | Baja | No |
| Señales | Unidireccional | Solo número | Muy rápido | Baja | No |
| Socket Unix | Bidireccional | Stream o datagramas | Rápido | Media | No |
| Memoria compartida | Bidireccional | Cualquier cosa | Más rápido | Alta | No |
| Cola de mensajes | Bidireccional | Mensajes tipados | Medio | Media | No |
| Semáforos | N/A (sincronización) | Ninguno | Rápido | Media | No |

---

## ¿Cuál usar?

```mermaid
flowchart TD
    A["¿Qué necesitas?"] --> B{"¿Solo notificar\nsin datos?"}
    B -->|Sí| SIG["Señales"]
    B -->|No| C{"¿Flujo simple\npadre → hijo?"}
    C -->|Sí| PIPE["Pipe"]
    C -->|No| D{"¿Comunicación\nbidireccional?"}
    D -->|Sí| SOCK["Socket Unix"]
    D -->|No| E{"¿Máxima\nvelocidad?"}
    E -->|Sí| SHM["Memoria compartida\n+ semáforos"]
    E -->|No| MQ["Cola de mensajes"]

    style SIG fill:#e74c3c,color:#fff
    style PIPE fill:#4a9eff,color:#fff
    style SOCK fill:#2ecc71,color:#fff
    style SHM fill:#f39c12,color:#fff
    style MQ fill:#9b59b6,color:#fff
```

---

## IPC en Rust

Rust no reinventa estos mecanismos — los usa a través de la biblioteca estándar y crates:

| Mecanismo Unix | En Rust |
|---|---|
| Pipe | `Command` + `Stdio::piped()` |
| Señales | Crates `ctrlc`, `signal-hook` |
| Sockets Unix | `std::os::unix::net::UnixStream` |
| Memoria compartida | Crate `shared_memory` |
| Semáforos | Crate `posix-semaphore` |

Dentro del mismo proceso, Rust ofrece alternativas propias que son más seguras:
- `mpsc::channel()` en lugar de pipes entre hilos
- `Arc<Mutex<T>>` en lugar de memoria compartida con semáforos
- `Condvar` en lugar de señales entre hilos

La ventaja de las primitivas de Rust es que el compilador verifica la seguridad en tiempo de compilación. Con IPC de Unix, la responsabilidad es del programador.
