/*
&str es texto prestado.
String es texto con dueño.
format! construye una String.
para logging, String y &str reemplazan mucho del uso inseguro de sprintf, strcat, etc.
*/

/// Punto de entrada principal del programa.
/// Construye una línea de log con información del módulo, descriptor de archivo y evento,
/// y la imprime solo si supera una longitud mínima.
fn main() {
    // Nombre del módulo que genera el log
    /*
"net" es un string literal.
 Rust lo guarda directamente en el binario compilado (en memoria de solo lectura). 
 modulo no es dueño de ese texto, 
 solo es un puntero que dice "el texto está allá y mide 3 bytes".

 La distinción importante en Rust:

&str → texto prestado, no se puede modificar ni crecer. Barato, solo una referencia.
String → texto con dueño, vive en el heap, se puede modificar y crecer.

Internamente &str es lo mismo que un slice &[u8]: un apuntador . 
La diferencia es que &str garantiza que el contenido es UTF-8 válido.
    */
    let modulo: &str = "net";
    // Descripción del evento ocurrido
    let evento: &str = "socket abierto";
    // Descriptor de archivo asociado al evento
    let fd = 3;

    // Formatea la línea de log con nivel INFO incluyendo módulo, fd y evento
    let log_line = format!("[INFO] modulo={} fd={} evento={}", modulo, fd, evento);

    // Verifica que la línea tenga una longitud suficiente antes de imprimirla
    if log_line.len() > 20 {
        println!("{}", log_line);
    } else {
        // Advierte si el mensaje generado es demasiado corto
        println!("[WARN] mensaje muy corto");
    }
}
