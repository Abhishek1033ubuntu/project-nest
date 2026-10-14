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
