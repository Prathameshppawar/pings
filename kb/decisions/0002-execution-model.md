# ADR 0002: Execution model: tree-walker first, then a bytecode VM

- **Status:** Proposed
- **Date:** 2026-11-14
- **Deciders:**

## Context

"Interpreted or compiled?" describes an *implementation*, not a language. CPython compiles to bytecode and
then interprets that bytecode. V8 interprets JavaScript and compiles the hot functions to machine code. We
have to pick where Pings starts on that spectrum and keep a path to where it might go.
Background: [research/interpreted-vs-compiled.md](../research/interpreted-vs-compiled.md).

## Options

1. A tree-walking interpreter only.
2. A bytecode compiler and VM from day one.
3. A tree-walker first, then a bytecode VM, keeping the tree-walker as a reference implementation.
4. Ahead-of-time compilation to native code (via LLVM or Cranelift).
5. Transpiling to another language (C, JavaScript, Go).

## Decision (proposed)

**Option 3.** Phase 1 is a tree-walker; Phase 4 adds a bytecode compiler and VM. A JIT or AOT backend is a
Phase 8 question, decided with measurements.

Why:

- A tree-walker gets us to a working, publicly testable language fastest, and teaches semantics without
  performance distractions.
- After Phase 4 the tree-walker becomes the **reference implementation**: every test runs on both engines
  and their outputs must match (differential testing). That's a strong correctness net for the VM.
- A bytecode VM is the sweet spot for an interactive language. It's typically several times faster than a
  naive tree-walker (we'll measure our own number), stays portable, compiles to WASM, and keeps the REPL
  and instant startup.
- Going AOT first would push us to static types everywhere and give up the REPL and notebook experience
  that data work needs.

## Consequences

- Two engines to maintain for a while. Mitigation: they share the lexer, parser, and checker, and run the
  same test suite.
- Avoid features only a tree-walker makes cheap, like runtime access to the AST.
- To keep JIT/AOT possible, avoid semantics that make compilation hard, such as redefining anything at
  runtime or Python-style `locals()` mutation.
