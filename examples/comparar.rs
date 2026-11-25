//! Imprime las probabilidades de un circuito fijo de 4 qubits,
//! para compararlas con Qiskit (ver validacion/comparar_qiskit.py).
use rqsim::{Circuito, puertas};

fn main() {
    let mut c = Circuito::nuevo(4);
    c.u1(0, puertas::h())
        .u1(1, puertas::rx(0.3))
        .u1(2, puertas::rz(1.1))
        .u1(2, puertas::ry(0.8))
        .cnot(0, 3)
        .u1(3, puertas::ry(2.0))
        .cnot(2, 1)
        .u1(1, puertas::h());
    for p in c.ejecutar().probabilidades() {
        println!("{p:.15}");
    }
}
