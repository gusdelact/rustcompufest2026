# Fundamentos de programación de sistemas con Rust


## Instalación Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Las estructuras básicas de la programación de sistemas

Cuando uno programa sistemas, casi todo gira alrededor de cuatro cosas:

- buffer de bytes
- texto para mostra mensajes del programa
- estado
- handle de recurso

Si entendemos bien esas cuatro, ya tenemos el esqueleto de casi cualquier programa de sistemas: un echo, un cat, un servidor TCP, un parser de logs o una herramienta de línea de comandos.

1. Buffer de bytes

La estructura más elemental en sistemas es el buffer de bytes.

Un buffer de bytes es simplemente una región de memoria que contiene datos crudos: lo que llegó de un archivo, de un socket, de stdin o lo que vamos a enviar a stdout. En este nivel todavía no sabemos si eso representa texto, una imagen, un paquete de red o un número binario. Son solamente bytes.

En C, esto solía verse como:

char buf[1024];

o:

unsigned char buf[1024];

En Rust, las formas típicas son:

let buf: [u8; 1024];
let v: Vec<u8>;
let slice: &[u8];

Aquí hay una diferencia importante. En Rust, el tipo deja más claro qué clase de acceso tenemos:

[u8; 1024] es un arreglo de tamaño fijo
Vec<u8> es un buffer dinámico, que puede crecer
&[u8] es una vista prestada sobre bytes
&mut [u8] es una vista mutable sobre bytes

Eso ya nos enseña una lección muy propia de Rust: no basta con saber qué datos tengo; también importa saber quién los posee y quién puede modificarlos.

Idea clave para el taller

En programación de sistemas, el buffer de bytes es la materia prima.
Primero llegan o salen bytes. Después decidimos cómo interpretarlos.

2. Texto

La segunda estructura es el texto.

En sistemas, muchas cosas terminan siendo texto: comandos, rutas de archivos, logs, encabezados HTTP, líneas de configuración, nombres de usuarios. Pero aquí conviene insistir en algo fundamental:

texto no es lo mismo que bytes.

El texto es una interpretación de los bytes.
En Rust, esa diferencia está muy bien marcada.

Los tipos principales son:

String
&str
String es texto propio, almacenado dinámicamente
&str es una vista prestada sobre texto UTF-8 válido

Eso significa que en Rust no cualquier buffer de bytes puede tratarse como texto alegremente. Primero hay que verificar que realmente sea UTF-8 válido, o usar una conversión apropiada.

Ese detalle es valioso para enseñar programación de sistemas, porque obliga a pensar con disciplina:

si estoy leyendo de un socket, primero tengo bytes
si quiero imprimirlos como texto, tengo que interpretarlos como texto
si no son texto válido, debo tratarlos como binario

En C muchas veces esa frontera era borrosa. En Rust, en cambio, queda más clara y eso evita errores clásicos.

Idea clave para el taller

El texto en Rust no es un “char buffer con otro nombre”.
Es una estructura con reglas precisas.
Y eso es bueno, porque en sistemas la confusión entre bytes y texto suele causar bugs muy viejos y muy conocidos.

3. Estado

La tercera estructura es el estado.

Todo programa de sistemas mantiene estado.
Por ejemplo:

cuántos bytes se han leído
en qué fase está un protocolo
si un archivo está abierto o cerrado
si una conexión sigue activa
cuál fue el último error
qué modo eligió el usuario con una bandera de línea de comandos

En C, el estado a veces estaba disperso entre variables sueltas:

int connected = 1;
int bytes_read = 0;
int mode = 2;

En Rust, aunque también puede haber variables simples, es muy natural representar el estado con:

struct
enum
match

Por ejemplo, en lugar de usar enteros mágicos para representar fases, podemos escribir algo como:

enum EstadoConexion {
    Esperando,
    Conectado,
    Cerrado,
}

Esto es una maravilla para explicar sistemas, porque muchos programas de bajo nivel son en realidad máquinas de estados:

esperar entrada
procesar entrada
responder
cerrar recurso
manejar error

Rust ayuda mucho aquí porque hace natural modelar el estado de manera explícita. Y cuando el estado está explícito, el programa se entiende mejor.

Idea clave para el taller

Programar sistemas no es solo mover bytes.
También es mantener el control del estado del programa: saber dónde estoy, qué tengo abierto, qué ya procesé y qué falta por hacer.

4. Handle de recurso

La cuarta estructura es el handle de recurso.

En sistemas no trabajamos solamente con datos en memoria. Trabajamos con recursos del sistema operativo:

archivos
sockets
pipes
procesos
mutexes
directorios

En Unix clásico, un ejemplo típico era el file descriptor, un entero como 3, 4 o 5.

En Rust, en cambio, normalmente usamos valores con tipo, por ejemplo:

std::fs::File
std::net::TcpStream
std::process::Child

Eso cambia mucho la manera de enseñar.
En lugar de decir “tengo un entero que representa algo abierto”, podemos decir:

tengo un valor que representa la posesión o el acceso a un recurso del sistema.

Y aquí aparece una de las ideas más bonitas de Rust:
cuando el valor sale de alcance, Rust suele liberar el recurso automáticamente mediante Drop.

Es decir, el lenguaje hace natural algo que en C dependía mucho de la disciplina del programador:

abrir
usar
cerrar

En Rust, esa disciplina sigue existiendo, pero queda mucho mejor apoyada por el modelo del lenguaje.

Idea clave para el taller

Un handle de recurso no es solo un dato.
Es un valor que representa una relación viva con el sistema operativo.
