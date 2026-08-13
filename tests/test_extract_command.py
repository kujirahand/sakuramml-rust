import importlib.util
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
SCRIPT_PATH = ROOT / "scripts" / "extract_command.py"
SPEC = importlib.util.spec_from_file_location("extract_command", SCRIPT_PATH)
assert SPEC is not None and SPEC.loader is not None
extract_command = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = extract_command
SPEC.loader.exec_module(extract_command)


class ExtractCommandTest(unittest.TestCase):
    def test_committed_command_markdown_is_current(self) -> None:
        generated = extract_command.render_markdown(ROOT / "src")
        current = (ROOT / "command.md").read_text(encoding="utf-8")
        self.assertEqual(current, generated)

    def test_output_is_unchanged_after_rustfmt(self) -> None:
        expected = extract_command.render_markdown(ROOT / "src")
        with tempfile.TemporaryDirectory() as temporary_dir:
            temporary_root = Path(temporary_dir)
            shutil.copy2(ROOT / "Cargo.toml", temporary_root / "Cargo.toml")
            shutil.copytree(ROOT / "src", temporary_root / "src")
            subprocess.run(
                ["cargo", "fmt", "--all", "--manifest-path", str(temporary_root / "Cargo.toml")],
                check=True,
                capture_output=True,
                text=True,
            )
            actual = extract_command.render_markdown(temporary_root / "src")
        self.assertEqual(expected, actual)

    def test_previous_multiline_failures_are_listed(self) -> None:
        generated = extract_command.render_markdown(ROOT / "src")
        expected_commands = [
            "| # | Macro - マクロ定義",
            "| 音源初期化 |",
            "| OctaveUnison |",
            "| Unison5th |",
            "| Unison3th |",
            "| Unison |",
            "| PortamentoTime |",
            "| PortamentoSwitch |",
        ]
        for command in expected_commands:
            with self.subTest(command=command):
                self.assertIn(command, generated)

    def test_annotation_may_follow_a_multiline_call(self) -> None:
        section = '''
items.set_item(
    "音源初期化",
    "ResetGM;",
);
// @ 音源を初期化する
'''
        calls = extract_command.extract_annotated_calls(
            section, r"items\.set_item", "fixture.rs", "FIXTURE"
        )
        self.assertEqual(1, len(calls))
        self.assertEqual("音源初期化", calls[0][0])
        self.assertIn('"ResetGM;"', calls[0][1])
        self.assertEqual("@ 音源を初期化する", calls[0][2])


if __name__ == "__main__":
    unittest.main()
