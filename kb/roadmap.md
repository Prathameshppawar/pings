# Roadmap

Each phase lists what we **build**, **learn**, and **read**, and when it's **done**. The order after Phase 2
is tentative. Readings refer to [reading-list.md](reading-list.md).

## Phase 0: Foundations ← we are here

- **Build:** repo, knowledge base, CI, `pings --version`.
- **Learn:** Rust basics (ownership, enums, pattern matching); the map of how languages are implemented.
- **Read:** Rust Book ch. 1–6; Crafting Interpreters ch. 1–3.
- **Decide:** ADR 0002 (execution model), 0003 (license), 0004 (typing); vision open questions 1 and 5.
- **Done when:** those ADRs are Accepted or deliberately deferred.

## Phase 1: Tree-walking interpreter

- **Build:** lexer → Pratt parser → AST → evaluator. Variables, functions, closures, control flow. A REPL,
  `pings run file.pi`, and error messages that point at the source (line, column, underline).
- **Learn:** tokens, grammars, precedence, recursive descent, environments and scope, closures, error reporting.
- **Read:** Crafting Interpreters ch. 4–13; Pratt (1973).
- **Done when:** FizzBuzz, recursive Fibonacci, and a closure-based counter run; every feature has tests;
  the lexer and parser have been fuzzed.

## Phase 2: v0.1 public, so people can try it

- **Build:** WebAssembly build and a browser playground (GitHub Pages); release binaries for macOS, Linux,
  and Windows; install script; Docker image on GHCR; docs site; `SECURITY.md`, `CONTRIBUTING.md`.
- **Learn:** cross-compilation, WASM, release pipelines, signing and provenance, collecting feedback.
- **Read:** [research/distribution-and-hosting.md](research/distribution-and-hosting.md),
  [research/security.md](research/security.md); Haas et al. (2017).
- **Done when:** a stranger can open a link, run Pings in the browser, and file an issue.

## Phase 3: Types

- **Build:** a type checker (per ADR 0004) with good error messages.
- **Learn:** type systems, inference, soundness, gradual typing.
- **Read:** Pierce, TAPL ch. 1–11; Siek & Taha (2006); Takikawa et al. (2016); Dunfield & Krishnaswami (2021).

## Phase 4: Bytecode VM and garbage collector

- **Build:** compiler from AST to bytecode, a stack VM, a mark-sweep GC, a benchmark suite, and
  differential tests (the tree-walker and the VM must agree on every test).
- **Learn:** instruction design, dispatch, stack vs register VMs, GC algorithms, profiling.
- **Read:** Crafting Interpreters ch. 14–30; the Lua 5.0 paper (2005); Ertl & Gregg (2003); Wilson (1992).
- **Done when:** the VM passes every test and is measurably faster on the benchmarks (target: ≥5×).

## Phase 5: Modules, standard library, tooling, packages

- **Build:** `import`, `pings.toml`, a lockfile, git-based dependencies; `pings fmt`, `pings test`;
  a language server (LSP) for editor support.
- **Learn:** module systems, version resolution and why it's NP-complete, the LSP protocol.
- **Read:** [research/packages-and-libraries.md](research/packages-and-libraries.md); Cox (2016, 2018).

## Phase 6: Data and AI

- **Build:** a native `tensor` type (strided n-d arrays), broadcasting, `@`, BLAS-backed matmul,
  dataframes via Apache Arrow and Polars, a Jupyter kernel, maybe Python interop.
- **Learn:** memory layout and strides, vectorization, SIMD, BLAS, Arrow, notebooks.
- **Read:** Harris et al. (2020); Bezanson et al. (2017).

## Phase 7: Concurrency

- **Build:** tasks and channels, a scheduler, and no global interpreter lock.
- **Learn:** CSP, green threads, work stealing, why CPython had a GIL (and PEP 703).
- **Read:** Hoare (1978).

## Phase 8: Performance frontier

- **Build:** a JIT (Cranelift) or ahead-of-time compilation to native code or WASM, chosen when we get there.
- **Learn:** IR, SSA, register allocation, tracing vs method JITs.
- **Read:** Aycock (2003); Gal et al. (2009); Bolz et al. (2009).

## Always on

- Every session ends with a journal entry.
- Security: fuzzing from Phase 1, dependency auditing from Phase 2.
- Every feature ships with tests.
