# initialization
import numpy as np
import sys
# importing Qiskit
from qiskit.providers.fake_provider import FakeOslo
from qiskit import QuantumCircuit, execute
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

if len(sys.argv) >= 2 and sys.argv[1] == 'sim':
    backend = FakeOslo()
    job = execute(circuit, backend, shots=1000)
    print(sys.argv[0], job.result().get_counts())
else:
    service = QiskitRuntimeService()

    with Session(service, "ibm_oslo"):
        sampler = Sampler()
        job = sampler.run(circuits=circuit, shots=1000)

    print(sys.argv[0], job.job_id())
