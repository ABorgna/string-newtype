# Welcome to the string-newtype development guide <!-- omit in toc -->

This guide is intended to help you get started with developing string-newtype.

If you find any errors or omissions in this document, please [open an issue](https://github.com/aborgna/string-newtype/issues/new)!

## #️⃣ Setting up the development environment

To setup the environment manually you will need:

- Rust 1.75+: https://www.rust-lang.org/tools/install
- Just (optional): 

## 🏃 Running the tests

The repository root contains a Justfile with the most common development tasks.
Run `just` to see a list.

To manually compile and test the rust code, run:

```bash
cargo test
```

Run the benchmarks with:

```bash
cargo bench
```

Finally, if you have rust nightly installed, you can run `miri` to detect
undefined behaviour in the code. Note that the _devenv_ shell only has rust
stable available.

```bash
cargo +nightly miri test
```

If you have `pre-commit` installed, you can run the checks with:

```bash
pre-commit run --all-files
```

## 💅 Coding Style

The rustfmt tool is used to enforce a consistent rust coding style. The CI will fail if the code is not formatted correctly.
To check this run:

```bash
# Format rust code
cargo fmt
```

We also check for clippy warnings, which are a set of linting rules for rust. To run clippy, run:

```bash
cargo clippy --all-targets
```

## 📈 Code Coverage

We run coverage checks on the CI. Once you submit a PR, you can review the
line-by-line coverage report on
[codecov](https://app.codecov.io/gh/aborgna/string-newtype/commits?branch=All%20branches).

To run the rust coverage checks locally, install `cargo-llvm-cov`, generate the report with:
```bash
cargo llvm-cov --lcov > lcov.info
```
and open it with your favourite coverage viewer. In VSCode, you can use
[`coverage-gutters`](https://marketplace.visualstudio.com/items?itemName=ryanluker.vscode-coverage-gutters).

## 🌐 Contributing to string-newtype

We welcome contributions to string-newtype! Please open [an issue](https://github.com/aborgna/string-newtype/issues/new) or [pull request](https://github.com/aborgna/string-newtype/compare) if you have any questions or suggestions.

PRs should be made against the `main` branch, and should pass all CI checks before being merged. This includes using the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) format for the PR title.

The general format of a contribution title should be:

```
<type>(<scope>)!: <description>
```

Where the scope is optional, and the `!` is only included if this is a semver breaking change that requires a major version bump.

We accept the following contribution types:

- feat: New features.
- fix: Bug fixes.
- docs: Improvements to the documentation.
- style: Formatting, missing semi colons, etc; no code change.
- refactor: Refactoring code without changing behaviour.
- perf: Code refactoring focused on improving performance.
- test: Adding missing tests, refactoring tests; no production code change.
- ci: CI related changes. These changes are not published in the changelog.
- chore: Updating build tasks, package manager configs, etc. These changes are not published in the changelog.
- revert: Reverting previous commits.