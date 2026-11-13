# ADR 0003: License: MIT OR Apache-2.0

- **Status:** Proposed
- **Date:** 2026-11-14
- **Deciders:**

## Context

A public repo without a license isn't open source: nobody may legally use, modify, or redistribute the code.
We need a license for the implementation, and we have to think about what it means for *programs written in
Pings* and for the standard library that gets bundled into them.
Background: [research/licensing.md](../research/licensing.md).

## Options

| License | Kind | Patent grant | What users must do | Used by |
|---|---|---|---|---|
| MIT | permissive | not explicit | keep the notice | Node.js, Julia, Lua |
| BSD-3-Clause | permissive | not explicit | keep the notice; no endorsement | Go |
| Apache-2.0 | permissive | explicit | keep notice, state changes | Kotlin, TypeScript, Swift (+ runtime exception) |
| MIT OR Apache-2.0 | permissive, user picks | via Apache | either license's terms | Rust and most of its ecosystem |
| MPL-2.0 | file-level copyleft | explicit | share changes to *our* files | Firefox |
| GPL-3.0 | strong copyleft | explicit | derivatives must be GPL too | GCC (+ runtime exception) |

## Decision (proposed)

**MIT OR Apache-2.0**, the same as Rust.

- Permissive, so anyone can use Pings anywhere, including at work. That matters for a language people should try.
- Apache-2.0 brings an explicit patent grant. MIT keeps compatibility with GPLv2-only projects, which
  Apache-2.0 alone doesn't have.
- It matches the Rust ecosystem, so code can move between our crates and others' without friction.
- Programs written in Pings are unaffected, and the standard library can be bundled into users' programs
  without a runtime exception.
- Contributions: inbound = outbound (GitHub Terms §D.6), plus a DCO sign-off (`git commit -s`). No CLA.

## Consequences

- Add `LICENSE-MIT` and `LICENSE-APACHE`, set `license = "MIT OR Apache-2.0"` in `Cargo.toml`, and update the README.
- Dependencies must be compatible. `cargo-deny` enforces an allow-list in CI (Phase 2).
- The name "Pings" is a trademark question, which no code license covers. Revisit before v0.1.
- Optional: license the knowledge base under CC BY 4.0 instead.
