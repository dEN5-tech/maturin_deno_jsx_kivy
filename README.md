# maturin_deno_jsx_kivy (research scaffold)

This repository now contains a **research-first Rust architecture scaffold** for the concepts you asked about:

- `DECLARATIVE_INTERFACE_MAPPING_SPEC` (JSX-like node -> Kivy-like widget spec)
- `ABSTRACT_SYNTAX_EVALUATION_SPEC` (`_eval_jsx_expression` boundary)
- `DYNAMIC_COMPONENT_RESOLUTION_SPEC` (`_create_custom_widget` boundary)
- `POLYGLOT_RUNTIME_INTEGRATION_SPEC` (foundation hooks for `deno_core` + `pyo3`)
- `RUNTIME_OBSERVABILITY_SPEC` (`tracing` + `tracing-subscriber`)

## Current module map

- `src/specs.rs`
  - `DeclarativeNode`
  - `AstEvaluationResult`
  - `RuntimeResearchError`
  - `JsxExpressionEvaluator` trait
  - `NoopJsxEvaluator` placeholder implementation

- `src/bridge.rs`
  - `WidgetSpec`
  - `DynamicComponentResolver` trait
  - `NoopKivyResolver` placeholder mapping (`View -> BoxLayout`, etc.)

- `src/runtime.rs`
  - `RuntimeConfig`
  - `ResearchRuntimeEngine` orchestrator

- `src/main.rs`
  - async demo entrypoint with tracing init
  - executes evaluate -> resolve flow and prints resulting widget tree JSON

## Cargo features (foundation only)

- `deno-runtime` enables optional `deno_core`
- `python-bridge` enables optional `pyo3`

These are intentionally optional so the scaffold remains buildable while you iterate on integration design.

## Run

```bash
cargo run
```

You should see tracing logs plus a JSON output of a resolved widget tree.

## Build Python wheel (maturin)

```bash
maturin build --release
```

Wheel artifacts will be created in `target/wheels/`.

Install one wheel locally:

```bash
pip install target/wheels/kivy_jsx_runtime-0.1.0-*.whl
```

## Consume bundled React output from Python

```python
import os
from kivy_jsx_runtime import JSXRuntimeEngine

engine = JSXRuntimeEngine()
bundle_path = os.path.join(os.getcwd(), "dist", "app.bundle.js")

with open(bundle_path, "r", encoding="utf-8") as f:
    js_payload = f.read()

widget_tree_json = engine.mount_application(js_payload)
print(widget_tree_json)
```

This demonstrates the intended pipeline:

1. React build outputs `dist/app.bundle.js`
2. Python loads bundle text
3. Rust bridge evaluates + resolves to widget-spec JSON
4. Python/Kivy layer can map the JSON tree to native widgets
# maturin_deno_jsx_kivy 
