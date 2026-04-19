/* Buffer de bytes + for + if
Vec<u8> es un buffer dinámico.
for recorre los bytes.
if decide si un byte representa un carácter imprimible.
&buffer presta el buffer para lectura, no lo consume.
*/

fn main() {
    // Creamos un buffer (vector de bytes u8) con valores específicos:
    // 72='H', 101='e', 108='l', 108='l', 111='o', 10='\n' (salto de línea), 0=NULL, 255=ÿ
    let buffer: Vec<u8> = vec![72, 101, 108, 108, 111, 10, 0, 255];

    // Imprimimos el buffer completo en formato de depuración (muestra cada byte como número)
    println!("Buffer completo: {:?}", buffer);

    // Iteramos sobre cada byte del buffer por referencia
    for byte in &buffer {
        // Verificamos si el byte está en el rango ASCII imprimible (32 = espacio, 126 = '~')
        // Los caracteres imprimibles van del 32 al 126 en la tabla ASCII
        if *byte >= 32 && *byte <= 126 {
            // Si es imprimible, lo convertimos a char con `as char` y lo mostramos
            println!("ASCII imprimible: {}", *byte as char);
        } else {
            // Si está fuera del rango (0-31 son caracteres de control, 127+ son extendidos),
            // lo mostramos como valor numérico sin convertir a carácter
            println!("Byte de control o no imprimible: {}", byte);
        }
    }
}
