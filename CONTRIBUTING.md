# Contributing to ObsidianLog

Thanks for your interest! ObsidianLog is MIT-licensed and developed in the open.

## Development setup

You need a stable Rust toolchain. The repo pins it via `rust-toolchain.toml`, so
`rustup` will install the right channel and components automatically.

```sh
git clone https://github.com/emmaglorypraise/ObsidianLog
cd ObsidianLog
cargo build --workspace
```

## Before you open a PR

Run the same checks CI runs — all must pass:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

If you touched dependencies, also run a vulnerability check:

```sh
cargo install cargo-audit   # once
cargo audit
```

## Workspace layout

- `crates/obsidianlog-core` — shared types, the canonical error, and the
  `StorageBackend` trait (no I/O)
- `crates/obsidianlog-store` — storage pipeline + backend impls (`LocalBackend`
  default; `SiaBackend` behind the `sia` feature)
- `crates/obsidianlog-ingest` — Vector-compatible HTTP ingest service
- `crates/obsidianlog-cli` — the `obsidianlog` binary and published crate

Keep the Sia integration behind the `sia` feature and the single `SiaBackend`
impl; the compression/encryption/chunking code talks only to the
`StorageBackend` trait and must build and test against `LocalBackend` with no
Sia node.

## Conventions

- Stubs are marked with `todo!()` and a `TODO(impl):` comment describing intent.
  When you implement one, remove the marker and add or un-`#[ignore]` a test.
- Public items carry doc comments; run `cargo doc --workspace` to check.
- Significant or hard-to-reverse design choices get an
  [ADR](docs/adr/README.md).
- Update `CHANGELOG.md` under `[Unreleased]` for user-visible changes.

## Branch names

Never commit to `main` directly — work on a branch and open a PR. Name branches
to mirror the commit convention:

```
<type>/<short-kebab-description>
```

- **`<type>`** uses the same vocabulary as commit types (`feat`, `fix`, `docs`,
  `refactor`, `perf`, `test`, `build`, `ci`, `chore`).
- **`<description>`** is lowercase, kebab-case, and short.
- Optionally prefix the issue number: `fix/142-config-permissions`.
- Release branches use `release/<version>`, e.g. `release/0.2.0`.

Examples:

```
feat/zstd-compression
fix/ingest-malformed-batch
docs/adr-async-backend
chore/bump-axum
```

> Branch naming is a convention, not a CI gate: external contributors work from
> forks, where their branch name lives on their own repository. It mainly keeps
> the history readable for maintainers and direct collaborators.

## Commit messages

We follow [Conventional Commits 1.0.0](https://www.conventionalcommits.org).
A clean, machine-readable history lets us auto-generate the changelog and pick
the next version automatically.

Format:

```
<type>(<scope>): <description>

[optional body explaining the "why", wrapped ~72 cols]

[optional footer(s)]
```

**Types:**

| Type | Use for |
| --- | --- |
| `feat` | a new feature (→ minor version bump) |
| `fix` | a bug fix (→ patch bump) |
| `docs` | documentation only |
| `refactor` | code change that is neither a fix nor a feature |
| `perf` | a performance improvement |
| `test` | adding or fixing tests |
| `build` | build system or dependency changes |
| `ci` | CI configuration (the workflows) |
| `chore` | maintenance with no source change |
| `revert` | reverting a previous commit |

**Scopes** name the affected crate, or a repo-wide area: `core`, `store`,
`ingest`, `cli`, `deps`, `release`. Scope is optional — omit it for changes that
don't belong to a single crate (e.g. `ci: ...`).

**Breaking changes** add a `!` before the colon and/or a `BREAKING CHANGE:`
footer:

```
feat(store)!: change on-Sia chunk framing to include a header version

BREAKING CHANGE: chunks written before this change cannot be read by `verify`.
```

While the project is pre-1.0, a breaking change bumps the minor version
(`0.1` → `0.2`), per SemVer.

Examples:

```
feat(store): add zstd compression to the pipeline
fix(ingest): reject malformed Vector batches instead of panicking
docs(adr): record the async vs sync StorageBackend decision
ci: add the cross-platform test matrix
chore(deps): bump axum to 0.8.10
```

To have `git commit` pre-fill this format, enable the template once:

```sh
git config commit.template .gitmessage
```

### Enforcement (planned for Month 3)

Today the convention is **adopted by habit and review**, not enforced by CI —
there is intentionally no commit-lint gate while the contributor base is small.

Before the public launch (Month 3) we will enforce it on **pull-request titles**
(not individual commits), since PRs are squash-merged and the PR title becomes
the commit that lands on `main`. That requires:

1. A `.github/workflows/pr-title.yml` running
   [`amannn/action-semantic-pull-request`](https://github.com/amannn/action-semantic-pull-request).
2. Repo settings: allow **squash-merge only**, and add the PR-title check to
   `main`'s required status checks under branch protection.

## Reporting security issues

Please **do not** open public issues for vulnerabilities — see
[SECURITY.md](SECURITY.md).
