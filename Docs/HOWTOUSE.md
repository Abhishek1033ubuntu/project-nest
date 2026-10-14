# How to Use project-nest v2.0.0

This guide provides step-by-step instructions for compiling, running, testing, and deploying the **project-nest** causal industrial automation engine on local hardware or air-gapped edge servers.

---

## 1. System Prerequisites

Ensure the following dependencies are installed on your workstation or edge IPC:

* **Rust Toolchain:** `rustc` & `cargo` (1.75+) — [Install Rust](https://www.rust-lang.org/tools/install)
* **Python Environment:** Python 3.10+ with `pip`
* **Node.js & Npm:** Node.js 18+ (for 3D GUI)
* **Build Tools:** `maturin` (for Rust-Python PyO3 compilation), `gcc`/`clang`, `cmake`
* **Container Runtime (Optional for Deployment):** Docker & Docker Compose v2

---

## 2. Quick Start: Compiling & Installation

### Step A: Clone the Repository
```bash
git clone [https://github.com/Abhishek1033ubuntu/project-nest.git](https://github.com/Abhishek1033ubuntu/project-nest.git)
cd project-nest

```

### Step B: Build the Rust Core Engine (`scm_core`)

Compile the lock-free Rust engine and install it into your Python environment using `maturin`:

```bash
# Install maturin build tool
pip install maturin

# Build and install scm_core C-extension in editable mode
cd rust_core
maturin develop --release
cd ..

```

### Step C: Install Python Dependencies

```bash
pip install -r python/requirements.txt

```

---

## 3. Running local HIL Simulation & Causal Diagnostics

To test the engine under simulated physical plant conditions:

### Step A: Launch the OPC-UA Hardware-in-the-Loop (HIL) Simulator

In Terminal 1, start the mock OPC-UA pump station server:

```bash
python simulation/opc_ua_hil_simulator.py

```

*The simulator starts streaming telemetry at 10Hz on `opc.tcp://0.0.0.0:4840` and automatically injects a simulated bearing fault after 5 seconds.*

### Step B: Run the SCM Causal Engine Pipeline

In Terminal 2, execute the Causal Pipeline Controller:

```bash
python python/project_nest_causal/engine.py

```

*The engine evaluates incoming telemetry, detects structural residual shifts ($\delta_i$), and outputs exact root-cause attributions.*

---

## 4. Running the Interactive 3D CAD Technician GUI

To launch the web interface for 3D spatial fault tracking:

```bash
cd gui
npm install
npm start

```

*Open `http://localhost:3000` in a browser or technician tablet to view the interactive Three.js 3D plant layout and pulsing fault beacons.*

---

## 5. Model Hot-Reloading & Cryptographic Verification

`project-nest` supports zero-downtime ONNX model hot-reloading protected by SHA-256 and Ed25519 signatures.

### Enabling Auto-Reload in Python

```python
import scm_core

engine = scm_core.PySCMEngine()
engine.set_trusted_public_key("<YOUR_64_CHAR_HEX_PUBLIC_KEY>")

# Register node ONNX model and enable automatic file-system watcher
engine.register_onnx_equation("Discharge_Pressure", ["RPM", "Flow"], "models/pressure_model.onnx")
engine.enable_auto_reload({"Discharge_Pressure": "models/pressure_model.onnx"})

```

*Any write operation to `models/pressure_model.onnx` automatically triggers an atomic pointer swap inside the Rust core without dropping telemetry frames.*

---

## 6. Running Integration Tests

Run the PyTest integration suite to verify OPC-UA simulation, NOTEARS causal discovery, and SCM engine diagnostics end-to-end:

```bash
pytest tests/test_integration.py -v

```

---

## 7. Production Air-Gapped Edge Deployment

To deploy the full containerized stack (NATS JetStream, Redis buffer, Rust core, and NGINX dashboard) on an air-gapped edge node:

```bash
# Make deployment script executable
chmod +x deploy_edge.sh

# Run air-gapped edge startup
./deploy_edge.sh

```

---

## Project Structure Quick Reference

```text
project-nest/
├── rust_core/              # Rust execution engine (SCM, lock-free swap, security)
├── python/                 # High-level pipeline, CMMS bridge, NOTEARS discovery
├── simulation/             # OPC-UA HIL simulator with fault injection
├── gui/                    # React + Three.js 3D CAD plant viewer
├── api/                    # OpenAPI 3.0 & AsyncAPI 2.6 specifications
├── tests/                  # PyTest integration test suite
├── docker-compose.edge.yml # Air-gapped container orchestration stack
└── deploy_edge.sh          # One-click edge deployment script

```

```

```
