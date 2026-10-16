from __future__ import annotations

import json
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CATALOG = ROOT / "src" / "i18n" / "zh-CN.json"
SRC = ROOT / "src"
MESSAGE_CALL = re.compile(r'crate::trf?!\(\s*"((?:\\.|[^"\\])*)"')
PLACEHOLDER = re.compile(r"\{([a-zA-Z_][a-zA-Z0-9_]*)\}")
NAMED_ARGUMENT = re.compile(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\s*=")

# `entry(key, "label")` in keybind_help.rs localizes its label through
# `crate::i18n::text`, so the label never appears next to `crate::tr!` and the
# message-call scan above cannot see it. The first argument is frequently a call
# such as `binding_label(&keybinds.split_horizontal)` or `format!(...)`, so the
# arguments must be split with balanced-delimiter scanning rather than a regex:
# a nested-paren regex silently skips most of the table.
ENTRY_CALL = re.compile(r"\bentry\(")
LEAF_ABOUT = re.compile(
    r"""(?:leaf!\(\s*"[^"]+",\s*("((?:\\.|[^"\\])*)")|about:\s*("((?:\\.|[^"\\])*)"))"""
)
# Literals in the CLI layer that are deliberately not translated: command
# syntax, machine-readable keys, and identifiers are part of the interface.
UNTRANSLATED_CLI_ALLOWLIST = ROOT / "scripts" / "localization-cli-allowlist.txt"
PARSE_CHOICE_CALL = re.compile(r'\bparse_choice\([^,]+,\s*"((?:\\.|[^"\\])*)"')
# Help text reaches clap as a plain `&'static str`, and the catalog is keyed by
# that English text. A bare literal keeps English in the help output even when
# `zh-CN.json` already carries a translation, so every call site must pass
# `crate::tr!("...")`; the regex below only matches the unwrapped form.
HELP_CALL = re.compile(r'\.help\(\s*"((?:\\.|[^"\\])*)"')
CLI_PROSE = re.compile(r"[a-z]+ [a-z]+")
CLI_NOT_PROSE = re.compile(
    r"^(?:[a-z0-9_.-]+/)+"  # paths
    r"|^--?[a-z]"  # flags
    r"|^\{.*\}$"  # pure placeholder
    r"|^[a-z0-9_]+$"  # identifier
    r"|^[A-Z_]+$"  # constant
    r"|^\s*herdr "  # indented command syntax, kept in English by design
)
# `usage: herdr ...` lines keep their command syntax in English but the
# `usage: ` label is translated, so they must still be flagged when unwrapped.
# Excluding them here previously hid 51 usage lines that had no catalog entry.
USAGE_PREFIX = "usage: "
STRING = re.compile(r'"((?:\\.|[^"\\])*)"')


def balanced_arguments(source: str, open_index: int) -> list[str] | None:
    """Split the argument list whose opening `(` is at ``open_index``."""
    depth = 0
    in_string = False
    escaped = False
    start = open_index + 1
    arguments: list[str] = []
    index = open_index
    while index < len(source):
        char = source[index]
        if in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                in_string = False
            index += 1
            continue
        if char == '"':
            in_string = True
        elif char in "([{":
            depth += 1
        elif char in ")]}":
            depth -= 1
            if depth == 0:
                arguments.append(source[start:index])
                return arguments
        elif char == "," and depth == 1:
            arguments.append(source[start:index])
            start = index + 1
        index += 1
    return None


def literal_spans(source: str) -> list[tuple[int, int]]:
    """Character ranges of every string literal passed to tr!/trf!.

    The untranslated-CLI scan must ignore exactly these literals. They are
    collected as spans rather than by inspecting the source line, because a
    formatted call routinely puts its literal on the following line:

        eprintln!(
            "{}",
            crate::trf!("live handoff complete; server log: {log}", log = log)
        );

    A line-based check would report that literal as untranslated even though it
    is localized, and a line-based allowlist would then hide real gaps.
    """
    spans: list[tuple[int, int]] = []
    for match in MESSAGE_CALL.finditer(source):
        spans.append((match.start(1), match.end(1)))
    for match in ENTRY_CALL.finditer(source):
        arguments = balanced_arguments(source, match.end() - 1)
        if not arguments or len(arguments) < 2:
            continue
        raw = string_literal(arguments[1])
        if raw is None:
            continue
        # locate the literal inside the original source after the call opens
        offset = source.find(raw, match.end())
        if offset != -1:
            spans.append((offset + 1, offset + len(raw) - 1))
    for match in LEAF_ABOUT.finditer(source):
        # leaf! or about: span
        if match.group(1) is not None:
            spans.append((match.start(2), match.end(2)))
        elif match.group(3) is not None:
            spans.append((match.start(4), match.end(4)))
    for match in PARSE_CHOICE_CALL.finditer(source):
        spans.append((match.start(1), match.end(1)))
    return spans


def string_literal(expression: str) -> str | None:
    """Return the raw literal if ``expression`` is exactly one string literal."""
    expression = expression.strip()
    if not expression.startswith('"'):
        return None
    index = 1
    escaped = False
    while index < len(expression):
        char = expression[index]
        if escaped:
            escaped = False
        elif char == "\\":
            escaped = True
        elif char == '"':
            return expression[: index + 1]
        index += 1
    return None


def macro_arguments(source: str, position: int) -> str:
    while position < len(source) and source[position].isspace():
        position += 1
    if position >= len(source) or source[position] != ",":
        return ""

    start = position + 1
    depth = 0
    in_string = False
    escaped = False
    clean_parts: list[str] = []
    part_start = start
    for index in range(start, len(source)):
        char = source[index]
        if in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                in_string = False
                part_start = index + 1
            continue
        if char == '"':
            clean_parts.append(source[part_start:index])
            in_string = True
            continue
        elif char == "(":
            depth += 1
        elif char == ")":
            if depth == 0:
                clean_parts.append(source[part_start:index])
                return "".join(clean_parts)
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


def load_allowlist() -> set[str]:
    """Literals exempt from the untranslated-text scan.

    Lines are kept verbatim apart from the trailing newline. Stripping them
    would make an entry that legitimately carries leading or trailing spaces
    unmatchable, which is exactly the shape of the notification-title markers
    (`" needs attention"`, `"Herdr v"`) that a `strip_suffix`/`strip_prefix`
    call must compare against verbatim. A `#` comment marker is still honored
    when it is the first non-space character of the line.
    """
    if not UNTRANSLATED_CLI_ALLOWLIST.exists():
        return set()
    entries: set[str] = set()
    for line in UNTRANSLATED_CLI_ALLOWLIST.read_text(encoding="utf-8").splitlines():
        if not line or line.lstrip().startswith("#"):
            continue
        entries.add(line)
    return entries


def iter_sources():
    for path in SRC.rglob("*.rs"):
        if "tests" in path.parts:
            continue
        yield path, path.read_text(encoding="utf-8")


def check_catalog_keys(catalog: dict[str, str], errors: list[str]) -> None:
    for path, source in iter_sources():
        relative = path.relative_to(ROOT)

        for match in MESSAGE_CALL.finditer(source):
            key = json.loads(f'"{match.group(1)}"')
            source_args = set(PLACEHOLDER.findall(key))
            if "trf" in match.group(0):
                provided = set(NAMED_ARGUMENT.findall(macro_arguments(source, match.end())))
                if source_args != provided:
                    errors.append(
                        f"{relative}: arguments differ for {key!r} "
                        f"(message={sorted(source_args)}, call={sorted(provided)})"
                    )
            if key not in catalog:
                errors.append(f"{relative}: missing zh-CN text for {key!r}")
                continue
            translated_args = set(PLACEHOLDER.findall(catalog[key]))
            if source_args != translated_args:
                errors.append(
                    f"{relative}: placeholders differ for {key!r} "
                    f"(source={sorted(source_args)}, zh-CN={sorted(translated_args)})"
                )

        for match in ENTRY_CALL.finditer(source):
            arguments = balanced_arguments(source, match.end() - 1)
            if not arguments or len(arguments) < 2:
                continue
            raw = string_literal(arguments[1])
            if raw is None:
                continue
            key = json.loads(raw)
            if key not in catalog:
                errors.append(f"{relative}: missing zh-CN text for {key!r}")

        for match in LEAF_ABOUT.finditer(source):
            raw = match.group(1) or match.group(3)
            key = json.loads(raw)
            if key not in catalog:
                errors.append(f"{relative}: missing zh-CN text for {key!r}")

        for match in PARSE_CHOICE_CALL.finditer(source):
            key = json.loads(f'"{match.group(1)}"')
            if key not in catalog:
                errors.append(f"{relative}: missing zh-CN text for {key!r}")

        for match in HELP_CALL.finditer(source):
            key = json.loads(f'"{match.group(1)}"')
            errors.append(
                f"{relative}: help text {key!r} is not wrapped in crate::tr! "
                f"(bare literals reach clap untranslated)"
            )


def item_end(source: str, start: int) -> int:
    """Offset just past the Rust item that begins at ``start``.

    An external module declaration ends at its `;`, while a block item ends at
    its matching `}`. Both are needed to know how far a `#[cfg(test)]` item
    really reaches.
    """
    depths = {"(": 0, "[": 0, "{": 0}
    closers = {")": "(", "]": "[", "}": "{"}
    in_string = False
    escaped = False
    index = start
    while index < len(source):
        char = source[index]
        if in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                in_string = False
            index += 1
            continue
        if char == "/" and source[index : index + 2] == "//":
            newline = source.find("\n", index)
            index = len(source) if newline == -1 else newline + 1
            continue
        if char == "/" and source[index : index + 2] == "/*":
            close = source.find("*/", index + 2)
            index = len(source) if close == -1 else close + 2
            continue
        if char == '"':
            in_string = True
        elif char in depths:
            depths[char] += 1
        elif char in closers:
            opener = closers[char]
            depths[opener] = max(0, depths[opener] - 1)
            if char == "}" and depths["{"] == 0:
                return index + 1
        elif char == ";" and not any(depths.values()):
            return index + 1
        index += 1
    return len(source)


def test_regions(source: str) -> list[tuple[int, int]]:
    """Character ranges covered by `#[cfg(test)]` items.

    Unit tests assert against literal English messages, so the untranslated-CLI
    scan must not treat them as user-visible gaps. Catalog-key scanning still
    covers them, because a test that calls `crate::tr!` needs a real entry.

    `#[cfg(test)]` annotates a single item, and that item is not always the
    trailing test module. A two-line `mod bindings_tests;` declaration used to
    be read as the start of everything after it, so the rest of `agent.rs` was
    never scanned and real untranslated strings there stayed invisible.
    """
    return [
        (match.start(), item_end(source, match.end()))
        for match in re.finditer(r"^#\[cfg\(test\)\]", source, re.MULTILINE)
    ]


def scanned_sources():
    """Every Rust file whose user-visible text must be localized.

    The scan started as CLI-only and is widened in stages, because the TUI
    renders most of its text outside `src/cli/` and a gap there is invisible to
    every other check: a bare literal compiles, ships, and reaches the screen in
    English while the catalog already holds the translation.

    `src/update.rs` is deliberately absent. Most of its literals are internal
    error strings for the download/install path, which a localized build refuses
    before reaching (`i18n::LOCALIZED_BUILD` short-circuits `self_update`), plus
    test fixtures. Only its user-reachable instruction text is localized, and
    that is covered by the catalog-key scan.

    Files under a `tests` directory are excluded, as are `*_tests.rs`; their
    literals are fixtures and assertions rather than interface text.
    """
    files = [
        SRC / "cli.rs",
        *sorted((SRC / "cli").rglob("*.rs")),
        *sorted((SRC / "client" / "shell").rglob("*.rs")),
        *sorted((SRC / "ui").rglob("*.rs")),
    ]
    for path in files:
        if "tests" in path.parts or path.name.endswith("_tests.rs"):
            continue
        yield path


def check_untranslated_cli(catalog: dict[str, str], errors: list[str]) -> None:
    allowlist = load_allowlist()
    for path in scanned_sources():
        relative = path.relative_to(ROOT)
        source = path.read_text(encoding="utf-8")
        localized = literal_spans(source)
        tests = test_regions(source)
        line_starts = [0]
        for index, char in enumerate(source):
            if char == "\n":
                line_starts.append(index + 1)

        def line_of(offset: int) -> int:
            low, high = 0, len(line_starts) - 1
            while low < high:
                middle = (low + high + 1) // 2
                if line_starts[middle] <= offset:
                    low = middle
                else:
                    high = middle - 1
            return low + 1

        for match in STRING.finditer(source):
            start, end = match.start(1), match.end(1)
            if any(region_start <= start and end <= region_end for region_start, region_end in tests):
                continue
            if any(span_start <= start and end <= span_end for span_start, span_end in localized):
                continue
            value = match.group(1)
            if CLI_NOT_PROSE.match(value) or not CLI_PROSE.search(value):
                continue
            if value in allowlist:
                continue
            if value.startswith(USAGE_PREFIX):
                errors.append(
                    f"{relative}:{line_of(start)}: untranslated usage text {value!r} "
                    f"(wrap it in crate::tr!, or list it in "
                    f"{UNTRANSLATED_CLI_ALLOWLIST.relative_to(ROOT)})"
                )
                continue
            errors.append(
                f"{relative}:{line_of(start)}: untranslated CLI text {value!r} "
                f"(wrap it in crate::tr!, or list it in "
                f"{UNTRANSLATED_CLI_ALLOWLIST.relative_to(ROOT)})"
            )


def check_stale_entries(catalog: dict[str, str], errors: list[str]) -> None:
    """Report catalog entries no source literal can ever produce.

    Upstream string changes leave the old entry behind. Nothing fails when that
    happens, so the catalog grows with dead keys and reviewers cannot tell
    which entries are live. A key is live when some literal in `src/` matches it
    exactly, because `text_ref` also looks keys up at runtime.
    """
    literals: set[str] = set()
    for path, source in iter_sources():
        for match in STRING.finditer(source):
            try:
                literals.add(json.loads(f'"{match.group(1)}"'))
            except json.JSONDecodeError:
                continue

    for key in sorted(catalog):
        if key in literals:
            continue
        errors.append(
            f"catalog: stale entry {key!r} has no matching literal in src/ "
            f"(remove it, or restore the text it translates)"
        )


def check() -> list[str]:
    errors: list[str] = []
    catalog = load_catalog()
    check_catalog_keys(catalog, errors)
    check_untranslated_cli(catalog, errors)
    check_stale_entries(catalog, errors)
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
