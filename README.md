# 2025-12-21

Results of comparing `gix-benchmarks blame gix` with `gix-blame` 0.1 vs.
`gix-benchmarks blame gix` with `gix-blame` 0.5. There doesn’t seem to be a
difference at all. This is with both versions of `gix-blame` using `imara-diff`
0.1 under the hood. In the future, I’ll add benchmarks for a version of
`gix-blame` that uses `imara-diff` 0.2.

Note: in `gix-blame` 0.2 and 0.3, there was a performance regression caused by
[this commit][commit-96164c5] and resolved by [this commit][commit-8dc5e98].
See [this issue][issue-2148] and [this PR][pr-2154] for context.

[commit-96164c5]: https://github.com/GitoxideLabs/gitoxide/commit/96164c5936032b4edb973828178cc55793dd57cc
[commit-8dc5e98]: https://github.com/GitoxideLabs/gitoxide/commit/8dc5e98d9dfe1fdbbd9db15ea149f253c9aaad0e
[pr-2154]: https://github.com/GitoxideLabs/gitoxide/pull/2154
[issue-2148]: https://github.com/GitoxideLabs/gitoxide/issues/2148

```
❯ env GIT_DIR="$HOME/github/Byron/gitoxide/.git" hyperfine --warmup 1 --export-markdown results.md 'gix-benchmarks@v0.1.0 blame --path Cargo.toml gix' 'gix-benchmarks@v0.5.0 blame --path Cargo.toml gix'
Benchmark 1: gix-benchmarks@v0.1.0 blame --path Cargo.toml gix
  Time (mean ± σ):      64.5 ms ±   1.3 ms    [User: 54.5 ms, System: 9.5 ms]
  Range (min … max):    62.4 ms …  67.8 ms    45 runs

Benchmark 2: gix-benchmarks@v0.5.0 blame --path Cargo.toml gix
  Time (mean ± σ):      64.5 ms ±   2.0 ms    [User: 53.9 ms, System: 10.2 ms]
  Range (min … max):    61.3 ms …  71.7 ms    45 runs

Summary
  gix-benchmarks@v0.1.0 blame --path Cargo.toml gix ran
    1.00 ± 0.04 times faster than gix-benchmarks@v0.5.0 blame --path Cargo.toml gix
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `gix-benchmarks@v0.1.0 blame --path Cargo.toml gix` | 64.5 ± 1.3 | 62.4 | 67.8 | 1.00 |
| `gix-benchmarks@v0.5.0 blame --path Cargo.toml gix` | 64.5 ± 2.0 | 61.3 | 71.7 | 1.00 ± 0.04 |

# 2025-04-09

Results of comparing a patched version of `git rev-list` vs. `gix-benchmarks log gix`, i. e. traversing a repository’s entire history chronologically using either `gix` (as a library) or `git` (as a patched standalone binary). The [patch][patch] makes `git rev-list` do roughly what the other benchmarks do: traverse the entire history of a repository, counting the number of `c`s in commit hashes as a checksum. Applying the patch on top of 5b97a56fa0e7d580dc8865b73107407c9b3f0eff and running the benchmark yields the following results.

At the time the benchmark was run, my version of `git/git` had 76_464 commits while `torvalds/linux` had 1_337_406 commits. `gix-benchmarks` was compiled using `cargo build --release` at commit e37c6fbf93d3d5d4ad6e2a3c695b09a2f6bc7579.

```
❯ env GIT_DIR="$HOME/github/git/git/.git" hyperfine --warmup 1 --export-markdown results.md 'target/release/gix-benchmarks log gix' '~/github/git/git/git rev-list HEAD'
Benchmark 1: target/release/gix-benchmarks log gix
  Time (mean ± σ):     720.0 ms ±   2.8 ms    [User: 710.2 ms, System: 7.8 ms]
  Range (min … max):   715.8 ms … 725.7 ms    10 runs

Benchmark 2: ~/github/git/git/git rev-list HEAD
  Time (mean ± σ):     602.6 ms ±   1.8 ms    [User: 589.0 ms, System: 11.4 ms]
  Range (min … max):   599.6 ms … 604.7 ms    10 runs

Summary
  '~/github/git/git/git rev-list HEAD' ran
    1.19 ± 0.01 times faster than 'target/release/gix-benchmarks log gix'
```

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/gix-benchmarks log gix` | 720.0 ± 2.8 | 715.8 | 725.7 | 1.19 ± 0.01 |
| `~/github/git/git/git rev-list HEAD` | 602.6 ± 1.8 | 599.6 | 604.7 | 1.00 |

```
❯ env GIT_DIR="$HOME/github/torvalds/linux/.git" hyperfine --warmup 1 --export-markdown results.md 'target/release/gix-benchmarks log gix' '~/github/git/git/git rev-list HEAD'
Benchmark 1: target/release/gix-benchmarks log gix
  Time (mean ± σ):     11.719 s ±  0.071 s    [User: 11.584 s, System: 0.115 s]
  Range (min … max):   11.656 s … 11.871 s    10 runs

Benchmark 2: ~/github/git/git/git rev-list HEAD
  Time (mean ± σ):     10.269 s ±  0.022 s    [User: 9.990 s, System: 0.248 s]
  Range (min … max):   10.240 s … 10.305 s    10 runs

Summary
  '~/github/git/git/git rev-list HEAD' ran
    1.14 ± 0.01 times faster than 'target/release/gix-benchmarks log gix'
```

| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `target/release/gix-benchmarks log gix` | 11.719 ± 0.071 | 11.656 | 11.871 | 1.14 ± 0.01 |
| `~/github/git/git/git rev-list HEAD` | 10.269 ± 0.022 | 10.240 | 10.305 | 1.00 |

[patch]: benchmark-git-rev-list.patch

# 2025-04-03

Results of comparing `gix-benchmarks log git` vs. `gix-benchmarks log gix`, i. e. traversing a repository’s entire history chronologically using either `gix` (as a library) or `git2` (as a library).

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
