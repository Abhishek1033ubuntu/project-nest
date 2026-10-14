use pyo3::prelude::*;
use std::collections::HashMap;

mod dag;
mod scm;

use dag::DAG;
use scm::SCMEngine;

#[pyclass]
pub struct PySCMEngine {
    engine: SCMEngine,
}

#[pymethods]
impl PySCMEngine {
    #[new]
    fn new() -> Self {
        PySCMEngine {
            engine: SCMEngine::new(DAG::new()),
        }
    }

    fn add_edge(&mut self, from: &str, to: &str) -> PyResult<()> {
        self.engine
            .dag
            .add_edge(from, to)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }

    fn register_linear_equation(&mut self, node: &str, parents: Vec<String>, weights: Vec<f64>, bias: f64) {
        self.engine.register_linear_equation(node, parents, weights, bias);
    }

    fn register_onnx_equation(&mut self, node: &str, parents: Vec<String>, model_path: &str) -> PyResult<()> {
        self.engine
            .register_onnx_equation(node, parents, model_path)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e))
    }

    fn diagnose(&self, telemetry: HashMap<String, f64>) -> (String, f64, HashMap<String, f64>) {
        self.engine.diagnose_fault(&telemetry)
    }
}

#[pymodule]
fn scm_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySCMEngine>()?;
    Ok(())
}
