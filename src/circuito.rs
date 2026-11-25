//! Circuitos como listas de operaciones, y el filtro cuanvolucional.

use crate::Estado;
use crate::puertas::{self, Matriz2};

/// Una operación del circuito.
#[derive(Clone, Debug)]
pub enum Op {
    U1 { q: usize, m: Matriz2 },
    Cnot { c: usize, t: usize },
}

/// Circuito fijo de `n` qubits.
#[derive(Clone, Debug)]
pub struct Circuito {
    n: usize,
    ops: Vec<Op>,
}

impl Circuito {
    pub fn nuevo(n: usize) -> Self {
        Self { n, ops: Vec::new() }
    }

    pub fn qubits(&self) -> usize {
        self.n
    }

    pub fn u1(&mut self, q: usize, m: Matriz2) -> &mut Self {
        assert!(q < self.n, "qubit fuera de rango");
        self.ops.push(Op::U1 { q, m });
        self
    }

    pub fn cnot(&mut self, c: usize, t: usize) -> &mut Self {
        assert!(
            c < self.n && t < self.n && c != t,
            "qubits inválidos para CNOT"
        );
        self.ops.push(Op::Cnot { c, t });
        self
    }

    /// Aplica el circuito sobre un estado existente.
    pub fn aplicar(&self, estado: &mut Estado) {
        assert_eq!(
            estado.qubits(),
            self.n,
            "el estado y el circuito no coinciden"
        );
        for op in &self.ops {
            match op {
                Op::U1 { q, m } => estado.aplicar_1q(*q, m),
                Op::Cnot { c, t } => estado.cnot(*c, *t),
            }
        }
    }

    /// Ejecuta el circuito desde |0…0⟩.
    pub fn ejecutar(&self) -> Estado {
        let mut e = Estado::nuevo(self.n);
        self.aplicar(&mut e);
        e
    }

    /// Filtro cuanvolucional sobre un bloque de píxeles en [0, 1]:
    /// codifica cada píxel con R_y(π·x) en su qubit, aplica este circuito
    /// y devuelve ⟨Z⟩ de cada qubit (una característica por qubit).
    pub fn filtro(&self, bloque: &[f64]) -> Vec<f64> {
        assert_eq!(
            bloque.len(),
            self.n,
            "el bloque debe tener un píxel por qubit"
        );
        let mut e = Estado::nuevo(self.n);
        for (q, &x) in bloque.iter().enumerate() {
            e.aplicar_1q(q, &puertas::ry(std::f64::consts::PI * x));
        }
        self.aplicar(&mut e);
        (0..self.n).map(|q| e.esperado_z(q)).collect()
    }
}
