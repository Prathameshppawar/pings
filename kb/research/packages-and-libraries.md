# How libraries and packages work

*Written 2026-11-14. Based on: Russ Cox, "Version SAT" (2016) and "Minimal Version Selection" (2018);
Natalie Weizenbaum, "PubGrub: Next-Generation Version Solving" (2018); Cargo, Go modules, and npm docs.*

## Four layers people mix up

1. **Module:** a unit of code with its own namespace (a file in Python, a directory in Go).
2. **Package:** a versioned, publishable bundle of modules with a manifest.
3. **Package manager:** the tool that fetches packages, resolves versions, builds, and locks them
   (pip/uv, npm, cargo, `go mod`).
4. **Registry:** where packages are published and discovered (PyPI, npm, crates.io). Or no central registry
   at all: Go fetches straight from version control, with a proxy and a checksum database in front.

## Standard library vs ecosystem

- **Batteries included** (Python, Go): a big stdlib means less dependency churn, but stdlib APIs are frozen
  forever (hence "the stdlib is where modules go to die").
- **Small std** (Rust, JavaScript): the ecosystem moves fast, but there's more supply-chain risk and more
  choices for beginners.
- Pings: to be decided. Probably Go-sized basics (strings, collections, files, JSON, HTTP, math, tensors),
  because the playground has to be useful with zero installs.

## Manifest and lockfile

- **Manifest** (`Cargo.toml`, `package.json`, `go.mod`; for us, `pings.toml`): what you *want*, as names and
  version *ranges*.
- **Lockfile** (`Cargo.lock`, `package-lock.json`, `go.sum`): what you *got*, as exact versions plus content
  hashes. It makes builds reproducible and exposes tampering.

## Semantic versioning

`MAJOR.MINOR.PATCH` means breaking change, new feature, bug fix. The contract is social, not enforced.
Tools like `cargo-semver-checks` try to enforce it.

## Version resolution, the hard part

Given "A needs B ≥ 1.2, C needs B < 1.5, …", find one version of every package that satisfies everyone.

- In general this is **NP-complete**: you can encode boolean satisfiability in it (Cox, "Version SAT").
- **Backtracking solvers with good error messages:** PubGrub (Dart's pub, Python's uv, Swift Package Manager).
- **Minimal Version Selection** (Go modules): pick the *minimum* version that satisfies every requirement,
  and allow no upper bounds. Resolution becomes simple and polynomial, and builds are reproducible even
  without a lockfile.
- **npm** sidesteps the problem by allowing several versions of one package in the same tree (nested
  `node_modules`), at the cost of bloat and duplicated state.

## Native extensions and FFI

Libraries that wrap C or Rust code need a stable binary interface (an ABI) and compiled artifacts for each
platform (Python wheels, npm prebuilt binaries). For Pings, extensions will likely be Rust crates compiled
into `pings`, or dynamic libraries loaded over a C ABI. That's an ADR for Phase 5 or 6.

## Registry problems

- **Names:** first-come squatting, typosquatting, flat names vs namespaces (`@scope/name`).
- **Immutability:** a published version must never change. You "yank" a version instead of deleting it
  (npm's 2016 left-pad incident shows why).
- **Account security:** 2FA, and *trusted publishing* via CI identity (OIDC) instead of long-lived tokens,
  which PyPI, crates.io, and npm now support.
- Mirrors, download statistics, and malware scanning.

## Pings plan (Phase 5, tentative)

1. Modules are files; `import "path"`.
2. Dependencies are git URLs plus tags in `pings.toml`, Go-style, so there's no registry to host.
3. A lockfile with content hashes from day one.
4. Resolution via MVS (simple and learnable) unless we find a reason not to.
5. A registry only if people actually publish packages.
