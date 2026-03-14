#!/usr/bin/env python
# /// script
# requires-python = ">=3.13"
# ///

# env GIT_DIR="$HOME/github/Byron/gitoxide/.git" BASELINE_EXECUTABLE="$HOME/bin/gix-blame-2026-03-08-29040a827" COMPARISON_EXECUTABLE="$HOME/bin/gix-blame-2026-03-08-e63d487fb" ./run_benchmark.py

import os


def run(path, i):
    os.system(
        f'hyperfine "$BASELINE_EXECUTABLE blame {path}" "$COMPARISON_EXECUTABLE blame {path}" --export-markdown benchmark-{i}.md --export-json benchmark-{i}.json'
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
