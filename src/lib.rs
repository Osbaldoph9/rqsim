//! `rqsim`: simulador de vector de estado para circuitos cuánticos pequeños,
//! pensado para acelerar capas cuanvolucionales.
//!
//! Convención de orden: little-endian, igual que Qiskit. El qubit `q`
//! corresponde al bit `q` del índice de la amplitud.

pub mod circuito;
pub mod estado;
pub mod puertas;

pub use circuito::{Circuito, Op};
pub use estado::Estado;
pub use num_complex::Complex64 as C;
