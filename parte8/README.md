# Parte 8 — Programación funcional en Rust (opcional)

Cada programa de las partes 1 a 7 reescrito con estilo funcional: iteradores, closures, combinadores. Cada archivo indica qué reemplaza y muestra la comparación imperativo vs funcional.

---

## Desde parte1 — Buffers y texto

| Original | Funcional | Cambio clave |
|---|---|---|
| `parte1/01_buffer.rs` | `01_buffer_funcional.rs` | `for + if` → `.iter().for_each()`, `.partition()`, `.filter().map().collect()` |
| `parte1/09_integrado.rs` | `04_log_funcional.rs` | 3 contadores mutables → `.flatten().map().inspect().fold()` |

## Desde parte2 — Comandos Unix

| Original | Funcional | Cambio clave |
|---|---|---|
| `parte2/01_cat.rs` | `p2_01_cat_funcional.rs` | `for + match` → `.lines().flatten().for_each()` |
| `parte2/02_echo.rs` | `p2_02_echo_funcional.rs` | `for + enumerate + if` → `.skip(1).collect().join(" ")` |
| `parte2/03_head.rs` | `03_head_funcional.rs` | contador + `if + break` → `.take(n)` |
| `parte2/04_tail.rs` | `p2_04_tail_funcional.rs` | buffer FIFO manual → `.collect()` + slice `[inicio..]` |
| `parte2/05_cut.rs` | `p2_05_cut_funcional.rs` | `split + if índice` → `.filter_map()` + `.nth()` |
| `parte2/06_od.rs` | `p2_06_od_funcional.rs` | `loop + match + for` → `.chunks(16).for_each()` |
| `parte2/07_wc.rs` | `02_wc_funcional.rs` | 3 variables → `.fold()` en una pasada |

## Desde parte3 — Procesos e hilos

| Original | Funcional | Cambio clave |
|---|---|---|
| `parte3/01_proceso.rs` | `p3_01_proceso_funcional.rs` | Comandos como datos + `.iter().for_each()` |
| `parte3/05_move_multihilos.rs` | `p3_05_multihilos_funcional.rs` | `for + push` → `(0..3).map(spawn).collect()` |

## Desde parte4 — Pipes y señales

| Original | Funcional | Cambio clave |
|---|---|---|
| `parte4/01_simple_pipeline.rs` | `p4_01_pipeline_funcional.rs` | Variables intermedias → `.take().map().unwrap()` encadenado |
| `parte4/03_pipeline_threads.rs` | `p4_03_pipeline_threads_funcional.rs` | `for` en cada hilo → `.iter().map().for_each()` |

## Desde parte5 — Sincronización

| Original | Funcional | Cambio clave |
|---|---|---|
| `parte5/01_mutex.rs` | `p5_01_mutex_funcional.rs` | `for + push + for join` → `(0..10).map(spawn).collect()` + `.into_iter().for_each(join)` |
| `parte5/04_channels.rs` | `p5_04_channels_funcional.rs` | `for` → `(1..=5).map(format).for_each(send)` |

## Desde parte6 — Problemas clásicos

| Original | Funcional | Cambio clave |
|---|---|---|
| `parte6/01_productor_consumidor.rs` | `p6_01_productor_consumidor_funcional.rs` | `for + send` → `(0..5).for_each(send)` + `rx.iter().for_each()` |
| `parte6/03_filosofos_comensales.rs` | `p6_03_filosofos_funcional.rs` | Creación de tenedores y hilos con `.map().collect()` |

## Desde parte7 — Red

| Original | Funcional | Cambio clave |
|---|---|---|
| `parte7/03_servidor_multihilo.rs` | `p7_03_servidor_multihilo_funcional.rs` | `for + match` → `.incoming().flatten().for_each(spawn)` |
| `parte7/04_net_echo.rs` | `p7_04_echo_funcional.rs` | `for + match` → `.incoming().flatten().for_each(spawn)` |

---

## Documentos

- **estilo_funcional.md** — Guía completa: tabla de equivalencias, patrones comunes, referencia de métodos de iteradores.

## Cómo compilar

```bash
rustc p2_01_cat_funcional.rs -o bin/p2_01_cat_funcional
```
