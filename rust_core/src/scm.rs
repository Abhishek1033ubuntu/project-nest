use arc_swap::ArcSwap;
use ndarray::Array2;
use ort::{GraphOptimizationLevel, Session};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use crate::dag::DAG;
use crate::security::ModelSecurityVerifier;

pub enum EquationEvaluator {
    Linear {
        parents: Vec<String>,
        weights: Vec<f64>,
        bias: f64,
    },
    ONNX {
        parents: Vec<String>,
        session: ArcSwap<Session>,
    },
}

impl EquationEvaluator {
    pub fn evaluate(&self, parent_values: &HashMap<String, f64>) -> Result<f64, String> {
        match self {
            EquationEvaluator::Linear { parents, weights, bias } => {
                let mut val = *bias;
                for (parent, weight) in parents.iter().zip(weights.iter()) {
                    let parent_val = parent_values.get(parent).copied().unwrap_or(0.0);
                    val += weight * parent_val;
                }
                Ok(val)
            }
            EquationEvaluator::ONNX { parents, session } => {
                let current_session = session.load();

                let input_vec: Vec<f32> = parents
                    .iter()
                    .map(|p| parent_values.get(p).copied().unwrap_or(0.0) as f32)
                    .collect();

                let input_tensor = Array2::from_shape_vec((1, parents.len()), input_vec)
                    .map_err(|e| format!("Tensor allocation error: {}", e))?;

                let inputs = ort::inputs!["input" => input_tensor]
                    .map_err(|e| format!("ONNX input error: {}", e))?;

                let outputs = current_session
                    .run(inputs)
                    .map_err(|e| format!("ONNX evaluation error: {}", e))?;

                let output_tensor = outputs["output"]
                    .extract_tensor::<f32>()
                    .map_err(|e| format!("Tensor extraction error: {}", e))?;

                Ok(output_tensor.as_slice().ok_or("Empty tensor")?[0] as f64)
            }
        }
    }

    pub fn reload_onnx_verified(
        &self,
        file_path: &Path,
        expected_sha256: &str,
        signature_hex: &str,
        verifier: &ModelSecurityVerifier,
    ) -> Result<(), String> {
        match self {
            EquationEvaluator::ONNX { session, .. } => {
                let verified_bytes = verifier.verify_onnx_binary(
                    file_path,
                    expected_sha256,
                    signature_hex,
                )?;

                let new_session = Session::builder()
                    .map_err(|e| e.to_string())?
                    .with_optimization_level(GraphOptimizationLevel::Level3)
                    .map_err(|e| e.to_string())?
                    .commit_from_memory(&verified_bytes)
                    .map_err(|e| format!("Failed to compile verified ONNX model: {}", e))?;

                session.store(Arc::new(new_session));
                Ok(())
            }
            EquationEvaluator::Linear { .. } => {
                Err("Node is configured as linear, not ONNX.".to_string())
            }
        }
    }

    pub fn reload_onnx_unverified(&self, file_path: &Path) -> Result<(), String> {
        match self {
            EquationEvaluator::ONNX { session, .. } => {
                let new_session = Session::builder()
                    .map_err(|e| e.to_string())?
                    .with_optimization_level(GraphOptimizationLevel::Level3)
                    .map_err(|e| e.to_string())?
                    .commit_from_file(file_path)
                    .map_err(|e| format!("Failed to load ONNX model: {}", e))?;

                session.store(Arc::new(new_session));
                Ok(())
            }
            EquationEvaluator::Linear { .. } => Err("Node is linear".to_string()),
        }
    }
}

pub struct SCMEngine {
    pub dag: DAG,
    pub equations: HashMap<String, EquationEvaluator>,
}

impl SCMEngine {
    pub fn new() -> Self {
        SCMEngine {
            dag: DAG::new(),
            equations: HashMap::new(),
        }
    }

    pub fn register_linear_equation(&mut self, node: &str, parents: Vec<String>, weights: Vec<f64>, bias: f64) {
        self.equations.insert(
            node.to_string(),
            EquationEvaluator::Linear { parents, weights, bias },
        );
    }

    pub fn register_onnx_equation(&mut self, node: &str, parents: Vec<String>, model_path: &str) -> Result<(), String> {
        let session = Session::builder()
            .map_err(|e| e.to_string())?
            .with_optimization_level(GraphOptimizationLevel::Level3)
            .map_err(|e| e.to_string())?
            .commit_from_file(model_path)
            .map_err(|e| format!("Failed to load ONNX model: {}", e))?;

        self.equations.insert(
            node.to_string(),
            EquationEvaluator::ONNX {
                parents,
                session: ArcSwap::from_pointee(session),
            },
        );
        Ok(())
    }

    pub fn reload_node_model(&self, node: &str, new_model_path: &str) -> Result<(), String> {
        if let Some(eq) = self.equations.get(node) {
            eq.reload_onnx_unverified(Path::new(new_model_path))
        } else {
            Err(format!("Node {} not found", node))
        }
    }

    pub fn evaluate_node(&self, node: &str, telemetry: &HashMap<String, f64>) -> Result<f64, String> {
        if let Some(eq) = self.equations.get(node) {
            eq.evaluate(telemetry)
        } else {
            Err(format!("Node {} not found", node))
        }
    }

    pub fn diagnose_fault(&self, telemetry: &HashMap<String, f64>) -> (String, f64, HashMap<String, f64>) {
        let mut residuals = HashMap::new();
        let mut max_residual = -1.0;
        let mut root_cause = String::from("UNKNOWN");

        for node in &self.dag.nodes {
            if let Some(eq) = self.equations.get(node) {
                let expected = eq.evaluate(telemetry).unwrap_or(0.0);
                let actual = telemetry.get(node).copied().unwrap_or(expected);
                let residual = (actual - expected).abs();

                residuals.insert(node.clone(), residual);

                if residual > max_residual {
                    max_residual = residual;
                    root_cause = node.clone();
                }
            }
        }

        (root_cause, max_residual, residuals)
    }
}
