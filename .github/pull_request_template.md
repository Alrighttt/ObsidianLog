<!--
PR title must follow Conventional Commits, e.g.
  feat(store): add zstd compression
It becomes the squash-merge commit on `main`. See CONTRIBUTING.md.
-->

## What & why

<!-- What does this change, and what problem does it solve? -->

## Related issues

<!-- e.g. Closes #123 -->

## Checklist

- [ ] PR title follows [Conventional Commits](../blob/main/CONTRIBUTING.md#commit-messages)
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test --workspace --all-features` passes
- [ ] Added or updated tests for the change
- [ ] Updated `CHANGELOG.md` under `[Unreleased]` (for user-visible changes)
- [ ] Added an ADR under `docs/adr/` (for significant design decisions)
