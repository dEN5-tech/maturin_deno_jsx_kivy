use kivy_jsx_runtime::{
    NoopJsxEvaluator, NoopKivyResolver, ResearchRuntimeEngine, RuntimeConfig,
};
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .try_init();

    let engine = ResearchRuntimeEngine::new(
        NoopJsxEvaluator,
        NoopKivyResolver,
        RuntimeConfig {
            enable_tracing: true,
            research_profile: "declarative-ui-mapping".to_string(),
        },
    );

    let input = r#"<View orientation='vertical'><Label text='hello from jsx'/></View>"#;
    let widget_tree = engine.evaluate_and_resolve(input).await?;

    println!("Resolved widget tree: {}", serde_json::to_string_pretty(&widget_tree)?);
    Ok(())
}
