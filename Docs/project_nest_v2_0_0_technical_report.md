# project-nest v2.0.0 — System Architecture & Technical Specification Report

**Document Title:** Comprehensive Technical Architecture and Operational Specification Report  
**Project Name:** `project-nest`  
**Version:** v2.0.0  
**Date:** October 14, 2026  
**License:** Apache License 2.0  
**Repository:** [https://github.com/Abhishek1033ubuntu/project-nest](https://github.com/Abhishek1033ubuntu/project-nest)  

---

## Author Credentials & Acknowledgements

* **Lead Author & Principal Developer:** Abhishek Singh
* **GitHub Profile:** [@Abhishek1033ubuntu](https://github.com/Abhishek1033ubuntu)
* **ORCID Identifier:** [0009-0004-6913-096X](https://orcid.org/0009-0004-6913-096X)
* **Primary Contact:** abhishek1033@gmail.com
* **Sponsorship & Support:** [GitHub Sponsors](https://github.com/sponsors/Abhishek1033ubuntu) | [PayPal Contribution](https://www.paypal.me/Abhishek1033ubuntu)
* **AI Architecture Collaboration:** Designed and engineered in technical collaboration with **Google Gemini**.

---

## Executive Summary

`project-nest` v2.0.0 is an edge-native, deterministic industrial automation platform engineered to optimize complex feedback loops, eliminate false-alarm cascades, and predict equipment failures long before catastrophic breakdown occurs.

Traditional Supervisory Control and Data Acquisition (SCADA) systems rely on static threshold alerts that trigger massive secondary alarm cascades when an upstream component degrades. `project-nest` replaces empirical correlation with **Structural Causal Models (SCMs)** and **$do$-calculus intervention logic**, isolating the exact physical root cause of a system failure in real time.

Operating entirely on-premise on air-gapped local edge servers, `project-nest` combines:
1. **Microsecond Rust Concurrency:** Lock-free graph traversal and sub-millisecond evaluation loops.
2. **Non-Linear Neural Estimators:** Embedded C++ ONNX Runtime integration for dynamic physical dynamics.
3. **Lock-Free Hot-Reloading:** Atomic pointer swapping (`ArcSwap`) for zero packet loss during live model updates.
4. **Cryptographic Binary Security:** Mandatory SHA-256 integrity and Ed25519 public-key signature verification.
5. **Pre-Arrival Maintenance Logistics:** Automatic requisition dispatch into SAP S/4HANA and IBM Maximo.
6. **3D Spatial Guidance:** React + Three.js plant canvas rendering exact component coordinates ($[X,Y,Z]$) for technician tablets.

---

## 1. System Architecture & Topology

The platform operates as a multi-tier, air-gapped edge appliance. Telemetry streams from physical field sensors via OPC-UA or Modbus TCP into a high-throughput NATS JetStream bus and Redis sliding-window memory buffer before evaluation by the Rust execution engine.

```text
       +-------------------------------------------------------------+
       |                  Industrial Field Devices                   |
       |      (Sensors, PLCs, Actuators, Drives, VFDs, Valves)       |
       +------------------------------+------------------------------+
                                      | High-Frequency Telemetry (OPC-UA / Modbus)
                                      v
       +-------------------------------------------------------------+
       |             project-nest Edge Ingestion Layer               |
       |     (NATS JetStream Bus + Redis Sliding Window Memory)      |
       +--------------+-------------------------------+--------------+
                      |                               |
                      v                               v
       +------------------------------+ +----------------------------+
       | Real-Time Control Loop       | | Causal Telemetry Buffer    |
       | (Deterministic Edge IPC /    | | (Lock-Free In-Memory     |
       |  PLC Logic / Interlocks)     | |  Sliding Window Queue)     |
       +------------------------------+ +--------------+-------------+
                                                       |
                                                       v
       +-------------------------------------------------------------+
       |        rust_core (PyO3 C-Extension Execution Engine)        |
       |                                                             |
       |  +------------------------+    +-------------------------+  |
       |  |  DAG & Topological     |    | Structural Causal Model |  |
       |  |  Sort Engine (dag.rs)  |--->| (SCM) Engine (scm.rs)   |  |
       |  +------------------------+    +------------+------------+  |
       |                                             |               |
       |  +------------------------+                 v               |
       |  | Ed25519 & SHA-256      |    +-------------------------+  |
       |  | Security (security.rs) |--->| Lock-Free ArcSwap ONNX  |  |
       |  +------------------------+    | Hot-Reloader            |  |
       |                                +------------+------------+  |
       +---------------------------------------------|---------------+
                                                     |
                                                     v
                                +------------------------------------+
                                | Actionable Diagnostic Outputs      |
                                |  • SAP S/4HANA & IBM Maximo PRs    |
                                |  • 3D Three.js Tablet GUI Overlay  |
                                |  • Automated Safety Interlocks     |
                                +------------------------------------+
```

---

## 2. Mathematical & Theoretical Foundations

### 2.1 Structural Causal Models (SCMs) & $do$-Calculus
A physical manufacturing unit is represented as a Directed Acyclic Graph (DAG) $\mathcal{G} = \langle \mathbf{V}, \mathbf{E} \rangle$, where each node $V_i \in \mathbf{V}$ corresponds to a sensor, actuator, or state variable. The structural value of $V_i$ is governed by a non-linear function $f_i$:

$$V_i = f_i(\mathbf{PA}_i, U_i)$$

where $\mathbf{PA}_i$ represents the direct causal parent nodes in $\mathcal{G}$, and $U_i$ represents unobserved noise or exogenous variables.

To distinguish true root causes from downstream symptoms, the engine computes interventional probabilities via Pearl's $do$-calculus:

$$P(Y \mid do(X = x)) \neq P(Y \mid X = x)$$

By forcing node state $X = x$ counterfactually, the system evaluates whether downstream anomalies dissolve, confirming whether $X$ is the primary root cause.

### 2.2 Structural Residual Shifts ($\delta_i$)
During operational execution, the Rust engine continuously evaluates the expected value $\hat{V}_i = f_i(\mathbf{PA}_i)$ against the observed telemetry $V_i^{\text{actual}}$. The **structural residual shift** $\delta_i$ is defined as:

$$\delta_i = \left| V_i^{\text{actual}} - f_i(\mathbf{PA}_i) \right|$$

When component degradation occurs (e.g., sub-surface bearing pitting), $\delta_i$ shifts significantly even while raw telemetry remains within traditional safety thresholds. The node exhibiting the maximum structural shift relative to its parent inputs is flagged as the root cause.

### 2.3 Continuous Causal Discovery (NOTEARS)
For unmapped plant processes, `project-nest` implements the NOTEARS algorithm to learn adjacency matrices $W \in \mathbb{R}^{d \times d}$ directly from raw PLC logs by formulating DAG structure learning as a continuous optimization problem:

$$\min_{W} \frac{1}{2n} \|X - XW\|_F^2 + \lambda_1 \|W\|_1 \quad \text{subject to} \quad h(W) = \text{tr}\left(e^{W \circ W}\right) - d = 0$$

The constraint $h(W) = 0$ mathematically guarantees that the learned graph $W$ is strictly acyclic.

---

## 3. Core Software Modules & Components

### 3.1 Rust Core Execution Engine (`rust_core/`)

* **`dag.rs`:** Implements Kahn's algorithm for sub-millisecond topological sorting and cycle detection.
* **`scm.rs`:** Evaluates structural equations using linear approximations or high-performance C++ ONNX Runtime bindings. Implements lock-free atomic model swapping via `arc_swap::ArcSwap<Session>`.
* **`security.rs`:** Enforces SHA-256 payload hashing and Ed25519 signature verification before committing ONNX models to memory.
* **`watcher.rs`:** Monitors file-system write events via the `notify` crate, triggering automated, non-blocking model reloads.
* **`lib.rs`:** Exposes zero-cost C-extension bindings to Python via PyO3.

### 3.2 Enterprise CMMS & ERP Bridge (`python/project_nest_causal/cmms_bridge.py`)

Automatically maps isolated causal nodes to internal warehouse Bill of Materials (BOM) SKUs and issues automated purchase requisitions:
* **IBM Maximo:** Communicates via OSLC REST JSON endpoints (`/maximo/api/os/mxpr`).
* **SAP S/4HANA:** Communicates via OData Plant Maintenance APIs (`/API_PURCHASEREQUISITION_PROCESS_SRV`).

### 3.3 Hardware-in-the-Loop Simulator (`simulation/opc_ua_hil_simulator.py`)

A 10Hz OPC-UA server simulating an industrial pumping station with real-time non-linear couplings and interactive fault injection endpoints (bearing wear, valve blockages, VFD frequency drift).

### 3.4 Interactive 3D Spatial GUI (`gui/src/PlantCadViewer.jsx`)

A React + Three.js (`@react-three/fiber`) web component that renders 3D plant CAD geometries, displaying pulsing, color-coded fault beacons at exact spatial coordinates ($[X, Y, Z]$) to guide field technicians.

---

## 4. API & Interface Specifications

### 4.1 Real-Time WebSocket Schema (`api/asyncapi.yaml`)
Pushes real-time causal diagnostics, 3D CAD coordinates, and auto-reserved spare parts metadata to technician tablets over WebSockets (`ws://edge-server.local:8080/ws/v1/diagnostics/live`).

### 4.2 Work-Order REST API (`api/openapi.yaml`)
Provides RESTful endpoints (`PATCH /api/v1/work-orders/{event_id}/status`) for technicians to acknowledge alerts, verify Lockout-Tagout (LOTO) safety protocols, and record completed maintenance tasks.

---

## 5. Security & Cryptographic Verification

To prevent unauthorized model tampering or malicious neural network injection on edge IPCs:

```text
[ New ONNX File on Disk ]
          │
          ▼
┌────────────────────────────────────────────────────────┐
│ 1. File Watcher Event Trigger (watcher.rs)            │
└─────────────────────────┬──────────────────────────────┘
                          │
                          ▼
┌────────────────────────────────────────────────────────┐
│ 2. SHA-256 Checksum Validation                         │
│    Calculates file hash and compares to manifest.      │
└─────────────────────────┬──────────────────────────────┘
                          │
                          ▼
┌────────────────────────────────────────────────────────┐
│ 3. Ed25519 Cryptographic Signature Verification        │
│    Verifies signature against trusted Public Key.      │
└─────────────────────────┬──────────────────────────────┘
                          │ (If Valid)
                          ▼
┌────────────────────────────────────────────────────────┐
│ 4. In-Memory Compilation & Atomic Swap                 │
│    Session::commit_from_memory() -> ArcSwap::store()   │
└────────────────────────────────────────────────────────┘
```

1. **In-Memory Loading:** ONNX binaries are read directly into RAM and compiled via `Session::commit_from_memory()`, preventing Time-of-Check to Time-of-Use (TOCTOU) file-swapping attacks.
2. **Ed25519 Signatures:** Models must be signed by an authorized build pipeline private key before the edge engine will accept runtime updates.

---

## 6. Air-Gapped Deployment & Operations

`project-nest` is packaged as a completely air-gapped Docker Compose stack (`docker-compose.edge.yml`):

```bash
# Production Deployment Command
chmod +x deploy_edge.sh
./deploy_edge.sh
```

### Container Stack Composition
* **`nats-broker`:** High-throughput NATS JetStream event router (`port 4222 / 8222`).
* **`redis-buffer`:** In-memory sliding-window telemetry store (`port 6379`).
* **`scm-core-engine`:** Native Rust/Python SCM execution engine.
* **`web-gui-server`:** Air-gapped NGINX server hosting the React/Three.js dashboard (`port 80 / 443`).

---

## 7. Verification & Integration Testing

The platform includes an end-to-end integration test suite (`tests/test_integration.py`) covering:
1. **OPC-UA Telemetry Stream Validation:** Verifies nominal signal generation and signal-to-noise ratios.
2. **Fault Injection Attributability:** Confirms that injecting bearing wear triggers `Motor_Current` root-cause flags with $\delta > 10.0$.
3. **NOTEARS Acyclicity Recovery:** Validates that continuous optimization correctly recovers directed causal graphs without cycles.
4. **End-to-End CMMS Dispatch:** Verifies that isolated faults successfully issue mock SAP/Maximo purchase requisitions.

---

## Conclusion & Repository Inventory

`project-nest` v2.0.0 bridges advanced theoretical physics and mathematical causality with low-level systems engineering. By deploying deterministic Rust concurrency at the asset level, industrial facilities achieve true predictive maintenance, total data privacy, and zero unplanned downtime.

### Primary Repository Files
```text
project-nest/
├── .github/FUNDING.yml             # GitHub Sponsors & PayPal funding configuration
├── api/
│   ├── asyncapi.yaml               # Real-time WebSocket diagnostic & 3D spatial schema
│   └── openapi.yaml                # REST API schema for work-order status lifecycle
├── gui/src/PlantCadViewer.jsx      # React + Three.js 3D CAD plant fault viewer
├── rust_core/                      # Native Rust execution engine (PyO3 module)
│   ├── Cargo.toml                  # Crate configuration & dependencies
│   └── src/                        # lib.rs, dag.rs, scm.rs, security.rs, watcher.rs
├── python/project_nest_causal/     # Python pipeline, CMMS bridge, NOTEARS discovery
├── simulation/opc_ua_hil_simulator.py # OPC-UA HIL pump station simulator
├── tests/test_integration.py       # PyTest end-to-end integration test suite
├── deploy_edge.sh                  # One-click air-gapped deployment script
├── docker-compose.edge.yml         # Containerized edge appliance stack
├── CITATION.cff                    # Citation metadata (ORCID: 0009-0004-6913-096X)
├── CHANGELOG.md                    # Release history & feature updates
├── LICENSE                         # Apache License 2.0
├── HOWTOUSE.md                     # Step-by-step user compilation & operational guide
└── README.md                       # Primary repository documentation & badges
```
