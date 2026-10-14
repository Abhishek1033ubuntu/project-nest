use crate::dag::DAG;
use std::collections::HashMap;

pub struct StructuralEquation {
    pub parents: Vec<String>,
    pub weights: Vec<f64>,
    pub bias: f64,
}

impl StructuralEquation {
    pub fn evaluate(&self, parent_values: &HashMap<String, f64>) -> f64 {
        let mut val = self.bias;
        for (parent, weight) in self.parents.iter().zip(self.weights.iter()) {
            let parent_val = parent_values.get(parent).copied().unwrap_or(0.0);
            val += weight * parent_val;
        }
        val
    }
}

pub struct SCMEngine {
    pub dag: DAG,
    pub equations: HashMap<String, StructuralEquation>,
}

#[derive(Debug, Clone)]
pub struct DiagnosticResult {
    pub root_cause: String,
    pub residual: f64,
    pub structural_residuals: HashMap<String, f64>,
}

impl SCMEngine {
    pub fn new(dag: DAG) -> Self {
        SCMEngine {
            dag,
            equations: HashMap::new(),
        }
    }

    pub fn register_equation(&mut self, node: &str, parents: Vec<String>, weights: Vec<f64>, bias: f64) {
        self.equations.insert(
            node.to_string(),
            StructuralEquation { parents, weights, bias },
        );
    }

    /// Evaluates expected forward propagation through the SCM
    pub fn predict(&self, inputs: &HashMap<String, f64>) -> HashMap<String, f64> {
        let order = self.dag.topological_sort();
        let mut state = inputs.clone();

        for node in order {
            if let Some(eq) = self.equations.get(&node) {
                // If value isn't an exogenous root input, compute from parents
                if !inputs.contains_key(&node) {
                    let pred = eq.evaluate(&state);
                    state.insert(node, pred);
                }
            }
        }
        state
    }

    /// Identifies the root cause node with the largest structural shift relative to parents
    pub fn diagnose_fault(&self, telemetry: &HashMap<String, f64>) -> DiagnosticResult {
        let mut residuals = HashMap::new();
        let mut max_residual = -1.0;
        let mut root_cause = String::from("UNKNOWN");

        for node in &self.dag.nodes {
            if let Some(eq) = self.equations.get(node) {
                let expected = eq.evaluate(telemetry);
                let actual = telemetry.get(node).copied().unwrap_or(expected);
                let residual = (actual - expected).abs();

                residuals.insert(node.clone(), residual);

                if residual > max_residual {
                    max_residual = residual;
                    root_cause = node.clone();
                }
            }
        }

        DiagnosticResult {
            root_cause,
            residual: max_residual,
            structural_residuals: residuals,
        }
    }
}
