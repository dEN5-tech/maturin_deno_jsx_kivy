use crate::bridge::{DynamicComponentResolver, WidgetSpec};
use crate::specs::{AstEvaluationResult, JsxExpressionEvaluator, RuntimeResearchError};
use tracing::{info, instrument};

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub enable_tracing: bool,
    pub research_profile: String,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            enable_tracing: true,
            research_profile: "baseline".to_string(),
        }
    }
}

/// POLYGLOT_RUNTIME_INTEGRATION_SPEC scaffold.
pub struct ResearchRuntimeEngine<E, R>
where
    E: JsxExpressionEvaluator,
    R: DynamicComponentResolver,
{
    evaluator: E,
    resolver: R,
    config: RuntimeConfig,
}

impl<E, R> ResearchRuntimeEngine<E, R>
where
    E: JsxExpressionEvaluator,
    R: DynamicComponentResolver,
{
    pub fn new(evaluator: E, resolver: R, config: RuntimeConfig) -> Self {
        Self {
            evaluator,
            resolver,
            config,
        }
    }

    #[instrument(skip(self, jsx_source), fields(profile = %self.config.research_profile))]
    pub async fn evaluate_and_resolve(
        &self,
        jsx_source: &str,
    ) -> Result<WidgetSpec, RuntimeResearchError> {
        if jsx_source.trim().is_empty() {
            return Err(RuntimeResearchError::InvalidInput(
                "jsx_source must not be empty".to_string(),
            ));
        }

        info!("starting ABSTRACT_SYNTAX_EVALUATION_SPEC");
        let ast = self.evaluator.eval_jsx_expression(jsx_source).await?;

        info!(diagnostics = ?ast.diagnostics, "starting DYNAMIC_COMPONENT_RESOLUTION_SPEC");
        let widget = self.resolver.create_custom_widget(&ast.root)?;
        widget.validate()?;
        Ok(widget)
    }

    #[instrument(skip(self, jsx_source), fields(profile = %self.config.research_profile))]
    pub async fn evaluate_to_ast(
        &self,
        jsx_source: &str,
    ) -> Result<AstEvaluationResult, RuntimeResearchError> {
        if jsx_source.trim().is_empty() {
            return Err(RuntimeResearchError::InvalidInput(
                "jsx_source must not be empty".to_string(),
            ));
        }
        self.evaluator.eval_jsx_expression(jsx_source).await
    }

    pub async fn evaluate_and_resolve_json(
        &self,
        jsx_source: &str,
    ) -> Result<String, RuntimeResearchError> {
        let widget = self.evaluate_and_resolve(jsx_source).await?;
        widget.to_json()
    }
}
