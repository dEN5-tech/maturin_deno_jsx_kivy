//! Research-oriented runtime scaffold for JSX -> Kivy orchestration.
//!
//! This crate intentionally focuses on architecture boundaries and tracing-first
//! observability, not a complete production integration.

pub mod bridge;
#[cfg(feature = "python-bridge")]
pub mod python_bridge;
pub mod runtime;
pub mod specs;

pub use bridge::{DynamicComponentResolver, NoopKivyResolver, WidgetSpec};
pub use runtime::{ResearchRuntimeEngine, RuntimeConfig};
pub use specs::{
    AstEvaluationResult, DeclarativeNode, JsxExpressionEvaluator, NoopJsxEvaluator,
    RuntimeResearchError,
};
