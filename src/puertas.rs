//! Puertas de un qubit como matrices de 2×2.

use crate::C;

/// Matriz de una puerta de un qubit, por filas.
pub type Matriz2 = [[C; 2]; 2];

const CERO: C = C::new(0.0, 0.0);
const UNO: C = C::new(1.0, 0.0);

pub fn x() -> Matriz2 {
    [[CERO, UNO], [UNO, CERO]]
}

pub fn z() -> Matriz2 {
    [[UNO, CERO], [CERO, -UNO]]
}

pub fn h() -> Matriz2 {
    let s = C::new(std::f64::consts::FRAC_1_SQRT_2, 0.0);
    [[s, s], [s, -s]]
}

/// R_x(θ) = exp(-i θ X / 2)
pub fn rx(theta: f64) -> Matriz2 {
    let (c, s) = ((theta / 2.0).cos(), (theta / 2.0).sin());
    [
        [C::new(c, 0.0), C::new(0.0, -s)],
        [C::new(0.0, -s), C::new(c, 0.0)],
    ]
}

/// R_y(θ) = exp(-i θ Y / 2)
pub fn ry(theta: f64) -> Matriz2 {
    let (c, s) = ((theta / 2.0).cos(), (theta / 2.0).sin());
    [
        [C::new(c, 0.0), C::new(-s, 0.0)],
        [C::new(s, 0.0), C::new(c, 0.0)],
    ]
}

/// R_z(θ) = exp(-i θ Z / 2)
pub fn rz(theta: f64) -> Matriz2 {
    let mitad = theta / 2.0;
    [
        [C::from_polar(1.0, -mitad), CERO],
        [CERO, C::from_polar(1.0, mitad)],
    ]
}
