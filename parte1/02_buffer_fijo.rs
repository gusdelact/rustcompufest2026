/* Buffer fijo de trabajo + variable de control
[u8; 4] es un buffer fijo, muy cercano a C.
mut aparece porque el buffer y el contador cambian.
usados sustituye la típica variable de control manual en C.
Rust sigue dejando pensar “a bajo nivel”, pero sin perder claridad.
*/
fn imprimir(buffer: &[u8; 4]) -> () {
   let texto = String::from_utf8_lossy(buffer);
    println!("{}", texto);
}

// Definimos la función principal del programa
fn main() {
    // Entrada de bytes simulada: representa el texto "abc\ndef\n"
    let input = b"abc\ndef\n";

    // Creamos un buffer de 4 bytes inicializado en ceros
    let mut buf = [0u8; 4];

    // Contador que lleva la cantidad de posiciones usadas en el buffer
    let mut usados = 0;

    // Recorremos cada byte de la entrada uno por uno
    for b in input {
        // Si todavía hay espacio en el buffer
        if usados < buf.len() {
            // Guardamos el byte actual en la posición correspondiente del buffer
            buf[usados] = *b;
            // Avanzamos el contador de posiciones usadas
            usados += 1;
        } else {
            // El buffer está lleno, así que imprimimos su contenido
            println!("Buffer lleno: {:?}", &buf);
            // Reiniciamos el contador para empezar a llenar el buffer desde el inicio
            usados = 0;
            // Guardamos el byte actual en la primera posición del buffer
            buf[usados] = *b;
            // Avanzamos el contador
            usados += 1;
        }
    }

    // Al terminar el ciclo, si quedaron bytes sin imprimir en el buffer
    if usados > 0 {
        // Imprimimos solo la porción del buffer que fue utilizada
        println!("Contenido final parcial: {:?}", &buf[..usados]); //uf[..usados] significa: desde el inicio hasta usados (sin incluirlo).
        imprimir(&buf)
    }
}