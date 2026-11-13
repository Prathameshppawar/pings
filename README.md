# Pings

A programming language built from scratch, in public, to learn how languages really work:
lexing, parsing, interpreters, bytecode VMs, garbage collectors, type checkers, package managers,
distribution and security. Every decision and every paper we read is recorded in [`kb/`](kb/).

> **Status: Phase 0, foundations.** There is no language yet. `pings --version` works; the lexer is next.
> Follow along in the [journal](kb/journal/).

## Goals

- **Go-like simplicity:** small syntax, structural interfaces, built-in tooling, one static binary.
- **Python-like comfort for data and AI:** a REPL, notebooks, native tensors, fast native libraries underneath.
- **Safe by default:** programs get no file or network access unless you grant it.
- **Easy to try:** browser playground (WebAssembly), prebuilt binaries, Docker image.

Full vision: [kb/design/vision.md](kb/design/vision.md). Plan: [kb/roadmap.md](kb/roadmap.md).

## Build from source

Requires [rustup](https://rustup.rs). The compiler version is pinned in `rust-toolchain.toml` and installs automatically.

```sh
cargo run -- --version
cargo test
```

## Layout

```
crates/pings/   the `pings` command-line tool (split into more crates as it grows)
kb/             knowledge base: vision, roadmap, decisions (ADRs), research notes, reading list, journal
```

Pings source files use the `.pi` extension.

## License

Not chosen yet, see [ADR 0003](kb/decisions/0003-license.md). Until a license file is added, all rights are reserved.
