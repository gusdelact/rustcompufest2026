/*
El Barbero Dormilón (Sleeping Barber)

🧩 El problema
Una barbería con:
- Un barbero que atiende clientes uno por uno.
- Una sala de espera con un número limitado de sillas.

Reglas:
- Si no hay clientes, el barbero se duerme.
- Si llega un cliente y hay silla disponible, se sienta y despierta al barbero.
- Si no hay silla, el cliente se va.
- El barbero atiende en orden de llegada (FIFO).

⚠️ Problemas típicos
- Sincronización de llegada: el cliente llega justo cuando el barbero
  se está durmiendo → se pierden la notificación.
- Despertar al barbero sin condiciones de carrera.
- Cierre limpio: el barbero debe terminar cuando la barbería cierra
  y no quedan clientes.

La solución usa Condvar (variable de condición) para que el barbero
duerma sin consumir CPU y se despierte solo cuando un cliente lo notifica.
*/

// VecDeque: cola de doble extremo, la usamos como cola FIFO para los clientes.
use std::collections::VecDeque;
// Arc: compartir ownership entre hilos.
// Condvar: variable de condición — permite que un hilo duerma hasta que otro lo despierte.
// Mutex: protege el estado compartido de acceso concurrente.
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

/// Representa un cliente con un identificador único.
/// `Debug` permite imprimir con `{:?}`.
/// `Clone` permite clonar el struct (necesario para algunas operaciones).
#[derive(Debug, Clone)]
struct Cliente {
    id: usize,
}

/// Estado compartido de la barbería.
/// Protegido por un `Mutex` y acompañado de una `Condvar`.
#[derive(Debug)]
struct EstadoBarberia {
    /// Cola FIFO de clientes esperando. El barbero atiende al primero.
    sala_espera: VecDeque<Cliente>,
    /// Número máximo de sillas en la sala de espera.
    capacidad: usize,
    /// Indica si la barbería sigue abierta. Cuando se pone en `false`,
    /// el barbero termina después de atender a los clientes restantes.
    abierta: bool,
}

/// Punto de entrada del programa.
///
/// Crea la barbería, lanza el hilo del barbero, genera 10 clientes
/// que llegan escalonados, y al final cierra la barbería.
fn main() {
    // Inicializamos el estado de la barbería:
    // - Sala de espera vacía.
    // - Capacidad de 3 sillas.
    // - Barbería abierta.
    let estado = EstadoBarberia {
        sala_espera: VecDeque::new(),
        capacidad: 3,
        abierta: true,
    };

    // Envolvemos el estado en Arc<(Mutex<EstadoBarberia>, Condvar)>.
    // La tupla agrupa el Mutex y la Condvar porque siempre se usan juntos:
    // - El Mutex protege el estado.
    // - La Condvar permite al barbero dormir y ser despertado por clientes.
    let barberia = Arc::new((Mutex::new(estado), Condvar::new()));

    // --- Hilo del barbero ---
    // Clonamos el Arc para que el hilo del barbero tenga su propia referencia.
    let barberia_barbero = Arc::clone(&barberia);
    let hilo_barbero = thread::spawn(move || {
        barbero(barberia_barbero);
    });

    // --- Hilos de clientes ---
    // Generamos 10 clientes (id 1 a 10) que llegan escalonados.
    let mut handles_clientes = Vec::new();

    for id in 1..=10 {
        let barberia_cliente = Arc::clone(&barberia);

        let h = thread::spawn(move || {
            // Cada cliente llega con un retraso proporcional a su id.
            // Cliente 1 llega a los 300ms, cliente 2 a los 600ms, etc.
            // Esto simula llegadas escalonadas en el tiempo.
            thread::sleep(Duration::from_millis((id as u64) * 300));
            cliente(barberia_cliente, id);
        });

        handles_clientes.push(h);
    }

    // Esperamos a que todos los clientes hayan llegado (o se hayan ido).
    for h in handles_clientes {
        h.join().unwrap();
    }

    // Damos tiempo al barbero para terminar de atender a los que quedaron.
    thread::sleep(Duration::from_secs(5));

    // --- Cerrar la barbería ---
    // Adquirimos el lock, cambiamos `abierta` a false, y notificamos al barbero.
    // El bloque `{}` limita el scope del lock para liberarlo inmediatamente.
    {
        let (lock, cvar) = &*barberia;
        let mut estado = lock.lock().unwrap();
        estado.abierta = false;
        // Despertamos al barbero por si estaba dormido esperando clientes.
        // Sin esto, el barbero se quedaría dormido para siempre.
        cvar.notify_one();
    }
    // El lock se libera aquí (al salir del bloque).

    // Esperamos a que el hilo del barbero termine.
    hilo_barbero.join().unwrap();
}

/// Función que ejecuta el hilo del barbero.
///
/// El barbero entra en un loop infinito:
/// 1. Si no hay clientes y la barbería está abierta, se duerme con `cvar.wait()`.
/// 2. Cuando lo despiertan, toma al siguiente cliente de la cola.
/// 3. Lo atiende durante 2 segundos (simulado).
/// 4. Si la barbería cerró y no quedan clientes, termina.
///
/// # Argumentos
/// * `barberia` - Referencia compartida al estado de la barbería y su Condvar.
fn barbero(barberia: Arc<(Mutex<EstadoBarberia>, Condvar)>) {
    loop {
        // Bloque para adquirir el lock, verificar el estado, y obtener un cliente.
        // El lock se libera al final del bloque, ANTES de atender al cliente.
        // Esto es importante: no queremos mantener el lock durante los 2 segundos
        // de corte, porque bloquearía a los clientes que intentan entrar.
        let cliente_actual = {
            // Destructuramos la tupla: `lock` es el Mutex, `cvar` es la Condvar.
            let (lock, cvar) = &*barberia;

            // Adquirimos el lock del estado.
            let mut estado = lock.lock().unwrap();

            // Mientras no haya clientes Y la barbería siga abierta, dormimos.
            // `cvar.wait(estado)` hace tres cosas atómicamente:
            //   1. Libera el lock del Mutex (para que los clientes puedan entrar).
            //   2. Duerme el hilo (sin consumir CPU).
            //   3. Cuando lo despiertan con `notify_one()`, re-adquiere el lock.
            // El `while` es necesario porque `wait()` puede tener "spurious wakeups"
            // (despertares falsos) — el hilo puede despertar sin que nadie lo notificara.
            while estado.sala_espera.is_empty() && estado.abierta {
                println!("Barbero: no hay clientes, me duermo.");
                estado = cvar.wait(estado).unwrap();
            }

            // Si la barbería cerró y no quedan clientes, terminamos el loop.
            if !estado.abierta && estado.sala_espera.is_empty() {
                println!("Barbero: la barbería cerró, termino mi jornada.");
                return; // Sale de la función, terminando el hilo.
            }

            // Tomamos al siguiente cliente de la cola (FIFO).
            // `pop_front()` retorna `Option<Cliente>`: Some si había alguien, None si no.
            estado.sala_espera.pop_front()
            // El lock se libera aquí al salir del bloque.
        };

        // Atendemos al cliente FUERA del lock.
        // Esto permite que otros clientes entren a la sala de espera
        // mientras el barbero está cortando el pelo.
        if let Some(cliente) = cliente_actual {
            println!("Barbero: atendiendo al cliente {}", cliente.id);
            // Simulamos el tiempo de corte de pelo (2 segundos).
            thread::sleep(Duration::from_secs(2));
            println!("Barbero: terminé con el cliente {}", cliente.id);
        }
    }
}

/// Función que ejecuta la llegada de un cliente.
///
/// El cliente intenta sentarse en la sala de espera:
/// - Si hay silla disponible, se sienta y despierta al barbero.
/// - Si no hay silla, se va.
///
/// # Argumentos
/// * `barberia` - Referencia compartida al estado de la barbería y su Condvar.
/// * `id` - Identificador único del cliente.
fn cliente(barberia: Arc<(Mutex<EstadoBarberia>, Condvar)>, id: usize) {
    let cliente = Cliente { id };

    // Destructuramos la tupla para acceder al Mutex y la Condvar.
    let (lock, cvar) = &*barberia;

    // Adquirimos el lock para verificar y modificar el estado.
    let mut estado = lock.lock().unwrap();

    // Verificamos si hay silla disponible.
    if estado.sala_espera.len() < estado.capacidad {
        println!("Cliente {}: entra y espera.", id);
        // Agregamos al cliente al final de la cola (FIFO).
        estado.sala_espera.push_back(cliente);

        // Despertamos al barbero por si estaba dormido.
        // `notify_one()` despierta a un hilo que esté en `cvar.wait()`.
        // Si el barbero no estaba dormido (ya estaba atendiendo),
        // la notificación se pierde — pero no importa, porque el barbero
        // revisará la cola cuando termine con el cliente actual.
        cvar.notify_one();
    } else {
        // No hay sillas disponibles. El cliente se va sin ser atendido.
        println!("Cliente {}: no hay sillas, me voy.", id);
    }
    // El lock se libera aquí al salir de la función.
}
