#![cfg(feature = "python-bridge")]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::{NoopJsxEvaluator, NoopKivyResolver, ResearchRuntimeEngine, RuntimeConfig};
use pyo3::prelude::*;
use tokio::runtime::Builder;

#[pyclass]
pub struct JSXRuntimeEngine;

#[pymethods]
impl JSXRuntimeEngine {
    #[new]
    pub fn new() -> Self {
        Self
    }

    /// Accepts bundled JavaScript/JSX payload text and returns a widget-spec JSON string.
    pub fn mount_application(&self, js_payload: String) -> PyResult<String> {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

        let result: Result<String, _> = runtime.block_on(async {
            let engine = ResearchRuntimeEngine::new(
                NoopJsxEvaluator,
                NoopKivyResolver,
                RuntimeConfig {
                    enable_tracing: true,
                    research_profile: "python-bridge".to_string(),
                },
            );

            engine.evaluate_and_resolve_json(&js_payload).await
        });

        result.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }
}

#[pymodule]
pub fn kivy_jsx_runtime(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<JSXRuntimeEngine>()?;
    Ok(())
}
