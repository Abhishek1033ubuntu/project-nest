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
        self.engine.register_equation(node, parents, weights, bias);
    }

    fn diagnose(&self, telemetry: HashMap<String, f64>) -> (String, f64, HashMap<String, f64>) {
        let res = self.engine.diagnose_fault(&telemetry);
        (res.root_cause, res.residual, res.structural_residuals)
    }
}

#[pymodule]
fn scm_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySCMEngine>()?;
    Ok(())
}
