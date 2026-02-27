from __future__ import annotations

import ast
from pathlib import Path
from typing import Dict, List, Tuple

import kivy


class KivyASTScanner:
    def __init__(self) -> None:
        self.uix_path = Path(kivy.__file__).parent / "uix"
        self.class_props: Dict[str, List[Tuple[str, str]]] = {}
        self.class_bases: Dict[str, List[str]] = {}
        self.definitions: Dict[str, List[Tuple[str, str]]] = {}

    @staticmethod
    def _property_call_name(node: ast.Call) -> str:
        if isinstance(node.func, ast.Name):
            return node.func.id
        if isinstance(node.func, ast.Attribute):
            return node.func.attr
        return ""

    def parse_module(self, file_path: Path) -> None:
        source = file_path.read_text(encoding="utf-8")
        try:
            tree = ast.parse(source)
        except SyntaxError:
            return

        for node in ast.walk(tree):
            if not isinstance(node, ast.ClassDef):
                continue

            class_name = node.name
            bases: List[str] = []
            for base in node.bases:
                if isinstance(base, ast.Name):
                    bases.append(base.id)
                elif isinstance(base, ast.Attribute):
                    bases.append(base.attr)
            self.class_bases[class_name] = bases

            props: List[Tuple[str, str]] = []

            for item in node.body:
                if not isinstance(item, ast.Assign) or not isinstance(item.value, ast.Call):
                    continue

                func_name = self._property_call_name(item.value)
                if "Property" not in func_name:
                    continue

                for target in item.targets:
                    if isinstance(target, ast.Name):
                        props.append((target.id, func_name))

            unique = sorted(set(props), key=lambda it: it[0])
            self.class_props[class_name] = unique

    def resolve_inherited_props(
        self,
        class_name: str,
        visiting: set[str] | None = None,
    ) -> List[Tuple[str, str]]:
        if class_name in self.definitions:
            return self.definitions[class_name]

        if visiting is None:
            visiting = set()
        if class_name in visiting:
            return self.class_props.get(class_name, [])

        visiting.add(class_name)
        merged: Dict[str, str] = {}

        for base_name in self.class_bases.get(class_name, []):
            for prop_name, prop_type in self.resolve_inherited_props(base_name, visiting):
                merged[prop_name] = prop_type

        for prop_name, prop_type in self.class_props.get(class_name, []):
            merged[prop_name] = prop_type

        resolved = sorted(merged.items(), key=lambda it: it[0])
        self.definitions[class_name] = resolved
        visiting.remove(class_name)
        return resolved

    @staticmethod
    def map_ts_type(prop_type: str) -> str:
        if "String" in prop_type:
            return "string"
        if "Numeric" in prop_type or "Bounded" in prop_type:
            return "number"
        if "Boolean" in prop_type:
            return "boolean"
        if "List" in prop_type or "ReferenceList" in prop_type:
            return "any[]"
        if "Dict" in prop_type:
            return "Record<string, unknown>"
        return "any"

    def generate_dts(self) -> str:
        lines = [
            "import type React from 'react';",
            "",
            "declare global {",
            "  namespace JSX {",
            "    interface IntrinsicElements {",
        ]

        for widget in sorted(self.definitions):
            lines.append(f"      {widget}: {{")
            for prop_name, prop_type in self.definitions[widget]:
                lines.append(f"        {prop_name}?: {self.map_ts_type(prop_type)};")
            lines.append("        on_press?: () => void;")
            lines.append("        children?: React.ReactNode;")
            lines.append("      };")

        lines.extend(
            [
                "    }",
                "  }",
                "",
                "  interface Window {",
                "    Kivy: {",
                "      instantiateWidget: (id: number, type: string, props: Record<string, unknown>) => void;",
                "      terminateWidget: (id: number) => void;",
                "      applyPropertyUpdate: (id: number, props: Record<string, unknown>) => void;",
                "      bindHierarchy: (parentId: number, childId: number) => void;",
                "      unbindHierarchy: (parentId: number, childId: number) => void;",
                "    };",
                "  }",
                "}",
                "",
                "export {};",
                "",
            ]
        )
        return "\n".join(lines)

    def run(self, output_path: Path) -> None:
        for py_file in self.uix_path.glob("**/*.py"):
            self.parse_module(py_file)

        for class_name in list(self.class_props.keys()):
            resolved = self.resolve_inherited_props(class_name)
            if not resolved:
                self.definitions.pop(class_name, None)

        output_path.parent.mkdir(parents=True, exist_ok=True)
        output_path.write_text(self.generate_dts(), encoding="utf-8")
        print(f"ARTIFACT_GENERATION_COMPLETE: {output_path}")


if __name__ == "__main__":
    repo_root = Path(__file__).resolve().parents[1]
    destination = repo_root / "ui" / "src" / "global.d.ts"
    KivyASTScanner().run(destination)
