/*
Ilustrar move de forma clara

move significa:

“La closure se queda con la propiedad de lo que captura.”

No está tomando prestado saludo; se lo lleva. Eso encaja muy bien con la naturaleza de un hilo: el hilo nuevo puede vivir más que el scope donde nació, así que Rust prefiere que posea sus datos en vez de colgar de referencias potencialmente inválidas.
*/

use std::thread;

/// Ejemplo que demuestra cómo `move` transfiere la propiedad (ownership)
/// de variables capturadas hacia un hilo (thread).
///
/// Conceptos clave:
/// - `thread::spawn` crea un nuevo hilo de ejecución.
/// - La palabra clave `move` antes del closure fuerza a que el closure
///   tome posesión (ownership) de las variables que captura, en lugar de
///   tomarlas prestadas (borrow).
/// - Una vez movida la variable al hilo, el hilo padre ya no puede usarla.
/// - `join()` bloquea el hilo actual hasta que el hilo hijo termine,
///   devolviendo un `Result` que podemos desenvolver con `unwrap()`.
fn main() {
    // Creamos un String en el hilo principal.
    // String no implementa Copy, así que al moverlo se transfiere la propiedad.
    let saludo = String::from("hola desde el padre");

    // `thread::spawn` recibe un closure que se ejecutará en un nuevo hilo.
    // `move` transfiere la propiedad de `saludo` al closure del hilo hijo.
    // Esto es necesario porque Rust no puede garantizar que la referencia
    // a `saludo` siga siendo válida durante toda la vida del hilo hijo;
    // el hilo padre podría terminar antes y liberar la memoria.
    let h = thread::spawn(move || {
        // Aquí `saludo` ya pertenece a este hilo; podemos usarlo con seguridad.
        println!("hilo dice: {saludo}");
        // Al finalizar el closure, `saludo` se libera (drop) aquí.
    });

    // `join()` bloquea el hilo principal hasta que el hilo `h` termine.
    // Retorna `Result<T, Box<dyn Any>>`: Ok si el hilo terminó bien,
    // Err si el hilo entró en pánico (panic).
    h.join().unwrap();

    // println!("{saludo}");
    // ^ Esto ya no compila porque `saludo` fue movido al closure del hilo.
    //   El compilador emite el error:
    //     "value used here after move"
    //   Esto es parte del sistema de ownership de Rust: previene data races
    //   en tiempo de compilación al asegurar que solo un propietario
    //   puede acceder al dato a la vez.
}