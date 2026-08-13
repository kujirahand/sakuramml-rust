#!/usr/bin/env python3
"""Rustソースの注釈からcommand.mdを生成する。"""

from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from pathlib import Path


class ExtractionError(RuntimeError):
    """コマンド情報を抽出できなかった場合のエラー。"""


@dataclass(frozen=True)
class Entry:
    name: str
    description: str


RUST_STRING = r'"((?:\\.|[^"\\])*)"'


def read_source(src_dir: Path, relative_path: str) -> str:
    return (src_dir / relative_path).read_text(encoding="utf-8")


def extract_section(source: str, marker: str, source_name: str) -> str:
    pattern = re.compile(
        rf"//\s*<{re.escape(marker)}>\s*(.*?)//\s*</{re.escape(marker)}>",
        re.DOTALL,
    )
    match = pattern.search(source)
    if match is None:
        raise ExtractionError(f"{source_name}: <{marker}> セクションが見つかりません")
    return match.group(1)


def require_entries(entries: list[Entry], marker: str, source_name: str) -> list[Entry]:
    if not entries:
        raise ExtractionError(f"{source_name}: <{marker}> の抽出結果が空です")
    return entries


def annotated_fragments(section: str) -> list[tuple[str, str]]:
    """// @注釈と、その注釈が属するRust構文の先頭部分を返す。"""
    lines = section.splitlines()
    result: list[tuple[str, str]] = []
    for index, line in enumerate(lines):
        if "// @" not in line:
            continue
        before, description = line.split("// @", 1)
        if "=>" not in before:
            for previous in reversed(lines[:index]):
                if previous.strip() and "=>" in previous:
                    before = previous
                    break
        if "=>" in before:
            result.append((before.split("=>", 1)[0].strip(), description.strip()))
    return result


def extract_char_commands(src_dir: Path) -> list[Entry]:
    source_name = "src/lexer.rs"
    section = extract_section(
        read_source(src_dir, "lexer.rs"), "CHAR_COMMANDS", source_name
    )
    entries = []
    for command, description in annotated_fragments(section):
        command = command.replace("|", " ").replace("'", "").strip()
        entries.append(Entry(command, description))
    return require_entries(entries, "CHAR_COMMANDS", source_name)


def extract_annotated_calls(
    section: str, call_pattern: str, source_name: str, marker: str
) -> list[tuple[str, str, str]]:
    """複数行の関数・マクロ呼び出しと、その末尾注釈を抽出する。"""
    pattern = re.compile(
        rf"^[ \t]*(?://[ \t]*)?({call_pattern}\s*\((.*?)\)\s*;)[ \t]*(?:\n[ \t]*)?//[ \t]*(@?[ \t]*[^\n]*)",
        re.MULTILINE | re.DOTALL,
    )
    calls = []
    for match in pattern.finditer(section):
        name_match = re.search(RUST_STRING, match.group(2))
        if name_match is None:
            continue
        calls.append((name_match.group(1), match.group(2), match.group(3)))
    if not calls:
        raise ExtractionError(f"{source_name}: <{marker}> の抽出結果が空です")
    return calls


def extract_sutoton(src_dir: Path) -> list[tuple[str, str, str]]:
    source_name = "src/sutoton.rs"
    section = extract_section(read_source(src_dir, "sutoton.rs"), "SUTOTON", source_name)
    calls = extract_annotated_calls(section, r"items\.set_item", source_name, "SUTOTON")
    entries = []
    for name, arguments, comment in calls:
        values = re.findall(RUST_STRING, arguments)
        if len(values) < 2 or not comment.lstrip().startswith("@"):
            continue
        description = comment.lstrip()[1:].strip().replace("|", "/")
        entries.append((name, values[1], description))
    if not entries:
        raise ExtractionError(f"{source_name}: <SUTOTON> の抽出結果が空です")
    return entries


def extract_variables(src_dir: Path) -> list[tuple[str, str, str]]:
    source_name = "src/mml_def/variables.rs"
    section = extract_section(
        read_source(src_dir, "mml_def/variables.rs"), "VARIABLES", source_name
    )
    calls = extract_annotated_calls(section, r"var\.insert", source_name, "VARIABLES")
    entries = []
    for name, arguments, comment in calls:
        if not comment.lstrip().startswith("@"):
            continue
        value_match = re.search(
            r"SValue::from_\w+\s*\(\s*(\d+|\"(?:\\.|[^\"\\])*\")", arguments
        )
        if value_match is None:
            continue
        value = value_match.group(1)
        # 旧スクリプトとの出力互換のため、@直後の空白を残す。
        description = comment[comment.index("@") + 1 :]
        entries.append((name, value, description))
    if not entries:
        raise ExtractionError(f"{source_name}: <VARIABLES> の抽出結果が空です")
    return entries


def extract_rhythm(src_dir: Path) -> list[tuple[str, str]]:
    source_name = "src/mml_def/rhythm.rs"
    section = extract_section(
        read_source(src_dir, "mml_def/rhythm.rs"), "RHYTHM_MACRO", source_name
    )
    pattern = re.compile(
        rf"\[\s*'(.)'\s+as\s+usize\s*-\s*0x40\s*\]\s*=\s*String::from\s*\(\s*{RUST_STRING}\s*\)",
        re.DOTALL,
    )
    entries = [(match.group(1), match.group(2)) for match in pattern.finditer(section)]
    if not entries:
        raise ExtractionError(f"{source_name}: <RHYTHM_MACRO> の抽出結果が空です")
    return entries


def extract_system_functions(src_dir: Path) -> list[Entry]:
    source_name = "src/mml_def/system_functions.rs"
    section = extract_section(
        read_source(src_dir, "mml_def/system_functions.rs"), "SYSTEM_FUNCTION", source_name
    )
    calls = extract_annotated_calls(
        section, r"sysfunc_\w+!", source_name, "SYSTEM_FUNCTION"
    )
    return require_entries(
        [Entry(name, comment.strip()) for name, _arguments, comment in calls],
        "SYSTEM_FUNCTION",
        source_name,
    )


def extract_calc_functions(src_dir: Path) -> list[tuple[str, str, str]]:
    source_name = "src/mml_def/system_functions.rs"
    section = extract_section(
        read_source(src_dir, "mml_def/system_functions.rs"),
        "SYSTEM_CALC_FUNCTION",
        source_name,
    )
    calls = extract_annotated_calls(
        section, r"syscalc_add!", source_name, "SYSTEM_CALC_FUNCTION"
    )
    entries = []
    for name, _arguments, comment in calls:
        parts = comment.split("//", 1)
        if len(parts) != 2:
            continue
        entries.append(
            (name, parts[0].strip().replace("|", "/"), parts[1].strip())
        )
    if not entries:
        raise ExtractionError(f"{source_name}: <SYSTEM_CALC_FUNCTION> の抽出結果が空です")
    return entries


def extract_system_refs(src_dir: Path) -> list[tuple[list[str], str]]:
    source_name = "src/runner/function.rs"
    section = extract_section(
        read_source(src_dir, "runner/function.rs"), "SYSTEM_REF", source_name
    )
    pattern = re.compile(r"\bif\s+(.*?)\{\s*//\s*@\s*([^\n]*)", re.DOTALL)
    entries = []
    for match in pattern.finditer(section):
        names = re.findall(r'cmd\s*==\s*"([^"]+)"', match.group(1))
        if names:
            entries.append((names, match.group(2).strip()))
    if not entries:
        raise ExtractionError(f"{source_name}: <SYSTEM_REF> の抽出結果が空です")
    return entries


def render_markdown(src_dir: Path) -> str:
    char_rows = "\n".join(
        f"| {entry.name} | {entry.description} |" for entry in extract_char_commands(src_dir)
    )
    system_rows = "\n".join(
        f"| {entry.name} | {entry.description} |" for entry in extract_system_functions(src_dir)
    )
    calc_rows = "\n".join(
        f"| {name} | {format_text} | {description} |"
        for name, format_text, description in extract_calc_functions(src_dir)
    )
    ref_rows = "\n".join(
        f"| {' /  '.join(names)}  |{description}|"
        for names, description in extract_system_refs(src_dir)
    )
    variable_rows = "\n".join(
        f"| {name} | {description} (値:{value}) |"
        for name, value, description in extract_variables(src_dir)
    )
    rhythm_rows = "\n".join(
        f"| {name} | \"{value}\" |" for name, value in extract_rhythm(src_dir)
    )
    sutoton_rows = "\n".join(
        f'| {name} | {description} (=\"{value}\") |'
        for name, value, description in extract_sutoton(src_dir)
    )

    return f"""# Sakuramml command list - テキスト音楽 サクラ

## Single-character command

Single-character(lower case) command list. (1文字小文字コマンド)

| Command | Description |
|---------|--------|
{char_rows}


## Multiple-character command

Multiple-character(upper case) command list. (複数文字/大文字コマンド)

| Command | Description |
|---------|--------|
{system_rows}


## Function usable within an expression

Function usable within an expression (計算式で使える関数)

| Command | Format | Description |
|---------|--------|--------|
{calc_rows}


## Values in a formula

Values that can be referenced in a formula (計算式で参照できる値)

| Command | Description |
|---------|--------|
{ref_rows}


## Macro and Voice List

[🔗voice list - 日本語付きの音色一覧はこちら](voice.md)
Macros and Voice list (マクロや音色など変数定義):

| Voice | Description |
|-------|----|
{variable_rows}


## Rhythm macro

Rhythm macro (リズムマクロ)

| Macro's name | Value |
|---------|--------|
{rhythm_rows}


## Sutoton

日本語で指示できるストトン表記


| ストトン表記 | 説明 (=定義) |
| ---------|---------|
{sutoton_rows}
"""


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    default_root = Path(__file__).resolve().parent.parent
    parser.add_argument("--root", type=Path, default=default_root, help="リポジトリのルート")
    parser.add_argument("--check", action="store_true", help="command.mdが最新か検査する")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    root = args.root.resolve()
    output_path = root / "command.md"
    generated = render_markdown(root / "src")
    if args.check:
        current = output_path.read_text(encoding="utf-8") if output_path.exists() else ""
        if current != generated:
            print("[ERROR] command.mdが最新ではありません。make docを実行してください。")
            return 1
        print("command.mdは最新です。")
        return 0
    output_path.write_text(generated, encoding="utf-8")
    print(f"生成しました: {output_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
