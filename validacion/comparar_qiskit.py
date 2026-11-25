"""Compara el circuito de examples/comparar.rs contra Qiskit."""
import subprocess, sys
import numpy as np
from qiskit import QuantumCircuit
from qiskit.quantum_info import Statevector

qc = QuantumCircuit(4)
qc.h(0); qc.rx(0.3, 1); qc.rz(1.1, 2); qc.ry(0.8, 2)
qc.cx(0, 3); qc.ry(2.0, 3); qc.cx(2, 1); qc.h(1)
esperado = Statevector(qc).probabilities()

salida = subprocess.run(["cargo", "run", "-q", "--example", "comparar"],
                        capture_output=True, text=True, check=True).stdout
obtenido = np.array([float(x) for x in salida.split()])
dif = np.max(np.abs(esperado - obtenido))
print(f"diferencia máxima contra Qiskit: {dif:.2e}")
sys.exit(0 if dif < 1e-12 else 1)
