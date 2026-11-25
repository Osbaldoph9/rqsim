use rqsim::{Circuito, Estado, puertas};
use std::f64::consts::PI;

const TOL: f64 = 1e-12;

fn cerca(a: f64, b: f64) -> bool {
    (a - b).abs() < TOL
}

#[test]
fn hadamard_da_50_50() {
    let mut e = Estado::nuevo(1);
    e.aplicar_1q(0, &puertas::h());
    let p = e.probabilidades();
    assert!(cerca(p[0], 0.5) && cerca(p[1], 0.5));
}

#[test]
fn hadamard_dos_veces_es_identidad() {
    let mut e = Estado::nuevo(1);
    e.aplicar_1q(0, &puertas::h());
    e.aplicar_1q(0, &puertas::h());
    assert!(cerca(e.probabilidades()[0], 1.0));
}

#[test]
fn ry_sigue_seno_cuadrado() {
    for k in 0..=8 {
        let theta = k as f64 * PI / 8.0;
        let mut e = Estado::nuevo(1);
        e.aplicar_1q(0, &puertas::ry(theta));
        assert!(cerca(e.probabilidades()[1], (theta / 2.0).sin().powi(2)));
    }
}

#[test]
fn estado_de_bell() {
    let mut c = Circuito::nuevo(2);
    c.u1(0, puertas::h()).cnot(0, 1);
    let p = c.ejecutar().probabilidades();
    // |00⟩ y |11⟩ con 1/2 cada uno, como en Qiskit
    assert!(cerca(p[0b00], 0.5) && cerca(p[0b11], 0.5));
    assert!(cerca(p[0b01], 0.0) && cerca(p[0b10], 0.0));
}

#[test]
fn ghz_de_tres_qubits() {
    let mut c = Circuito::nuevo(3);
    c.u1(0, puertas::h()).cnot(0, 1).cnot(1, 2);
    let p = c.ejecutar().probabilidades();
    assert!(cerca(p[0b000], 0.5) && cerca(p[0b111], 0.5));
}

#[test]
fn orden_little_endian() {
    // X sobre el qubit 1 de 2 qubits da |10⟩, índice 2
    let mut e = Estado::nuevo(2);
    e.aplicar_1q(1, &puertas::x());
    assert!(cerca(e.probabilidades()[2], 1.0));
}

#[test]
fn la_norma_se_conserva() {
    let mut c = Circuito::nuevo(4);
    c.u1(0, puertas::h())
        .u1(1, puertas::rx(0.3))
        .u1(2, puertas::rz(1.1))
        .cnot(0, 3)
        .u1(3, puertas::ry(2.0))
        .cnot(2, 1);
    assert!(cerca(c.ejecutar().norma(), 1.0));
}

#[test]
fn parameter_shift_coincide_con_la_derivada() {
    // ⟨Z⟩ tras R_y(θ) es cos θ; su derivada es −sen θ
    let z = |theta: f64| {
        let mut e = Estado::nuevo(1);
        e.aplicar_1q(0, &puertas::ry(theta));
        e.esperado_z(0)
    };
    let theta = 0.7;
    let grad = (z(theta + PI / 2.0) - z(theta - PI / 2.0)) / 2.0;
    assert!(cerca(grad, -theta.sin()));
}

#[test]
fn filtro_sin_circuito_devuelve_cos_pi_x() {
    // Con circuito vacío, ⟨Z⟩ de cada qubit es cos(π·x)
    let c = Circuito::nuevo(4);
    let bloque = [0.0, 0.25, 0.5, 1.0];
    let f = c.filtro(&bloque);
    for (q, &x) in bloque.iter().enumerate() {
        assert!(cerca(f[q], (PI * x).cos()));
    }
}
