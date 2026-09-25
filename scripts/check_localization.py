from __future__ import annotations

import json
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CATALOG = ROOT / "src" / "i18n" / "zh-CN.json"
MESSAGE_CALL = re.compile(r'crate::trf?!\(\s*"((?:\\.|[^"\\])*)"')
PLACEHOLDER = re.compile(r"\{([a-zA-Z_][a-zA-Z0-9_]*)\}")
NAMED_ARGUMENT = re.compile(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\s*=")


def macro_arguments(source: str, position: int) -> str:
    while position < len(source) and source[position].isspace():
        position += 1
    if position >= len(source) or source[position] != ",":
        return ""

    start = position + 1
    depth = 0
    in_string = False
    escaped = False
    for index in range(start, len(source)):
        char = source[index]
        if in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                in_string = False
            continue
        if char == '"':
            in_string = True
        elif char == "(":
            depth += 1
        elif char == ")":
            if depth == 0:
                return source[start:index]
            depth -= 1
    return ""


def load_catalog() -> dict[str, str]:
    duplicates: list[str] = []

    def unique_pairs(pairs: list[tuple[str, str]]) -> dict[str, str]:
        result: dict[str, str] = {}
        for key, value in pairs:
            if key in result:
                duplicates.append(key)
            result[key] = value
        return result

    catalog = json.loads(CATALOG.read_text(encoding="utf-8"), object_pairs_hook=unique_pairs)
    if duplicates:
        raise ValueError(f"duplicate catalog keys: {', '.join(sorted(set(duplicates)))}")
    return catalog


def check() -> list[str]:
    errors: list[str] = []
    catalog = load_catalog()
    for path in (ROOT / "src").rglob("*.rs"):
        if "tests" in path.parts:
            continue
        source = path.read_text(encoding="utf-8")
        for match in MESSAGE_CALL.finditer(source):
            key = json.loads(f'"{match.group(1)}"')
            source_args = set(PLACEHOLDER.findall(key))
            if "trf" in match.group(0):
                provided_args = set(NAMED_ARGUMENT.findall(macro_arguments(source, match.end())))
                if source_args != provided_args:
                    errors.append(
                        f"{path.relative_to(ROOT)}: arguments differ for {key!r} "
                        f"(message={sorted(source_args)}, call={sorted(provided_args)})"
                    )
            if key not in catalog:
                errors.append(f"{path.relative_to(ROOT)}: missing zh-CN text for {key!r}")
                continue
            translated_args = set(PLACEHOLDER.findall(catalog[key]))
            if source_args != translated_args:
                errors.append(
                    f"{path.relative_to(ROOT)}: placeholders differ for {key!r} "
                    f"(source={sorted(source_args)}, zh-CN={sorted(translated_args)})"
                )
    return errors


def main() -> int:
    try:
        errors = check()
    except (OSError, UnicodeError, json.JSONDecodeError, ValueError) as error:
        print(f"error: localization catalog: {error}", file=sys.stderr)
        return 1
    if errors:
        print("error: Simplified Chinese catalog is incomplete", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
