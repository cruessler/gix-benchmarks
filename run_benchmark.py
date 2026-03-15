#!/usr/bin/env python
# /// script
# requires-python = ">=3.13"
# ///

# env GIT_DIR="$HOME/github/Byron/gitoxide/.git" BASELINE_EXECUTABLE="$HOME/bin/gix-blame-2026-01-25-3b6650a66" COMPARISON_EXECUTABLE="$HOME/bin/gix-blame-experimental-2026-01-25-3b6650a66" ./run_benchmark.py
# env GIT_DIR="$HOME/github/Byron/gitoxide/.git" BASELINE_EXECUTABLE="$HOME/bin/gix-blame-2026-03-08-29040a827" COMPARISON_EXECUTABLE="$HOME/bin/gix-blame-2026-03-08-e63d487fb" ./run_benchmark.py

import os


def run(path, i):
    os.system(
        f'hyperfine --parameter-list command "$BASELINE_EXECUTABLE","$COMPARISON_EXECUTABLE" --parameter-list path {path} "{{command}} blame {{path}}" --command-name baseline --command-name comparison --export-markdown benchmark-{i}.md --export-json benchmark-{i}.json'
    )


for i, path in enumerate(
    [
        "CHANGELOG.md",
        "STABILITY.md",
        "README.md",
        "Cargo.toml",
        "gix-blame/src/file/function.rs",
        "gix-path/src/env/mod.rs",
        "gix-index/tests/index/file/write.rs",
        "gix-object/src/lib.rs",
        "gix-odb/src/store_impls/loose/write.rs",
    ]
):
    run(path, i + 1)
