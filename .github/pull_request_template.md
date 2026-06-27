<!--
PR title must follow Conventional Commits, e.g.
  feat(store): add zstd compression
It becomes the squash-merge commit on `main`. See CONTRIBUTING.md.
-->

## What & why

<!-- What does this change, and what problem does it solve? -->

## Milestone / task

<!-- Which grant milestone and task does this implement? See docs/grant/PROGRESS.md.
     e.g. Milestone 1 — zstd compression -->

## Testing instructions

<!-- How does a reviewer verify this? Commands, an integration test to run, or UI
     steps. If testing doesn't apply, say so and why. -->

## Remaining work

<!-- If the task isn't fully done: what difficulties were hit and what's left.
     Otherwise: "None — task complete." -->

## Related issues

<!-- e.g. Closes #123 -->

## Checklist

- [ ] PR title follows [Conventional Commits](../blob/main/CONTRIBUTING.md#commit-messages)
- [ ] `cargo build --workspace` succeeds (code is buildable)
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test --workspace --all-features` passes
- [ ] Added or updated tests for the change
- [ ] Testing instructions included above (or a note on why not applicable)
- [ ] Updated the relevant README(s) if behavior, build, or run steps changed
- [ ] Updated `CHANGELOG.md` under `[Unreleased]` (for user-visible changes)
- [ ] Logged this task in `docs/grant/PROGRESS.md`
- [ ] Added an ADR under `docs/adr/` (for significant design decisions)
