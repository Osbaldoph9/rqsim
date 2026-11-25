# rqsim

Simulador de vector de estado en Rust para acelerar capas cuanvolucionales.
Proyecto de Estadía Profesional (UVEG) con el grupo del Dr. Javier Orduz, FES Acatlán.

## Estado actual: v0.1

- `Estado`: vector de 2^n amplitudes complejas, orden little-endian como Qiskit.
- Puertas de un qubit: X, Z, H, R_x, R_y, R_z.
- CNOT y valor esperado ⟨Z⟩ por qubit.
- `Circuito`: lista de operaciones que se aplica sobre un estado.
- `Circuito::filtro`: codifica un bloque de píxeles con R_y(π·x), aplica el circuito y devuelve ⟨Z⟩ por qubit.

## Uso

```rust
use rqsim::{puertas, Circuito};

let mut c = Circuito::nuevo(2);
c.u1(0, puertas::h()).cnot(0, 1);       // estado de Bell
let p = c.ejecutar().probabilidades();  // [0.5, 0, 0, 0.5]
```

## Trabajar en GitHub Codespaces

El repositorio trae un contenedor listo (`.devcontainer/`): Rust estable con `rustfmt` y `clippy`, Python 3.12 con Qiskit y la extensión rust-analyzer. Al crear el codespace se instalan las dependencias y se corren las pruebas solas.

1. En la página del repositorio: **Code → Codespaces → Create codespace on main**.
2. Espera a que termine la preparación (la primera vez tarda unos minutos).
3. En la terminal:

```bash
cargo test                              # pruebas de Rust
python validacion/comparar_qiskit.py    # validación contra Qiskit
cargo run --example comparar            # probabilidades del circuito de ejemplo
```

Los codespaces se apagan solos tras un rato sin uso y conservan los archivos. Para no gastar horas de la cuota gratuita, detén el codespace al terminar: **Code → Codespaces → ⋯ → Stop codespace**.

## Pruebas

```bash
cargo fmt --check                           # formato
cargo clippy --all-targets -- -D warnings   # lint sin advertencias
cargo test                                  # 9 pruebas: Bell, GHZ, R_y, parameter-shift, filtro…
python validacion/comparar_qiskit.py        # mismo circuito de 4 qubits en Qiskit
```

La comparación contra Qiskit da una diferencia máxima de 4e-16. GitHub Actions (`.github/workflows/ci.yml`) corre todo esto en cada `push`.

## Estructura

```
src/
  lib.rs          módulos públicos
  estado.rs       vector de estado, puertas de un qubit, CNOT, ⟨Z⟩
  puertas.rs      matrices X, Z, H, R_x, R_y, R_z
  circuito.rs     lista de operaciones y filtro cuanvolucional
tests/basicas.rs  pruebas de física: Bell, GHZ, norma, parameter-shift
examples/         circuito de ejemplo para comparar con Qiskit
validacion/       script de comparación contra Qiskit
```

## Siguientes pasos

| Versión | Qué agrega |
| --- | --- |
| v0.2 | Circuito aleatorio tipo `RandomLayers` y comparación contra PennyLane |
| v0.3 | Lotes de bloques en paralelo con `rayon` y benchmarks con `criterion` |
| v0.4 | Módulo de Python con PyO3 y maturin: `rqsim.quanv(imagenes, filtro)` |
| v0.5 | Gradientes (parameter-shift) para filtros entrenables |
