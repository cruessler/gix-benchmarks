# 2025-04-03

Results of comparing `gix-benchmarks log git` vs. `gix-benchmarks log gix`, i. e. traversing a repository’s entire history chronologically using either `gix` or `git2`.

At the time the benchmark was run, my version of `git/git` had 76_464 commits while `torvalds/linux` had 1_337_406 commits. `gix-benchmarks` was compiled using `cargo build --release` at commit e37c6fbf93d3d5d4ad6e2a3c695b09a2f6bc7579.

```
❯ env GIT_DIR="$HOME/github/git/git/.git" hyperfine --warmup 1 --export-markdown results.md 'target/release/gix-benchmarks log git' 'target/release/gix-benchmarks log gix'
Benchmark 1: target/release/gix-benchmarks log git
  Time (mean ± σ):     830.4 ms ±  10.7 ms    [User: 783.3 ms, System: 44.3 ms]
  Range (min … max):   816.0 ms … 855.7 ms    10 runs

Benchmark 2: target/release/gix-benchmarks log gix
  Time (mean ± σ):     717.4 ms ±   1.4 ms    [User: 707.1 ms, System: 8.4 ms]
  Range (min … max):   715.4 ms … 719.9 ms    10 runs

Summary
  'target/release/gix-benchmarks log gix' ran
    1.16 ± 0.02 times faster than 'target/release/gix-benchmarks log git'
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/gix-benchmarks log git` | 830.4 ± 10.7 | 816.0 | 855.7 | 1.16 ± 0.02 |
| `target/release/gix-benchmarks log gix` | 717.4 ± 1.4 | 715.4 | 719.9 | 1.00 |

```
❯ env GIT_DIR="$HOME/github/torvalds/linux/.git" hyperfine --warmup 1 --export-markdown results.md 'target/release/gix-benchmarks log git' 'target/release/gix-benchmarks log gix'
Benchmark 1: target/release/gix-benchmarks log git
  Time (mean ± σ):     22.221 s ±  0.359 s    [User: 21.854 s, System: 0.297 s]
  Range (min … max):   21.841 s … 23.090 s    10 runs

Benchmark 2: target/release/gix-benchmarks log gix
  Time (mean ± σ):     11.819 s ±  0.235 s    [User: 11.667 s, System: 0.126 s]
  Range (min … max):   11.680 s … 12.468 s    10 runs

Summary
  'target/release/gix-benchmarks log gix' ran
    1.88 ± 0.05 times faster than 'target/release/gix-benchmarks log git'
```

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/gix-benchmarks log git` | 22.221 ± 0.359 | 21.841 | 23.090 | 1.88 ± 0.05 |
| `target/release/gix-benchmarks log gix` | 11.819 ± 0.235 | 11.680 | 12.468 | 1.00 |
