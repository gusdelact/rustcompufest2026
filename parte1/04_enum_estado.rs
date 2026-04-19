/* Estado del sistema con enum + match
en C muchas veces usarías enteros, #define, banderas, o convenciones informales.
en Rust declaras los estados posibles en el tipo mismo.
match te obliga a atender todos los casos.
*/


// Enum que representa los posibles estados de un descriptor de archivo (file descriptor)
enum EstadoFd {
    Abierto,          // El descriptor está abierto y disponible para uso
    Cerrado,          // El descriptor ha sido cerrado
    Error(i32),       // El descriptor tiene un error, con su código asociado
}

// Función que recibe un estado de descriptor de archivo e imprime su descripción
fn describir_estado(estado: EstadoFd) {
    // Usamos match para manejar cada variante del enum
    match estado {
        EstadoFd::Abierto => println!("El descriptor está abierto"),
        EstadoFd::Cerrado => println!("El descriptor está cerrado"),
        // Extraemos el código de error contenido en la variante Error
        EstadoFd::Error(codigo) => println!("Error del sistema: {}", codigo),
    }
}

fn main() {
    // Creamos dos estados de ejemplo
    let e1 = EstadoFd::Abierto;
    let e2 = EstadoFd::Error(13); // Código 13: permiso denegado (EACCES en POSIX)
    let e3 = EstadoFd::Cerrado;
    // Describimos cada estado por consola
    describir_estado(e1);
    describir_estado(e2);
    describir_estado(e3);
}


/*
match se parece superficialmente a switch, pero hay diferencias fundamentales:

1. Es exhaustivo — el compilador te obliga a cubrir todos los casos. Si agregas una variante nueva al enum (ej. Timeout), tu código no compila hasta que la manejes. Un switch en C/Java simplemente cae al default o no hace nada.

2. Puede extraer datos — mira esta línea:

EstadoFd::Error(codigo) => println!("Error del sistema: {}", codigo),
No solo compara si es Error, sino que saca el valor de adentro y lo asigna a codigo. Esto se llama destructuring. Un switch solo compara valores planos.

3. No hay fall-through — en C, si olvidas el break, la ejecución cae al siguiente caso. En match cada brazo es independiente, no necesitas break.

4. Puede hacer pattern matching complejo — puedes matchear rangos, tuplas, structs, condiciones:

match valor {
    0..=9 => println!("un dígito"),
    n if n < 0 => println!("negativo"),
    _ => println!("otro"),
}
5. Es una expresión — devuelve un valor:

let msg = match estado {
    EstadoFd::Abierto => "abierto",
    EstadoFd::Cerrado => "cerrado",
    EstadoFd::Error(_) => "error",
};
Un switch es un statement, no devuelve nada.
*/