use pyo3::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

mod dag;
mod scm;
mod security;
mod watcher;

use scm::SCMEngine;
use security::ModelSecurityVerifier;
use watcher::ModelWatcher;

#[pyclass]
pub struct PySCMEngine {
    engine: Arc<SCMEngine>,
    verifier: Option<ModelSecurityVerifier>,
    _watcher: Option<ModelWatcher>,
}

#[pymethods]
impl PySCMEngine {
    #[new]
    fn new() -> Self {
        PySCMEngine {
            engine: Arc::new(SCMEngine::new()),
            verifier: None,
            _watcher: None,
        }
    }

    fn add_edge(&mut self, from: &str, to: &str) -> PyResult<()> {
        let mut_engine = Arc::get_mut(&mut self.engine)
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Cannot mutate DAG while engine is shared"))?;
        mut_engine.dag.add_edge(from, to).map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }

    fn register_linear_equation(&mut self, node: &str, parents: Vec<String>, weights: Vec<f64>, bias: f64) -> PyResult<()> {
        let mut_engine = Arc::get_mut(&mut self.engine)
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Cannot mutate engine while shared"))?;
        mut_engine.register_linear_equation(node, parents, weights, bias);
        Ok(())
    }

    fn register_onnx_equation(&mut self, node: &str, parents: Vec<String>, model_path: &str) -> PyResult<()> {
        let mut_engine = Arc::get_mut(&mut self.engine)
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Cannot mutate engine while shared"))?;
        mut_engine.register_onnx_equation(node, parents, model_path).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e))
    }

    fn set_trusted_public_key(&mut self, public_key_hex: &str) -> PyResult<()> {
        let verifier = ModelSecurityVerifier::new(public_key_hex).map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        self.verifier = Some(verifier);
        Ok(())
    }

    fn reload_model_verified(&self, node: &str, model_path: &str, sha256_hex: &str, signature_hex: &str) -> PyResult<()> {
        let verifier = self.verifier.as_ref().ok_or_else(|| {
            pyo3::exceptions::PyRuntimeError::new_err("Trusted public key not set.")
        })?;

        if let Some(eq) = self.engine.equations.get(node) {
            eq.reload_onnx_verified(Path::new(model_path), sha256_hex, signature_hex, verifier)
                .map_err(|e| pyo3::exceptions::PySecurityError::new_err(e))
        } else {
            Err(pyo3::exceptions::PyKeyError::new_err(format!("Node {} not found", node)))
        }
    }

    fn enable_auto_reload(&mut self, node_file_mappings: HashMap<String, String>) -> PyResult<()> {
        let watcher = ModelWatcher::new(Arc::clone(&self.engine), node_file_mappings)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e))?;
        self._watcher = Some(watcher);
        Ok(())
    }

    fn evaluate_node(&self, node: &str, telemetry: HashMap<String, f64>) -> PyResult<f64> {
        self.engine.evaluate_node(node, &telemetry).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e))
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
