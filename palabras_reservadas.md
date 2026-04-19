## 1) Keywords estrictas

Son las que realmente forman parte del lenguaje hoy y no pueden usarse libremente como nombres. La referencia oficial lista estas: `_`, `as`, `async`, `await`, `break`, `const`, `continue`, `crate`, `dyn`, `else`, `enum`, `extern`, `false`, `fn`, `for`, `if`, `impl`, `in`, `let`, `loop`, `match`, `mod`, `move`, `mut`, `pub`, `ref`, `return`, `self`, `Self`, `static`, `struct`, `super`, `trait`, `true`, `type`, `unsafe`, `use`, `where`, `while`. ([Rust Documentation][1])

Te las explico en lenguaje de taller, agrupadas:

* **Control de flujo**: `if`, `else`, `match`, `loop`, `while`, `for`, `in`, `break`, `continue`, `return`. Sirven para decidir, repetir e interrumpir o salir de funciones. `match` es especialmente importante en Rust porque hace selección por patrones, no solo por condiciones. ([Rust Documentation][2])
* **Definición de código y tipos**: `fn`, `struct`, `enum`, `trait`, `impl`, `type`, `mod`. Se usan para declarar funciones, estructuras, enumeraciones, traits, implementaciones, alias de tipos y módulos. ([Rust Documentation][2])
* **Ámbito y visibilidad**: `use`, `crate`, `super`, `self`, `Self`, `pub`, `where`. Ayudan a traer nombres al scope, navegar módulos, referirse al tipo actual y expresar restricciones de tipos. ([Rust Documentation][2])
* **Variables, enlaces y patrones**: `let`, `mut`, `ref`, `_`. `let` liga valores a nombres; `mut` marca mutabilidad; `ref` liga por referencia; `_` es el comodín que ignora valores cuando no importan. ([Rust Documentation][2])
* **Valores, memoria y bajo nivel**: `const`, `static`, `move`, `unsafe`, `extern`, `dyn`, `as`. `const` y `static` declaran valores fijos o globales; `move` hace que un closure capture por ownership; `unsafe` habilita operaciones que el compilador no puede garantizar; `extern` conecta con símbolos externos; `dyn` indica despacho dinámico sobre trait objects; `as` se usa para casting o renombrado en `use`. ([Rust Documentation][2])
* **Asincronía y booleanos**: `async`, `await`, `true`, `false`. `async` crea una computación que devuelve un `Future`; `await` suspende hasta que ese `Future` esté listo; `true` y `false` son los literales booleanos. ([Rust Documentation][2])

## 2) Keywords reservadas para futuro

Estas **todavía no tienen funcionalidad estable de uso general**, pero Rust las aparta para que nadie las use como nombres normales y así mantener compatibilidad futura: `abstract`, `become`, `box`, `do`, `final`, `gen`, `macro`, `override`, `priv`, `try`, `typeof`, `unsized`, `virtual`, `yield`. ([Rust Documentation][1])

Dos detalles históricos útiles:

* `try` quedó reservado desde la edición 2018. ([Rust Documentation][1])
* `gen` quedó reservado en la edición 2024, pensando en futuros **gen blocks**. ([Rust Documentation][1])

## 3) Keywords débiles o contextuales

Estas solo son especiales en ciertos contextos; fuera de ellos, varias sí pueden aparecer como nombres. La referencia oficial enumera: `'static`, `macro_rules`, `raw`, `safe`, `union`. ([Rust Documentation][1])

Su idea general es esta:

* `'static`: la lifetime estática; no puede usarse como parámetro de lifetime genérico ni como etiqueta de loop. ([Rust Documentation][1])
* `macro_rules`: se usa para definir macros declarativas. ([Rust Documentation][1])
* `raw`: aparece en operadores de *raw borrow*, por ejemplo `&raw const expr` o `&raw mut expr`. ([Rust Documentation][1])
* `safe`: tiene significado en funciones y estáticos dentro de bloques externos. ([Rust Documentation][1])
* `union`: sirve para declarar una `union` y solo actúa como keyword en ese contexto. ([Rust Documentation][1])

## 4) Lo más importante para no tropezar

Si intentas usar una keyword como nombre, el compilador te va a rechazar algo como `fn match(...)`. La forma de salir de eso es usar un **raw identifier**, por ejemplo `fn r#match(...)`. La guía oficial también muestra ese mismo mecanismo para migraciones como `gen` en Rust 2024. ([Rust Documentation][2])

[1]: https://doc.rust-lang.org/reference/keywords.html "Keywords - The Rust Reference"
[2]: https://doc.rust-lang.org/book/appendix-01-keywords.html "A - Keywords - The Rust Programming Language"
