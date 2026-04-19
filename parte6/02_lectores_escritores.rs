/*
Lectores–Escritores (Readers–Writers)

🧩 El problema
Muchos hilos necesitan leer un dato compartido, pero ocasionalmente
uno necesita escribirlo. Las reglas:
- Múltiples lectores pueden leer al mismo tiempo (no se estorban).
- Solo un escritor puede escribir a la vez (acceso exclusivo).
- Nadie puede leer mientras se escribe.

⚠️ Problemas típicos
- Starvation (hambre): si siempre hay lectores, el escritor nunca accede.
- Prioridad: ¿quién va primero, lectores o escritores?

En C, esto se implementa con mutex + contadores de lectores + semáforos.
En Rust, `RwLock` encapsula toda esa lógica.
*/

// Arc (Atomic Reference Counted): permite que múltiples hilos sean dueños
// del mismo dato. Cada `Arc::clone()` incrementa un contador atómico.
// RwLock (Read-Write Lock): candado que permite múltiples lectores
// simultáneos O un solo escritor exclusivo.
use std::sync::{Arc, RwLock};
// thread para crear hilos de ejecución.
use std::thread;

/// Punto de entrada del programa.
///
/// Crea un dato compartido (`i32` con valor inicial 5) protegido por
/// `Arc<RwLock<i32>>`. Un hilo lector lee el valor y un hilo escritor
/// lo incrementa en 1. El orden de ejecución no es determinista.
fn main() {
    // Creamos el dato compartido:
    // - `RwLock::new(5)`: protege el valor 5 con un candado de lectura/escritura.
    // - `Arc::new(...)`: envuelve el RwLock para poder compartirlo entre hilos.
    let data = Arc::new(RwLock::new(5));

    // --- Hilo lector ---
    let r = {
        // Clonamos el Arc para que este hilo tenga su propia referencia.
        // Esto incrementa el contador de referencias, no copia el dato.
        let data = Arc::clone(&data);

        thread::spawn(move || {
            // `data.read()` adquiere un lock de lectura (shared lock).
            // Múltiples lectores pueden mantener este lock al mismo tiempo.
            // `.unwrap()` hace panic si el lock está "envenenado"
            // (otro hilo hizo panic mientras tenía el lock).
            // `val` es un `RwLockReadGuard<i32>` — un guard que se libera
            // automáticamente al salir del scope.
            let val = data.read().unwrap();

            // `*val` desreferencia el guard para acceder al valor `i32`.
            // El lector puede ver 5 (si lee antes del escritor) o 6 (si lee después).
            println!("lector: {}", *val);
            // `val` sale de scope aquí → el lock de lectura se libera.
        })
    };

    // --- Hilo escritor ---
    let w = {
        let data = Arc::clone(&data);

        thread::spawn(move || {
            // `data.write()` adquiere un lock de escritura (exclusive lock).
            // Solo un escritor puede tener este lock a la vez.
            // Mientras el escritor tiene el lock, ningún lector puede leer.
            // `val` es un `RwLockWriteGuard<i32>` — permite modificar el dato.
            let mut val = data.write().unwrap();

            // Incrementamos el valor de 5 a 6.
            *val += 1;
            // `val` sale de scope aquí → el lock de escritura se libera.
        })
    };

    // Esperamos a que ambos hilos terminen.
    // El orden de ejecución entre lector y escritor no es determinista.
    // Posibles salidas:
    //   "lector: 5" (leyó antes de la escritura)
    //   "lector: 6" (leyó después de la escritura)
    r.join().unwrap();
    w.join().unwrap();
}
