//! Servidor HTTP minimalista en Rust.
//!
//! Solo sirve archivos HTML estáticos desde un directorio raíz (`public/`).
//! Escucha en `127.0.0.1:8080` y atiende cada conexión en un hilo separado.
//! Soporta únicamente el método GET; cualquier otro método devuelve 501.

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::thread;

/// Dirección IP y puerto donde escucha el servidor.
const ADDR: &str = "127.0.0.1:8080";

/// Directorio raíz desde el que se sirven los archivos estáticos.
/// Todas las rutas solicitadas se resuelven relativas a esta carpeta.
const WEB_ROOT: &str = "public";

/// Punto de entrada del programa.
///
/// 1. Crea un `TcpListener` vinculado a `ADDR`.
/// 2. Entra en un bucle infinito aceptando conexiones entrantes.
/// 3. Cada conexión aceptada se delega a un hilo nuevo que ejecuta
///    `handle_client`, de modo que el servidor puede atender múltiples
///    clientes de forma concurrente.
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(ADDR)?;
    println!("Servidor HTTP escuchando en http://{ADDR}");

    // `listener.incoming()` produce un iterador infinito de conexiones.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Se lanza un hilo por cada conexión para no bloquear
                // la aceptación de nuevas conexiones mientras se procesa una.
                thread::spawn(move || {
                    if let Err(err) = handle_client(stream) {
                        eprintln!("Error atendiendo cliente: {err}");
                    }
                });
            }
            Err(err) => eprintln!("Error aceptando conexión: {err}"),
        }
    }

    Ok(())
}

/// Procesa una conexión TCP individual siguiendo el protocolo HTTP/1.x.
///
/// Flujo:
/// 1. Lee la línea de petición (ej. `GET /index.html HTTP/1.1`).
/// 2. Valida que tenga exactamente tres partes: método, ruta y versión.
/// 3. Verifica que la versión sea HTTP/1.0 o HTTP/1.1.
/// 4. Consume las cabeceras restantes (hasta la línea vacía `\r\n`).
/// 5. Despacha según el método:
///    - `GET` → `handle_get`
///    - Cualquier otro → responde con 501 Not Implemented.
fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    // Se clona el stream para poder leer con un BufReader y seguir
    // escribiendo en el stream original.
    let mut reader = BufReader::new(stream.try_clone()?);

    // --- 1. Leer la línea de petición (request line) ---
    let mut request_line = String::new();
    let bytes = reader.read_line(&mut request_line)?;

    // Si no se leyó nada, el cliente cerró la conexión de inmediato.
    if bytes == 0 {
        return Ok(());
    }

    // Eliminar los caracteres de fin de línea (\r\n o \n).
    let request_line = request_line.trim_end_matches(['\r', '\n']);
    println!("Petición: {request_line}");

    // --- 2. Separar la línea en sus tres componentes ---
    // Ejemplo: ["GET", "/index.html", "HTTP/1.1"]
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() != 3 {
        // La línea de petición no tiene el formato esperado.
        send_response(
            &mut stream,
            "400 Bad Request",
            "text/plain; charset=utf-8",
            b"400 Bad Request\n",
        )?;
        return Ok(());
    }

    let method = parts[0];  // Método HTTP (GET, POST, etc.)
    let target = parts[1];  // Ruta solicitada (ej. "/about.html")
    let version = parts[2]; // Versión del protocolo (ej. "HTTP/1.1")

    // --- 3. Validar la versión HTTP ---
    if version != "HTTP/1.1" && version != "HTTP/1.0" {
        send_response(
            &mut stream,
            "400 Bad Request",
            "text/plain; charset=utf-8",
            b"400 Bad Request\n",
        )?;
        return Ok(());
    }

    // --- 4. Consumir las cabeceras HTTP ---
    // Las cabeceras terminan con una línea vacía ("\r\n").
    // No las procesamos, pero debemos leerlas para vaciar el buffer.
    loop {
        let mut header_line = String::new();
        let n = reader.read_line(&mut header_line)?;
        // n == 0 → fin del stream; "\r\n" → línea vacía que marca el fin
        // de las cabeceras.
        if n == 0 || header_line == "\r\n" {
            break;
        }
    }

    // --- 5. Despachar según el método HTTP ---
    match method {
        "GET" => handle_get(&mut stream, target),
        // Métodos conocidos pero no soportados por este servidor.
        "POST" | "PUT" | "DELETE" | "HEAD" => {
            send_response(
                &mut stream,
                "501 Not Implemented",
                "text/plain; charset=utf-8",
                b"501 Not Implemented\n",
            )
        }
        // Cualquier método desconocido.
        _ => {
            send_response(
                &mut stream,
                "501 Not Implemented",
                "text/plain; charset=utf-8",
                b"501 Not Implemented\n",
            )
        }
    }
}

/// Maneja una petición GET.
///
/// 1. Resuelve la ruta solicitada a una ruta segura dentro de `WEB_ROOT`
///    mediante `resolve_path`. Si la ruta es inválida (ej. intento de
///    directory traversal), responde con 400.
/// 2. Comprueba que el archivo exista; si no, responde con 404.
/// 3. Solo permite servir archivos con extensión `.html` o `.htm`.
///    Cualquier otro tipo devuelve 403 Forbidden.
/// 4. Lee el contenido del archivo y lo envía como respuesta 200.
///    Si la lectura falla, responde con 500.
fn handle_get(stream: &mut TcpStream, target: &str) -> std::io::Result<()> {
    // Resolver la ruta del request a una ruta segura en el sistema de archivos.
    let path = match resolve_path(target) {
        Some(p) => p,
        None => {
            // La ruta contiene componentes peligrosos (ej. "..").
            send_response(
                stream,
                "400 Bad Request",
                "text/plain; charset=utf-8",
                b"400 Bad Request\n",
            )?;
            return Ok(());
        }
    };

    // Verificar que el archivo existe y es un archivo regular (no directorio).
    if !path.exists() || !path.is_file() {
        send_response(
            stream,
            "404 Not Found",
            "text/plain; charset=utf-8",
            b"404 Not Found\n",
        )?;
        return Ok(());
    }

    // Comprobar que la extensión sea .html o .htm.
    // Esto restringe el servidor a servir únicamente contenido HTML.
    let is_html = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| matches!(e, "html" | "htm"))
        .unwrap_or(false);

    if !is_html {
        send_response(
            stream,
            "403 Forbidden",
            "text/plain; charset=utf-8",
            b"403 Forbidden\nSolo se sirven archivos HTML.\n",
        )?;
        return Ok(());
    }

    // Leer el archivo completo en memoria y enviarlo como respuesta.
    match fs::read(&path) {
        Ok(body) => send_response(stream, "200 OK", "text/html; charset=utf-8", &body),
        Err(_) => send_response(
            stream,
            "500 Internal Server Error",
            "text/plain; charset=utf-8",
            b"500 Internal Server Error\n",
        ),
    }
}

/// Convierte la ruta solicitada por el cliente (ej. `/about.html`) en una
/// ruta segura dentro de `WEB_ROOT`.
///
/// Medidas de seguridad contra directory traversal:
/// - Descarta la query string (`?clave=valor`).
/// - Si la ruta es `/`, la mapea a `index.html` (documento por defecto).
/// - Elimina la barra inicial para obtener una ruta relativa.
/// - Rechaza rutas absolutas.
/// - Rechaza cualquier componente `..` (ParentDir), `/` (RootDir) o
///   prefijo de unidad en Windows (Prefix), que podrían escapar de
///   `WEB_ROOT`.
///
/// Devuelve `None` si la ruta es insegura; de lo contrario devuelve
/// `Some(ruta)` con la ruta completa dentro de `WEB_ROOT`.
fn resolve_path(target: &str) -> Option<PathBuf> {
    // Separar la ruta de los parámetros de query string.
    // Ej: "/page.html?v=2" → "/page.html"
    let path_only = target.split('?').next()?;

    // Si la ruta es la raíz "/", servir index.html por defecto.
    // En caso contrario, quitar la barra inicial para hacerla relativa.
    let relative = if path_only == "/" {
        PathBuf::from("index.html")
    } else {
        PathBuf::from(path_only.trim_start_matches('/'))
    };

    // Rechazar rutas absolutas (ej. "/etc/passwd" en Linux o "C:\..." en Windows).
    if relative.is_absolute() {
        return None;
    }

    // Inspeccionar cada componente de la ruta para detectar intentos de
    // escapar del directorio raíz.
    for component in relative.components() {
        if matches!(component, Component::ParentDir | Component::RootDir | Component::Prefix(_)) {
            return None;
        }
    }

    // Construir la ruta final: WEB_ROOT + ruta relativa.
    // Ej: "public" + "about.html" → "public/about.html"
    Some(Path::new(WEB_ROOT).join(relative))
}

/// Envía una respuesta HTTP/1.1 completa al cliente.
///
/// Construye las cabeceras obligatorias:
/// - `Content-Type`: tipo MIME del cuerpo (ej. `text/html; charset=utf-8`).
/// - `Content-Length`: tamaño en bytes del cuerpo, necesario para que el
///   cliente sepa cuántos bytes leer.
/// - `Connection: close`: indica que el servidor cerrará la conexión tras
///   enviar la respuesta (no se reutiliza la conexión).
///
/// Después de las cabeceras escribe el cuerpo y hace flush para asegurar
/// que todos los bytes se envían antes de cerrar el socket.
fn send_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
) -> std::io::Result<()> {
    // Escribir la línea de estado y las cabeceras.
    // La doble secuencia \r\n al final separa las cabeceras del cuerpo.
    write!(
        stream,
        "HTTP/1.1 {status}\r\n\
         Content-Type: {content_type}\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n",
        body.len()
    )?;
    // Escribir el cuerpo de la respuesta.
    stream.write_all(body)?;
    // Forzar el envío de todos los datos pendientes en el buffer.
    stream.flush()?;
    Ok(())
}
