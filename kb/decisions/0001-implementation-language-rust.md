# ADR 0001: Implement Pings in Rust

- **Status:** Accepted
- **Date:** 2026-11-14
- **Deciders:** Prathamesh

## Context

The language we write the interpreter in shapes everything: runtime speed, how we manage memory for Pings
values, whether Pings can run in a browser, how we ship binaries, and how much we learn along the way.

## Options

| | Go | Rust | C |
|---|---|---|---|
| Already known | yes | no | partly |
| Runtime speed | good | best | best |
| Memory for the Pings heap | Go's GC does it for us (we learn less) | we design it (`Rc` first, our own GC later) | we write it, without safety nets |
| Memory safety of our own code | safe | safe outside `unsafe` | every bug can become an exploit |
| Browser playground (WASM) | works; large binaries (TinyGo is smaller) | first-class (`wasm-bindgen`), small binaries | works (Emscripten) |
| Ecosystem for data/AI (Phase 6) | gonum | ndarray, Polars, Arrow, candle, burn, PyO3 | BLAS and LAPACK directly |
| Code generation for a later JIT/AOT | limited | Cranelift, LLVM via inkwell | LLVM, libgccjit |
| Language projects built with it | — | Gleam, Boa (JS engine), RustPython, Ruff, uv, Deno | Lua, CPython, Ruby |

## Decision

Rust. Best runtime speed and WASM story, memory-safe, and the strongest ecosystem for the data/AI phase.

## Consequences

- We learn Rust and language implementation at the same time. Phase 0 includes deliberate Rust study, and
  Phase 1 will move slower than it would in Go.
- Rust's borrow checker makes tree- and graph-shaped data (ASTs, environments, closures) awkward. We'll use
  arenas with indices or `Rc<RefCell<…>>`, and write an ADR when we choose.
- We write our own garbage collector in Phase 4. That's a feature for a learning project, not a cost.
- The playground can run fully in the browser via WASM ([research/distribution-and-hosting.md](../research/distribution-and-hosting.md)).
- The workspace denies `unsafe_code`. Each exception needs a justification comment and an ADR.
