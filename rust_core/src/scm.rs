use crate::dag::DAG;
use ndarray::Array2;
use ort::{GraphOptimizationLevel, Session};
use std::collections::HashMap;
use std::sync::Arc;

pub enum EquationEvaluator {
    Linear {
        parents: Vec<String>,
        weights: Vec<f64>,
        bias: f64,
    },
    ONNX {
        parents: Vec<String>,
        session: Arc<Session>,
    },
}

impl EquationEvaluator {
    pub fn parents(&self) -> &[String] {
        match self {
            EquationEvaluator::Linear { parents, .. } => parents,
            EquationEvaluator::ONNX { parents, .. } => parents,
        }
    }

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
                // 1. Construct 1xN input tensor from ordered parent variables
                let input_vec: Vec<f32> = parents
                    .iter()
                    .map(|p| parent_values.get(p).copied().unwrap_or(0.0) as f32)
                    .collect();

                let input_tensor = Array2::from_shape_vec((1, parents.len()), input_vec)
                    .map_err(|e| format!("Failed to create tensor shape: {}", e))?;

                // 2. Execute ONNX session inference
                let inputs = ort::inputs!["input" => input_tensor]
                    .map_err(|e| format!("ONNX input binding error: {}", e))?;

                let outputs = session
                    .run(inputs)
                    .map_err(|e| format!("ONNX runtime execution error: {}", e))?;

                // 3. Extract scalar prediction output
                let output_tensor = outputs["output"]
                    .extract_tensor::<f32>()
                    .map_err(|e| format!("Tensor extraction error: {}", e))?;

                let predicted_val = output_tensor.as_slice().ok_or("Empty output tensor")?[0];
                Ok(predicted_val as f64)
            }
        }
    }
}

pub struct SCMEngine {
    pub dag: DAG,
    pub equations: HashMap<String, EquationEvaluator>,
}

impl SCMEngine {
    pub fn new(dag: DAG) -> Self {
        SCMEngine {
            dag,
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
            .map_err(|e| format!("Failed to load ONNX model from {}: {}", model_path, e))?;

        self.equations.insert(
            node.to_string(),
            EquationEvaluator::ONNX {
                parents,
                session: Arc::new(session),
            },
        );
        Ok(())
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
