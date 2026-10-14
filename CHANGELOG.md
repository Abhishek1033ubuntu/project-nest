# Release v2.0.0 — Major Architecture Overhaul: Causal Edge AI & Rust Core Engine

### Major Upgrades & Breaking Changes

* **Lock-Free SCM Engine Core (`rust_core`):**
  * Sub-millisecond DAG topological sorting and linear structural equation evaluations implemented in Rust.
  * Integration of C++ ONNX Runtime via `ort` crate for non-linear neural network estimators.
  * Lock-free atomic model hot-reloading using `arc_swap::ArcSwap` for zero telemetry packet loss during model updates.

* **Security & Verification Layer (`rust_core::security`):**
  * Mandatory SHA-256 binary hash checking and Ed25519 cryptographic signature verification prior to loading ONNX models into runtime memory (mitigating TOCTOU attacks).
  * Automated disk monitoring via `notify` crate file-watcher thread to trigger seamless, verified model swaps on file write completion.

* **Enterprise CMMS & ERP Automation (`cmms_bridge.py`):**
  * Automated mapping between SCM root-cause nodes and warehouse Bill of Materials (BOM) SKUs.
  * Production OData/REST drivers for automatic purchase requisition creation in SAP S/4HANA and IBM Maximo.

* **Data-Driven Causal Discovery (`discovery.py`):**
  * Implementation of NOTEARS continuous optimization algorithm (`tr(e<sup>W ∘ W</sup>) - d = 0`) to learn directed acyclic graphs directly from raw observational PLC logs.

* **Hardware-in-the-Loop Simulator & 3D Web GUI:**
  * OPC-UA mock server with dynamic fault injection endpoints (bearing wear, valve blockages).
  * React + Three.js plant canvas viewer displaying spatial coordinates and pulsing fault beacons for technician tablets.
  * Complete OpenAPI 3.0 and AsyncAPI 2.6 specification schemas.

* **Air-Gapped Edge Stack (`docker-compose.edge.yml`):**
  * Production-grade Docker Compose stack bundling NATS JetStream, Redis buffer, Rust engine, and NGINX dashboard for offline local factory servers.
