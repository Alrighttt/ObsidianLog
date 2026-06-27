# Security Policy

ObsidianLog handles operational logs and the encryption keys that protect them,
so security reports are taken seriously.

## Reporting a vulnerability

Please report vulnerabilities **privately** using GitHub's
[private vulnerability reporting](https://github.com/emmaglorypraise/ObsidianLog/security/advisories/new)
(Security tab → "Report a vulnerability"). Do not open a public issue.

Include, where possible: affected crate/version, a description, reproduction
steps, and impact. We aim to acknowledge reports within 72 hours.

## Scope

ObsidianLog is a **self-hosted, user-controlled** system. Its security model:

- **Client-side encryption only** — plaintext logs never leave the user's
  infrastructure; encryption (AES-256-GCM) happens before any write to Sia.
- **User-controlled keys** — keys are generated locally during `obsidianlog
  init`, stored in the OS keychain or a `0600` secrets file, and never
  transmitted. There is no key escrow.
- **Tamper-evidence** — chunks are append-only and SHA-256 hash-chained;
  `obsidianlog verify` detects any modification, reorder, or deletion.
- **No intermediary** — there is no ObsidianLog-operated proxy, gateway, or
  relay in the self-hosted path; the maintainers have zero access to user data.

Out of scope: the security of the user's own indexd deployment, Sia host
selection, and the user's operational key-management practices.

## Dependency hygiene

The dependency tree is audited against the RustSec advisory database via
`cargo audit` in CI (the `audit` job in `.github/workflows/ci.yml`). Critical dependencies
(`zstd`, `aes-gcm`, `sha2`) are maintained by the RustCrypto and zstd-rs
communities.
