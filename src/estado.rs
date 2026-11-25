//! Vector de estado de n qubits.

use crate::C;
use crate::puertas::Matriz2;

/// Estado puro de `n` qubits guardado como 2^n amplitudes complejas.
#[derive(Clone, Debug)]
pub struct Estado {
    n: usize,
    amp: Vec<C>,
}

impl Estado {
    /// Estado |0…0⟩ de `n` qubits.
    pub fn nuevo(n: usize) -> Self {
        assert!((1..=26).contains(&n), "n debe estar entre 1 y 26 qubits");
        let mut amp = vec![C::new(0.0, 0.0); 1 << n];
        amp[0] = C::new(1.0, 0.0);
        Self { n, amp }
    }

    pub fn qubits(&self) -> usize {
        self.n
    }

    pub fn amplitudes(&self) -> &[C] {
        &self.amp
    }

    /// Probabilidad de cada estado de la base computacional.
    pub fn probabilidades(&self) -> Vec<f64> {
        self.amp.iter().map(|a| a.norm_sqr()).collect()
    }

    pub fn norma(&self) -> f64 {
        self.amp.iter().map(|a| a.norm_sqr()).sum::<f64>().sqrt()
    }

    /// Aplica una puerta de un qubit sobre el qubit `q`.
    ///
    /// Recorre los pares de índices que difieren solo en el bit `q`:
    /// `i` con el bit apagado y `j = i | (1 << q)` con el bit encendido.
    pub fn aplicar_1q(&mut self, q: usize, m: &Matriz2) {
        assert!(q < self.n, "qubit fuera de rango");
        let bit = 1usize << q;
        for i in 0..self.amp.len() {
            if i & bit == 0 {
                let j = i | bit;
                let (a0, a1) = (self.amp[i], self.amp[j]);
                self.amp[i] = m[0][0] * a0 + m[0][1] * a1;
                self.amp[j] = m[1][0] * a0 + m[1][1] * a1;
            }
        }
    }

    /// CNOT con control `c` y objetivo `t`: intercambia las amplitudes
    /// donde el control vale 1 y el objetivo difiere.
    pub fn cnot(&mut self, c: usize, t: usize) {
        assert!(
            c < self.n && t < self.n && c != t,
            "qubits inválidos para CNOT"
        );
        let (bc, bt) = (1usize << c, 1usize << t);
        for i in 0..self.amp.len() {
            if i & bc != 0 && i & bt == 0 {
                self.amp.swap(i, i | bt);
            }
        }
    }

    /// Valor esperado ⟨Z⟩ del qubit `q`: P(bit = 0) − P(bit = 1).
    pub fn esperado_z(&self, q: usize) -> f64 {
        assert!(q < self.n, "qubit fuera de rango");
        let bit = 1usize << q;
        self.amp
            .iter()
            .enumerate()
            .map(|(i, a)| {
                if i & bit == 0 {
                    a.norm_sqr()
                } else {
                    -a.norm_sqr()
                }
            })
            .sum()
    }
}
