/*
Buffer de bytes — versión funcional

Compara con parte1/01_buffer.rs que usa for + if.
Aquí usamos .iter(), .for_each(), y un match dentro del closure.

Imperativo (parte1):
    for byte in &buffer {
        if *byte >= 32 && *byte <= 126 {
            println!("ASCII imprimible: {}", *byte as char);
        } else {
            println!("Byte de control: {}", byte);
        }
    }

Funcional (este archivo):
    buffer.iter().for_each(|&b| match b { ... });
*/

fn main() {
    let buffer: Vec<u8> = vec![72, 101, 108, 108, 111, 10, 0, 255];

    println!("Buffer completo: {:?}", buffer);

    // `.iter()` crea un iterador sobre referencias (&u8).
    // `.for_each()` ejecuta un closure por cada elemento.
    // `|&b|` destructura la referencia directamente, así `b` es `u8` (no `&u8`).
    // Esto elimina la necesidad de `*byte` para desreferenciar.
    buffer.iter().for_each(|&b| match b {
        // Rango de ASCII imprimible (32 = espacio, 126 = '~').
        // `b @ 32..=126` captura el valor en `b` y verifica el rango.
        // Aunque `b` ya existe, `@` es útil si quisiéramos un nombre diferente.
        32..=126 => println!("ASCII imprimible: {}", b as char),
        _ => println!("Byte de control o no imprimible: {}", b),
    });

    // Bonus: separar imprimibles y no imprimibles con partition.
    // `.partition()` divide un iterador en dos colecciones según un predicado.
    let (imprimibles, control): (Vec<u8>, Vec<u8>) =
        buffer.iter().partition(|&&b| (32..=126).contains(&b));

    println!("\nImprimibles: {:?}", imprimibles);
    println!("Control: {:?}", control);

    // Bonus: convertir los bytes imprimibles a un String de una vez.
    // `.filter()` descarta los que no cumplen el predicado.
    // `.map()` transforma cada byte en un char.
    // `.collect()` recopila los chars en un String.
    let texto: String = buffer
        .iter()
        .filter(|&&b| (32..=126).contains(&b))
        .map(|&b| b as char)
        .collect();

    println!("Como texto: {}", texto);
}
