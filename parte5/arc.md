# Arc — Compartir datos entre hilos en Rust

## El problema

En Rust, cada valor tiene un solo dueño. Cuando creas un hilo con `thread::spawn`, el closure necesita ser dueño de los datos que usa (por eso usamos `move`). Pero ¿qué pasa si dos o más hilos necesitan acceder al mismo dato?

```rust
let contador = Mutex::new(0);

// ERROR: no puedes mover `contador` a dos hilos
thread::spawn(move || { /* usa contador */ });
thread::spawn(move || { /* usa contador */ }); // ya fue movido arriba
```

No puedes mover el mismo valor a dos hilos. Necesitas una forma de tener múltiples dueños.

---

## ¿Qué es Arc?

`Arc` significa **Atomic Reference Counted**. Es un puntero inteligente que permite que múltiples partes del código sean dueñas del mismo dato, de forma segura entre hilos.

```rust
use std::sync::Arc;

let dato = Arc::new(42);
let copia = Arc::clone(&dato); // no copia el 42, solo incrementa el contador
```

```mermaid
flowchart TB
    ARC1["Arc (hilo 1)"] --> DATO["Dato real\n(en el heap)\ncontador de refs = 3"]
    ARC2["Arc (hilo 2)"] --> DATO
    ARC3["Arc (hilo 3)"] --> DATO

    style DATO fill:#f39c12,color:#fff
    style ARC1 fill:#4a9eff,color:#fff
    style ARC2 fill:#4a9eff,color:#fff
    style ARC3 fill:#4a9eff,color:#fff
```

Internamente, `Arc` mantiene un contador atómico de cuántas referencias existen. Cada `Arc::clone()` incrementa el contador. Cuando un `Arc` se destruye (sale de scope), el contador decrementa. Cuando llega a cero, el dato se libera.

"Atómico" significa que el contador se actualiza de forma segura entre hilos, sin necesidad de un lock.

---

## Arc::clone no copia el dato

Esto es importante: `Arc::clone(&dato)` no duplica el dato interno. Solo crea otro puntero al mismo dato e incrementa el contador de referencias. Es una operación barata.

```rust
let original = Arc::new(vec![1, 2, 3]); // el vector vive en el heap
let clon = Arc::clone(&original);       // otro puntero al MISMO vector

// original y clon apuntan a la misma memoria
```

```mermaid
flowchart LR
    subgraph antes["Arc::clone()"]
        direction TB
        O["original\n(ref count: 1)"] --> V["vec![1, 2, 3]"]
    end

    subgraph despues["Después"]
        direction TB
        O2["original"] --> V2["vec![1, 2, 3]\n(ref count: 2)"]
        C["clon"] --> V2
    end

    antes -->|"clone()"| despues

    style V fill:#2ecc71,color:#fff
    style V2 fill:#2ecc71,color:#fff
```

---

## Arc solo da acceso de lectura

`Arc` por sí solo solo permite leer el dato. No puedes modificarlo. Para modificar, necesitas combinarlo con un mecanismo de sincronización:

| Combinación | Uso |
|---|---|
| `Arc<Mutex<T>>` | Un hilo a la vez lee o escribe |
| `Arc<RwLock<T>>` | Muchos leen, uno escribe |
| `Arc<AtomicBool>` | Booleano compartido sin lock |
| `Arc<(Mutex<T>, Condvar)>` | Esperar una condición entre hilos |

```mermaid
flowchart TD
    ARC["Arc&lt;T&gt;"] --> Q{"¿Necesitas\nmodificar T?"}
    Q -->|No| READ["Usa Arc solo\n(lectura compartida)"]
    Q -->|Sí| Q2{"¿Patrón de acceso?"}
    Q2 -->|"Todos leen y escriben"| MUTEX["Arc&lt;Mutex&lt;T&gt;&gt;"]
    Q2 -->|"Muchos leen, pocos escriben"| RWLOCK["Arc&lt;RwLock&lt;T&gt;&gt;"]
    Q2 -->|"Solo un bool o entero"| ATOMIC["Arc&lt;AtomicBool&gt;\nArc&lt;AtomicI32&gt;"]

    style MUTEX fill:#e74c3c,color:#fff
    style RWLOCK fill:#f39c12,color:#fff
    style ATOMIC fill:#2ecc71,color:#fff
```

---

## Ejemplo: contador compartido con Arc + Mutex

Del archivo `01_mutex.rs`:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let contador = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let contador = Arc::clone(&contador); // clonar Arc, no el dato
        let h = thread::spawn(move || {
            let mut n = contador.lock().unwrap();
            *n += 1;
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("contador = {}", *contador.lock().unwrap()); // 10
}
```

```mermaid
sequenceDiagram
    participant Main as Hilo principal
    participant H1 as Hilo 1
    participant H2 as Hilo 2
    participant M as Mutex(0)

    Main->>Main: Arc::new(Mutex::new(0))
    Main->>H1: spawn(Arc::clone)
    Main->>H2: spawn(Arc::clone)
    H1->>M: lock() → n=0
    H1->>M: *n += 1 → n=1
    H1->>M: unlock (drop)
    H2->>M: lock() → n=1
    H2->>M: *n += 1 → n=2
    H2->>M: unlock (drop)
    Main->>M: lock() → lee 2
```

El flujo:
1. `Arc::new(Mutex::new(0))` crea el dato en el heap con ref count = 1
2. Cada `Arc::clone()` incrementa el ref count (no copia el dato)
3. `move` transfiere el Arc clonado al hilo
4. Dentro del hilo, `lock()` adquiere el mutex
5. Al salir del scope, el `MutexGuard` se destruye y libera el lock
6. Cuando el hilo termina, su Arc se destruye y el ref count decrementa
7. Al final, solo queda el Arc del hilo principal (ref count = 1)

---

## Arc vs Rc

Rust tiene dos punteros con conteo de referencias:

| | `Rc<T>` | `Arc<T>` |
|---|---|---|
| Módulo | `std::rc::Rc` | `std::sync::Arc` |
| Seguro entre hilos | No | Sí |
| Contador | Normal (no atómico) | Atómico |
| Costo | Más barato | Ligeramente más caro |
| Cuándo usarlo | Un solo hilo | Múltiples hilos |

`Rc` es más rápido porque no necesita operaciones atómicas, pero el compilador te impide usarlo entre hilos. Si intentas pasar un `Rc` a `thread::spawn`, obtienes un error de compilación.

---

## Ciclo de vida del ref count

```mermaid
flowchart TD
    A["Arc::new(dato)\nref count = 1"] --> B["Arc::clone()\nref count = 2"]
    B --> C["Arc::clone()\nref count = 3"]
    C --> D["Hilo 1 termina\ndrop → ref count = 2"]
    D --> E["Hilo 2 termina\ndrop → ref count = 1"]
    E --> F["Hilo principal termina\ndrop → ref count = 0"]
    F --> G["Dato se libera\n(drop del valor interno)"]

    style A fill:#2ecc71,color:#fff
    style G fill:#e74c3c,color:#fff
```

No hay garbage collector. No hay `free()` manual. El dato se libera exactamente cuando el último Arc desaparece.

---

## Resumen

- `Arc` permite múltiples dueños del mismo dato entre hilos.
- `Arc::clone()` es barato — solo incrementa un contador, no copia el dato.
- `Arc` solo da lectura. Para escribir, combínalo con `Mutex`, `RwLock`, o tipos atómicos.
- El dato se libera automáticamente cuando el último `Arc` se destruye.
- Usa `Rc` si estás en un solo hilo, `Arc` si necesitas cruzar hilos.
