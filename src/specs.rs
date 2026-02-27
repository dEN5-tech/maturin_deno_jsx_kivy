use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, instrument};

/// DECLARATIVE_INTERFACE_MAPPING_SPEC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclarativeNode {
    pub kind: String,
    pub props: serde_json::Value,
    pub children: Vec<DeclarativeNode>,
}

impl DeclarativeNode {
    pub fn new(kind: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            props: serde_json::json!({}),
            children: vec![],
        }
    }

    pub fn with_props(mut self, props: serde_json::Value) -> Self {
        self.props = props;
        self
    }

    pub fn with_children(mut self, children: Vec<DeclarativeNode>) -> Self {
        self.children = children;
        self
    }
}

/// ABSTRACT_SYNTAX_EVALUATION_SPEC result shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstEvaluationResult {
    pub root: DeclarativeNode,
    pub diagnostics: Vec<String>,
}

impl AstEvaluationResult {
    pub fn new(root: DeclarativeNode, diagnostics: Vec<String>) -> Self {
        Self { root, diagnostics }
    }
}

#[derive(Debug, Error)]
pub enum RuntimeResearchError {
    #[error("evaluation failed: {0}")]
    EvaluationFailed(String),
    #[error("dynamic component resolution failed for '{0}'")]
    ComponentResolutionFailed(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("serialization error: {0}")]
    Serialization(String),
}

/// ABSTRACT_SYNTAX_EVALUATION_SPEC boundary.
#[async_trait]
pub trait JsxExpressionEvaluator: Send + Sync {
    async fn eval_jsx_expression(
        &self,
        source: &str,
    ) -> Result<AstEvaluationResult, RuntimeResearchError>;
}

/// Minimal evaluator used for research scaffolding and architecture validation.
pub struct NoopJsxEvaluator;

#[async_trait]
impl JsxExpressionEvaluator for NoopJsxEvaluator {
    #[instrument(skip(self, source), fields(source_len = source.len()))]
    async fn eval_jsx_expression(
        &self,
        source: &str,
    ) -> Result<AstEvaluationResult, RuntimeResearchError> {
        debug!("running placeholder _eval_jsx_expression pipeline");

        if source.trim().is_empty() {
            return Err(RuntimeResearchError::InvalidInput(
                "empty JSX/JS payload".to_string(),
            ));
        }

        let source_hint = if source.contains("Kivy JSX TODO") {
            "BUNDLE_TODO_UI_DETECTED"
        } else {
            "NO_BUNDLE_SIGNATURE_DETECTED"
        };

        let root = DeclarativeNode {
            kind: "View".to_string(),
            props: serde_json::json!({
                "orientation": "vertical",
                "padding": [20, 20, 20, 20],
                "spacing": 12
            }),
            children: vec![
                DeclarativeNode {
                    kind: "Label".to_string(),
                    props: serde_json::json!({
                        "text": "Kivy JSX TODO",
                        "font_size": 28,
                        "color": [0.2, 0.9, 1.0, 1.0]
                    }),
                    children: vec![],
                },
                DeclarativeNode {
                    kind: "Label".to_string(),
                    props: serde_json::json!({
                        "text": format!("Runtime status: {}", source_hint),
                        "font_size": 14,
                        "color": [0.7, 0.7, 0.7, 1.0]
                    }),
                    children: vec![],
                },
                DeclarativeNode {
                    kind: "Button".to_string(),
                    props: serde_json::json!({
                        "text": "[x] Set up Rust + Python bridge"
                    }),
                    children: vec![],
                },
                DeclarativeNode {
                    kind: "Button".to_string(),
                    props: serde_json::json!({
                        "text": "[x] Bundle React UI into app.bundle.js"
                    }),
                    children: vec![],
                },
                DeclarativeNode {
                    kind: "Button".to_string(),
                    props: serde_json::json!({
                        "text": "[ ] Show TODO state in Kivy window"
                    }),
                    children: vec![],
                },
            ],
        };

        Ok(AstEvaluationResult::new(
            root,
            vec!["NOOP_EVALUATOR_ACTIVE".to_string()],
        ))
    }
}
