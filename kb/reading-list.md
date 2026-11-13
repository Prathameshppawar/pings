# Reading list

Status: `[ ]` not started · `[~]` in progress · `[x]` done (link your notes when done).
Read in phase order, and don't read more than one phase ahead.

## Phase 0: foundations (now)

- [ ] *The Rust Programming Language* ("the Book"), ch. 1–6. Free at doc.rust-lang.org/book
- [ ] Robert Nystrom, *Crafting Interpreters* (2021), ch. 1–3. Free at craftinginterpreters.com.
      Ch. 2, "A Map of the Territory", is the best short answer to "interpreted vs compiled".
- [ ] Our notes: [interpreted-vs-compiled](research/interpreted-vs-compiled.md), [licensing](research/licensing.md)

## Phase 1: tree-walking interpreter

- [ ] Crafting Interpreters ch. 4–13 (scanning, parsing, evaluation, scope, closures, classes)
- [ ] Vaughan Pratt, "Top Down Operator Precedence", POPL 1973. The expression-parsing technique we'll use.
      An easier entry point: Nystrom, "Pratt Parsers: Expression Parsing Made Easy" (blog, 2011).
- [ ] The Rust Book ch. 8–10 and 15 (collections, errors, generics and traits, smart pointers: `Rc`, `RefCell`)
- [ ] Alexis Beingessner, *Learn Rust With Entirely Too Many Linked Lists*. Rust ownership vs pointer-heavy data.

## Phase 2: going public

- [ ] Andreas Haas et al., "Bringing the Web up to Speed with WebAssembly", PLDI 2017
- [ ] The `wasm-bindgen` guide; the cargo-dist (`dist`) docs
- [ ] Our notes: [distribution-and-hosting](research/distribution-and-hosting.md), [security](research/security.md)

## Phase 3: types

- [ ] Benjamin Pierce, *Types and Programming Languages* (2002), ch. 1–11
- [ ] Jeremy Siek & Walid Taha, "Gradual Typing for Functional Languages", Scheme Workshop 2006
- [ ] Asumu Takikawa et al., "Is Sound Gradual Typing Dead?", POPL 2016
- [ ] Jana Dunfield & Neel Krishnaswami, "Bidirectional Typing", ACM Computing Surveys 2021
- [ ] Luis Damas & Robin Milner, "Principal type-schemes for functional programs", POPL 1982 (Hindley–Milner)

## Phase 4: VM and GC

- [ ] Crafting Interpreters ch. 14–30
- [ ] Roberto Ierusalimschy, Luiz Henrique de Figueiredo, Waldemar Celes, "The Implementation of Lua 5.0",
      Journal of Universal Computer Science, 2005
- [ ] M. Anton Ertl & David Gregg, "The Structure and Performance of Efficient Interpreters",
      Journal of Instruction-Level Parallelism, 2003
- [ ] Paul Wilson, "Uniprocessor Garbage Collection Techniques", IWMM 1992
- [ ] Richard Jones, Antony Hosking, Eliot Moss, *The Garbage Collection Handbook* (2nd ed., 2023). A reference, not a cover-to-cover read.

## Phase 5: packages

- [ ] Russ Cox, "Version SAT" (2016) and "Minimal Version Selection" (2018), research.swtch.com
- [ ] Natalie Weizenbaum, "PubGrub: Next-Generation Version Solving" (2018)

## Phase 6: data and AI

- [ ] Charles Harris et al., "Array programming with NumPy", Nature 585, 2020
- [ ] Jeff Bezanson, Alan Edelman, Stefan Karpinski, Viral Shah, "Julia: A Fresh Approach to Numerical Computing",
      SIAM Review, 2017
- [ ] PEP 3118 (the buffer protocol), PEP 465 (the `@` operator)
- [ ] The Apache Arrow columnar format specification

## Phase 7: concurrency

- [ ] C. A. R. Hoare, "Communicating Sequential Processes", Communications of the ACM, 1978
- [ ] PEP 703, "Making the Global Interpreter Lock Optional in CPython"

## Phase 8: JIT and AOT

- [ ] John Aycock, "A Brief History of Just-In-Time", ACM Computing Surveys, 2003
- [ ] Andreas Gal et al., "Trace-based Just-in-Time Type Specialization for Dynamic Languages", PLDI 2009
- [ ] Carl Friedrich Bolz et al., "Tracing the Meta-Level: PyPy's Tracing JIT Compiler", ICOOOLPS 2009
- [ ] Keith Cooper & Linda Torczon, *Engineering a Compiler*. A reference.

## Security (any time)

- [ ] Ken Thompson, "Reflections on Trusting Trust", Communications of the ACM, 1984 (3 pages)
