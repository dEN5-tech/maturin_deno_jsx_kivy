use crate::specs::{DeclarativeNode, RuntimeResearchError};
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

/// DYNAMIC_COMPONENT_RESOLUTION_SPEC output for Python/Kivy side.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetSpec {
    pub widget_type: String,
    pub props: serde_json::Value,
    pub children: Vec<WidgetSpec>,
}

impl WidgetSpec {
    pub fn new(widget_type: impl Into<String>) -> Self {
        Self {
            widget_type: widget_type.into(),
            props: serde_json::json!({}),
            children: vec![],
        }
    }

    pub fn with_props(mut self, props: serde_json::Value) -> Self {
        self.props = props;
        self
    }

    pub fn with_children(mut self, children: Vec<WidgetSpec>) -> Self {
        self.children = children;
        self
    }

    pub fn validate(&self) -> Result<(), RuntimeResearchError> {
        if self.widget_type.trim().is_empty() {
            return Err(RuntimeResearchError::InvalidInput(
                "widget_type must not be empty".to_string(),
            ));
        }

        if !self.props.is_object() {
            return Err(RuntimeResearchError::InvalidInput(
                "widget props must be a JSON object".to_string(),
            ));
        }

        for child in &self.children {
            child.validate()?;
        }

        Ok(())
    }

    pub fn to_json(&self) -> Result<String, RuntimeResearchError> {
        self.validate()?;
        serde_json::to_string(self)
            .map_err(|e| RuntimeResearchError::Serialization(e.to_string()))
    }
}

impl TryFrom<&str> for WidgetSpec {
    type Error = RuntimeResearchError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed: WidgetSpec =
            serde_json::from_str(value).map_err(|e| RuntimeResearchError::Serialization(e.to_string()))?;
        parsed.validate()?;
        Ok(parsed)
    }
}

pub trait DynamicComponentResolver: Send + Sync {
    fn create_custom_widget(
        &self,
        node: &DeclarativeNode,
    ) -> Result<WidgetSpec, RuntimeResearchError>;
}

/// Research-mode resolver with small static mapping.
pub struct NoopKivyResolver;

impl NoopKivyResolver {
    fn map_tag(tag: &str) -> &str {
        match tag {
            "View" => "BoxLayout",
            "Label" => "Label",
            "Button" => "Button",
            other => other,
        }
    }
}

impl DynamicComponentResolver for NoopKivyResolver {
    #[instrument(skip(self, node), fields(kind = %node.kind))]
    fn create_custom_widget(
        &self,
        node: &DeclarativeNode,
    ) -> Result<WidgetSpec, RuntimeResearchError> {
        debug!("running placeholder _create_custom_widget mapping");

        let children = node
            .children
            .iter()
            .map(|child| self.create_custom_widget(child))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(WidgetSpec {
            widget_type: Self::map_tag(&node.kind).to_string(),
            props: node.props.clone(),
            children,
        })
    }
}
