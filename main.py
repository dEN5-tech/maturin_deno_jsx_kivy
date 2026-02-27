from __future__ import annotations

import importlib
import json
import os
from pathlib import Path

from kivy.app import App
from kivy.clock import Clock
from kivy.uix.boxlayout import BoxLayout
from kivy.uix.button import Button
from kivy.uix.label import Label
from kivy.uix.widget import Widget
from kivy_jsx_runtime import JSXRuntimeEngine


def _coerce_props(props: dict) -> dict:
    safe = dict(props)
    if "on_press" in safe:
        safe.pop("on_press")
    return safe


class AutonomousKivyBridge:
    """AUTONOMOUS_MODULE_DISCOVERY_PROTOCOL for Kivy widget creation."""

    def __init__(self) -> None:
        self.engine = JSXRuntimeEngine()
        self.widget_cache: dict[str, type[Widget]] = {}

    def resolve_widget_class(self, widget_name: str) -> type[Widget]:
        if widget_name in self.widget_cache:
            return self.widget_cache[widget_name]

        package_name = f"kivy.uix.{widget_name.lower()}"
        try:
            module = importlib.import_module(package_name)
            widget_class = getattr(module, widget_name)
        except (ImportError, AttributeError) as exc:
            raise NotImplementedError(
                f"UNRESOLVED_WIDGET: {widget_name} at {package_name}"
            ) from exc

        self.widget_cache[widget_name] = widget_class
        return widget_class

    def resolve_and_instantiate(self, widget_name: str, **props: object) -> Widget:
        widget_class = self.resolve_widget_class(widget_name)
        return widget_class(**_coerce_props(props))

    def _build_widget_tree(self, node: dict) -> Widget:
        widget_type = node.get("widget_type", "Label")
        props = node.get("props", {})
        widget = self.resolve_and_instantiate(widget_type, **props)

        for child in node.get("children", []):
            if hasattr(widget, "add_widget"):
                widget.add_widget(self._build_widget_tree(child))

        return widget

    def sync_ui_tree(self, js_payload: str) -> Widget:
        widget_tree_json = self.engine.mount_application(js_payload)
        widget_tree = json.loads(widget_tree_json)
        return self._build_widget_tree(widget_tree)


def _fallback_static_build(node: dict) -> Widget:
    """Fallback kept for tags that are not direct module/class matches."""
    widget_type = node.get("widget_type", "Label")
    props = _coerce_props(node.get("props", {}))

    if widget_type == "BoxLayout":
        widget: Widget = BoxLayout(**props)
    elif widget_type == "Button":
        widget = Button(**props)
    else:
        widget = Label(**props)

    for child in node.get("children", []):
        if hasattr(widget, "add_widget"):
            widget.add_widget(_fallback_static_build(child))

    return widget


def main() -> None:
    bridge = AutonomousKivyBridge()

    bundle_path = Path(os.getcwd()) / "ui" / "dist" / "app.bundle.js"
    if not bundle_path.exists():
        raise FileNotFoundError(
            f"Bundle not found: {bundle_path}\n"
            "Build it first with: pnpm --dir c:\\projects\\maturin_deno_jsx_kivy\\ui build"
        )

    js_payload = bundle_path.read_text(encoding="utf-8")

    class RuntimeKivyApp(App):
        def build(self) -> Widget:
            auto_exit = os.getenv("APP_AUTO_EXIT_SECONDS")
            if auto_exit:
                Clock.schedule_once(lambda *_: self.stop(), float(auto_exit))

            try:
                return bridge.sync_ui_tree(js_payload)
            except NotImplementedError:
                # Fallback path for non-conventional tags.
                widget_tree_json = bridge.engine.mount_application(js_payload)
                widget_tree = json.loads(widget_tree_json)
                return _fallback_static_build(widget_tree)

    RuntimeKivyApp().run()


if __name__ == "__main__":
    main()
