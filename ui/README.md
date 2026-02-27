# ui (pnpm + vite + react reconciler)

Minimal, template-free frontend bundle project that outputs a single IIFE file for Rust/Python ingestion.

## Install

```bash
pnpm install
```

## Build

```bash
pnpm build
```

Output artifact:

- `ui/dist/app.bundle.js`

Use this file in Python and pass it to `JSXRuntimeEngine.mount_application(...)`.
