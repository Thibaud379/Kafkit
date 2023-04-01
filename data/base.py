# initialization
import numpy as np

# importing Qiskit
from qiskit_aer import AerSimulator
from qiskit.providers.fake_provider import FakeNairobi
from qiskit.providers.ibmq import least_busy
from qiskit import QuantumCircuit
from qiskit_ibm_runtime import QiskitRuntimeService, Session, Sampler


n = 3  # Circuit size
circuit = QuantumCircuit(n)  # Circuit

# Circuit begin
for i in range(n):  # Unroll
    circuit.h(i)

circuit.barrier()

circuit.cz(2, 0)
circuit.cz(2, 1)

circuit.barrier()

for i in range(n):  # Unroll
    circuit.h(i)
    circuit.x(i)

circuit.mcp(np.pi, list(range(0, n-1)), n-1)

for i in range(n):  # Unroll
    circuit.x(i)
    circuit.h(i)

circuit.measure_all(add_bits=True)
# Circuit end


service = QiskitRuntimeService()

with Session(service, "ibm_oslo"):
    sampler = Sampler()
    job = sampler.run(circuits=circuit, shots=1000)

print(job.job_id())
