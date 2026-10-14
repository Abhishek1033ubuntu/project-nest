# project-nest

[![Co-Developed with-Google Gemini-4285F4?style=for-the-badge&logo=google-gemini&logoColor=white](https://img.shields.io/badge/Co--Developed%20with-Google%20Gemini-4285F4?style=for-the-badge&logo=google-gemini&logoColor=white)](https://gemini.google.com)
[![Architecture-Causal%20Edge%20AI-00599C?style=for-the-badge&logo=rust&logoColor=white](https://img.shields.io/badge/Architecture-Causal%20Edge%20AI-00599C?style=for-the-badge&logo=rust&logoColor=white)](#)
[![License-Apache%202.0-blue.svg?style=for-the-badge](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=for-the-badge)](LICENSE)
[![Sponsor-GitHub%20Sponsors-EA4AAA?style=for-the-badge&logo=github-sponsors&logoColor=white](https://img.shields.io/badge/Sponsor-GitHub%20Sponsors-EA4AAA?style=for-the-badge&logo=github-sponsors&logoColor=white)](https://github.com/sponsors/Abhishek1033ubuntu)
[![Donate-PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white](https://img.shields.io/badge/Donate-PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/Abhishek1033ubuntu)
[![DOI](https://img.shields.io/badge/DOI-10.5281%2Fzenodo.23369012-blue?style=for-the-badge&logo=zenodo&logoColor=white)](https://doi.org/10.5281/zenodo.23369012)  


> Edge-native deterministic industrial automation platform featuring Causal SCM fault diagnosis, lock-free Rust concurrency, zero-downtime ONNX neural model hot-reloading, and sub-millisecond edge telemetry processing.

---

## Author Credentials & AI Collaboration Notice

* **Lead Author & Principal Developer:** Abhishek Singh
* **GitHub Profile:** [Abhishek1033ubuntu](https://github.com/Abhishek1033ubuntu)
* **ORCID Identifier:** [0009-0004-6913-096X](https://orcid.org/0009-0004-6913-096X)
* **Primary Contact:** abhishek1033@gmail.com | abhishek.singh.941491229013@proton.me | abhishek80@outlook.com
* **AI Architecture Collaboration:** Co-designed and engineered in collaboration with **Google Gemini**.

---
## Repository Structure

```
project-nest/
├── .github/
│   └── FUNDING.yml                  # GitHub Sponsors & PayPal funding config
├── api/
│   ├── asyncapi.yaml                # Real-time WebSocket diagnostic & 3D spatial schema
│   └── openapi.yaml                 # REST API schema for work-order status lifecycle
├── gui/
│   └── src/
│       └── PlantCadViewer.jsx       # React + Three.js 3D CAD plant fault viewer
├── rust_core/ (scm_core)
│   ├── Cargo.toml                   # Rust crate dependencies (PyO3, ort, arc-swap, sha2, ed25519-dalek, notify)
│   └── src/
│       ├── lib.rs                   # PyO3 binding module
│       ├── dag.rs                   # Directed Acyclic Graph & topological sorting engine
│       ├── scm.rs                   # Structural Causal Model & atomic lock-free hot-reloading
│       ├── security.rs              # SHA-256 & Ed25519 signature verifier
│       └── watcher.rs               # Notify file-system watcher for automatic ONNX updates
├── python/
│   ├── project_nest_causal/
│   │   ├── __init__.py
│   │   ├── engine.py                # High-level pipeline controller
│   │   ├── cmms_bridge.py           # SAP S/4HANA & IBM Maximo automated requisition driver
│   │   └── discovery.py             # NOTEARS data-driven causal graph learning
├── simulation/
│   └── opc_ua_hil_simulator.py      # OPC-UA hardware-in-the-loop server with fault injection
├── tests/
│   └── test_integration.py          # PyTest end-to-end integration suite
├── deploy_edge.sh                   # Air-gapped deployment script
├── docker-compose.edge.yml          # Local edge appliance container stack
├── LICENSE                          # Apache License 2.0
└── README.md                        # Primary documentation with author metadata & badges
```

## [How to use the repository](docs/HOWTOUSE.md)

---
## Support & Sponsorship

If `project-nest` accelerates your industrial automation workflows, research, or enterprise deployment, consider supporting ongoing independent development:

* **GitHub Sponsors:** [Sponsor @Abhishek1033ubuntu](https://github.com/sponsors/Abhishek1033ubuntu)
* **Direct PayPal Contribution:** [paypal.me/abhishek1033](https://www.paypal.me/Abhishek1033ubuntu)

---

## Intellectual Property & License Notice

Copyright (c) 2026 **Abhishek Singh**.

This software repository is published under the **Apache License 2.0**. Full legal text is available in the [LICENSE](LICENSE) file. 

*Note: Technical patent documentation, underlying mathematical claim disclosures, and hardware integration blueprints are preserved locally under private author ownership.*
