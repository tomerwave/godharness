from __future__ import annotations

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts" / "check-release.py"


class CheckReleaseTest(unittest.TestCase):
    def write_repo(self, directory: Path, version: str, changelog: str) -> None:
        (directory / "Cargo.toml").write_text(
            f'[workspace.package]\nversion = "{version}"\n', encoding="utf-8"
        )
        (directory / "CHANGELOG.md").write_text(changelog, encoding="utf-8")
        crates = directory / "crates" / "godharness-cli"
        crates.mkdir(parents=True)
        (crates / "Cargo.toml").write_text(
            f'[dependencies]\ngodharness-core = {{ path = "../godharness-core", version = "{version}" }}\n',
            encoding="utf-8",
        )

    def run_script(self, directory: Path, tag: str, notes: bool = False) -> subprocess.CompletedProcess[str]:
        args = [sys.executable, str(SCRIPT), tag]
        if notes:
            args.append("--notes")
        return subprocess.run(args, cwd=directory, capture_output=True, text=True, check=False)

    def test_agrees_when_tag_manifest_and_changelog_match(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            self.write_repo(
                path,
                "0.2.0",
                "## [0.2.0] - 2026-08-07\n\n### Added\n\n- Something users can read.\n",
            )

            result = self.run_script(path, "v0.2.0")

            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertIn("matches version 0.2.0", result.stdout)

    def test_rejects_a_tag_that_disagrees_with_the_manifest(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            self.write_repo(
                path, "0.2.0", "## [0.2.0] - 2026-08-07\n\n### Added\n\n- Something.\n"
            )

            result = self.run_script(path, "v0.3.0")

            self.assertNotEqual(result.returncode, 0)
            self.assertIn("does not match", result.stderr)

    def test_rejects_a_missing_changelog_section(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            self.write_repo(path, "0.2.0", "## [0.1.0] - 2026-08-01\n\n### Added\n\n- First.\n")

            result = self.run_script(path, "v0.2.0")

            self.assertNotEqual(result.returncode, 0)
            self.assertIn("no section for 0.2.0", result.stderr)

    def test_rejects_a_section_with_only_internal_entries(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            self.write_repo(
                path,
                "0.2.0",
                "## [0.2.0] - 2026-08-07\n\n### Internal\n\n- Refactored something.\n",
            )

            result = self.run_script(path, "v0.2.0")

            self.assertNotEqual(result.returncode, 0)
            self.assertIn("announces nothing", result.stderr)

    def test_rejects_a_mismatched_internal_crate_version(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            self.write_repo(
                path, "0.2.0", "## [0.2.0] - 2026-08-07\n\n### Added\n\n- Something.\n"
            )
            crate_manifest = path / "crates" / "godharness-cli" / "Cargo.toml"
            crate_manifest.write_text(
                '[dependencies]\ngodharness-core = { path = "../godharness-core", version = "0.1.9" }\n',
                encoding="utf-8",
            )

            result = self.run_script(path, "v0.2.0")

            self.assertNotEqual(result.returncode, 0)
            self.assertIn("godharness-core 0.1.9", result.stderr)

    def test_prints_release_notes_excluding_internal(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            self.write_repo(
                path,
                "0.2.0",
                "## [0.2.0] - 2026-08-07\n\n### Added\n\n- A real feature.\n\n"
                "### Internal\n\n- A refactor nobody sees.\n",
            )

            result = self.run_script(path, "v0.2.0", notes=True)

            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertIn("A real feature.", result.stdout)
            self.assertNotIn("A refactor nobody sees.", result.stdout)


if __name__ == "__main__":
    unittest.main()
