# Pings: instructions for Claude

Pings is a programming language built from scratch in Rust, in public, **primarily so Prathamesh
learns how languages work**. Understanding beats speed of delivery. Source files use `.pi`.

## Start of every session

1. Read the newest file in `kb/journal/`. It says where we stopped, what's waiting on Prathamesh, and what's next.
2. Check `kb/roadmap.md` for the current phase and `kb/decisions/` for Proposed ADRs that block the next step.

## How we work (teaching mode)

- One concept per step. Before writing code for a concept, explain it in a few sentences and point to the
  exact reading (book chapter, paper section) from `kb/reading-list.md`.
- Small commits, one concept each. Never build several roadmap phases in one go.
- Real design choices: present the options with tradeoffs and a recommendation, then let Prathamesh decide.
  Record the outcome as an ADR in `kb/decisions/` (copy `0000-template.md`). Never edit an Accepted ADR's
  decision; supersede it with a new ADR.
- Research goes in `kb/research/<topic>.md`: plain language, sources cited. New terms go in `kb/glossary.md`.
- End every session with `kb/journal/YYYY-MM-DD.md` (append if today's exists): built, learned, decided,
  waiting on Prathamesh, next.
- Prathamesh is learning Rust alongside this. Explain non-obvious Rust (ownership puzzles, lifetimes,
  `Rc<RefCell<_>>`, trait objects) the first time it appears.

## Code

- Cargo workspace; crates live in `crates/`. The CLI crate and binary are both `pings`.
- Toolchain pinned in `rust-toolchain.toml`; bump it deliberately in its own commit.
- Before every commit: `cargo fmt --all`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- `unsafe_code` is denied workspace-wide. An exception needs `#[allow(unsafe_code)]` on the smallest scope,
  a `// SAFETY:` comment, and an ADR.
- Every language feature ships with tests in the same commit.
- Prefer std over dependencies until we've built the thing ourselves once. Adding a dependency gets a line
  in the journal saying why.
- CI pins GitHub Actions to commit SHAs. Keep it that way.

## Git

- Remote: github.com/Prathameshppawar/pings (public), branch `main`.
- This repo pushes as **Prathameshppawar** through a repo-local credential helper
  (`git config --local --get-all credential.https://github.com.helper`) and commits with that account's
  noreply email. The machine's globally active `gh` account is a different one: don't change global git
  or `gh` config.
- For `gh` commands on this repo, prefix them with `GH_TOKEN=$(gh auth token --user Prathameshppawar)`.
- Commit or push only when asked.
