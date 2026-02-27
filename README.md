# maturin_deno_jsx_kivy

Bridging **React-style declarative UI ideas** and **native Kivy widgets** through a Rust core (PyO3 + maturin).

> Status: research/prototype with a production-oriented public API surface in progress.

---

## Why this project exists (for the Kivy community)

Kivy developers often want:

- modern declarative component authoring,
- stronger tooling/type support,
- and clean interop with Python-native rendering.

This project explores exactly that: a pipeline where a JS/TS bundle is evaluated by a Rust runtime, converted into a widget spec tree, and materialized as real Kivy widgets in Python.

The long-term goal is **better developer ergonomics without losing Kivy-native rendering and control**.

---

## High-level architecture

1. **UI authoring layer (React/TS + react-reconciler)**
   - Located in `ui/`
   - Produces `ui/dist/app.bundle.js`

2. **Rust runtime layer (`kivy_jsx_runtime`)**
   - Evaluates payload and produces a normalized widget specification tree
   - Exposes a Python module via PyO3

3. **Python + Kivy bridge layer (`main.py`)**
   - Loads bundle
   - Calls Rust engine (`JSXRuntimeEngine.mount_application`)
   - Dynamically resolves Kivy classes and mounts widgets

---

## Repository layout

- `src/specs.rs` — declarative AST/result/error contracts
- `src/bridge.rs` — `WidgetSpec`, validation, JSON conversions, resolver trait
- `src/runtime.rs` — orchestration API (`evaluate_to_ast`, `evaluate_and_resolve`, JSON helpers)
- `src/python_bridge.rs` — PyO3 module/class exposed to Python
- `ui/` — Vite + React + reconciler bundle source
- `tools/gen_definitions.py` — AST scanner that generates `ui/src/global.d.ts` from installed Kivy
- `main.py` — runtime Kivy app launcher using autonomous widget discovery

---

## Prerequisites (Windows)

- Rust toolchain
- Python 3.12 (recommended for Kivy on Windows)
- `pnpm`
- Kivy dependencies installable in your Python environment

---

## Quick start

### 1) Build UI bundle

```bash
pnpm --dir c:\projects\maturin_deno_jsx_kivy\ui install
pnpm --dir c:\projects\maturin_deno_jsx_kivy\ui build
```

### 2) Build Python wheel from Rust

```bash
python -m maturin build --release
```

### 3) Install into project venv

```bash
c:\projects\maturin_deno_jsx_kivy\.venv312\Scripts\python.exe -m pip install --force-reinstall c:\projects\maturin_deno_jsx_kivy\target\wheels\kivy_jsx_runtime-0.1.0-cp39-abi3-win_amd64.whl
```

### 4) Run Kivy app

```bash
c:\projects\maturin_deno_jsx_kivy\.venv312\Scripts\python.exe c:\projects\maturin_deno_jsx_kivy\main.py
```

Optional auto-exit for smoke tests:

```bash
set APP_AUTO_EXIT_SECONDS=5 && c:\projects\maturin_deno_jsx_kivy\.venv312\Scripts\python.exe c:\projects\maturin_deno_jsx_kivy\main.py
```

---

## Type generation for better TS DX

The project auto-generates `ui/src/global.d.ts` from installed Kivy source using Python AST scanning:

```bash
pnpm --dir c:\projects\maturin_deno_jsx_kivy\ui run gen:types
```

This improves IntelliSense for JSX-like Kivy tags and props.

---

## Current limitations

- JSX/JS payload evaluation is still a placeholder strategy (research mode).
- Event callback semantics across JS ↔ Rust ↔ Python are not production-complete yet.
- The dynamic resolver assumes module/class naming conventions for many widgets.

---

## Contributing

Issues and PRs are welcome—especially from Kivy developers interested in:

- reconciler host config design,
- dynamic widget/event binding,
- perf profiling across FFI boundaries,
- and robust production API hardening.

---

## License

MIT (recommended; add a `LICENSE` file if you want this explicitly enforced in the repo).
