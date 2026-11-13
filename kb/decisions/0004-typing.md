# ADR 0004: Typing discipline: gradual, with annotations parsed from day one

- **Status:** Proposed
- **Date:** 2026-11-14
- **Deciders:**

## Context

Go is statically typed with local inference (`x := 5`). Python is dynamic; its type hints are ignored at
runtime and checked only by external tools (mypy, pyright). We want Go's safety for larger programs and
Python's REPL comfort for exploration. The typing discipline also affects the parser (Phase 1), the checker
(Phase 3), and how fast the VM can be (Phase 4 and later).

## Options

1. **Dynamic only** (Python, JavaScript, Lua). Simplest to implement; errors show up at runtime.
2. **Static, annotations required, local inference** (Go). Errors caught early; more ceremony in a REPL.
3. **Static with full inference** (Hindley–Milner: OCaml, Gleam). Few annotations needed; inference errors can
   be confusing; awkward with some dynamic features.
4. **Gradual** (TypeScript, Python + mypy, Dart 1). Annotations are optional; annotated code gets checked;
   unannotated code is `any`.
   - *Unsound* gradual (TypeScript): types are checked at compile time and erased. Cheap, but types can lie at runtime.
   - *Sound* gradual (Typed Racket): runtime checks at typed/untyped boundaries. Types never lie, but the
     checks can cost a lot. See Takikawa et al., "Is Sound Gradual Typing Dead?" (POPL 2016).

## Decision (proposed)

**Gradual, staged:**

- **Phase 1:** the parser accepts type annotations; the tree-walker ignores them (dynamic semantics).
  This keeps Phase 1 small without painting us into a corner.
- **Phase 3:** add a static checker for annotated code, with local inference. Sound vs unsound gets its own
  ADR then, after the Phase 3 readings.

## Consequences

- The type syntax (`x: int`, `-> str`, generics?) must be designed before the parser, even though it's unused at first.
- Untyped scripts and REPL sessions stay frictionless; library authors can annotate public APIs.
- The VM can only optimize using types if they're sound. That ties this ADR to Phase 8 performance.
