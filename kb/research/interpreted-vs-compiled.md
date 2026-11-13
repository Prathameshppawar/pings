# Interpreted vs compiled

*Written 2026-11-14. Based on: Crafting Interpreters ch. 2 "A Map of the Territory"; Aycock (2003);
Ertl & Gregg (2003); Ierusalimschy et al., "The Implementation of Lua 5.0" (2005); CPython PEPs 659 and 744.*

## The key idea

"Interpreted" and "compiled" describe **implementations**, not languages. Python has a bytecode compiler
plus interpreter (CPython), a tracing JIT (PyPy), and has been compiled to C (Cython). JavaScript is
interpreted *and* JIT-compiled inside the same engine. Almost every real implementation is a pipeline:

```
source text → lexer → tokens → parser → AST → [checks] → one of the back ends below
```

The real question is what happens after the AST.

## The back ends, simplest to hardest

### 1. Tree-walking interpreter

Run the program by recursively evaluating AST nodes: `eval(Add(l, r)) = eval(l) + eval(r)`.

- Examples: Ruby before 1.9, most teaching languages, jlox in Crafting Interpreters.
- \+ Simplest to write and debug; instant startup; an easy REPL.
- − Slow. Every operation chases pointers through heap-allocated nodes (bad for CPU caches) and goes
  through a big recursive `match`.

### 2. Bytecode virtual machine

Compile the AST into a flat array of compact instructions for an imaginary machine, then loop:
fetch an instruction, decode it, execute it.

- Examples: CPython, Lua, Ruby (YARV, since 1.9), Erlang (BEAM), the JVM and .NET before their JITs kick in.
- **Stack-based** (CPython, JVM: `PUSH a; PUSH b; ADD`) vs **register-based** (Lua 5, Dalvik:
  `ADD r0, r1, r2`). Register VMs run fewer instructions; stack VMs are easier to compile to. The Lua 5.0
  paper explains why Lua switched to registers.
- \+ Several times faster than a tree-walker; bytecode can be cached (Python's `.pyc`); portable; keeps the REPL.
- − Two things to write (a compiler and a VM), and harder to debug.

### 3. JIT (just-in-time) compilation

Start by interpreting, find the hot code, and compile it to machine code while the program runs, using the
types actually observed at runtime.

- **Method JIT** (V8, HotSpot) compiles whole hot functions. **Tracing JIT** (LuaJIT, PyPy, TraceMonkey)
  records the path a hot loop actually takes and compiles that trace.
- \+ Can get dynamic languages close to native speed.
- − Very complex (deoptimization, inline caches, type feedback). Slow warm-up, high memory use. It needs
  memory that is both writable and executable, which is a security surface, and iOS forbids it for
  third-party apps.

### 4. Ahead-of-time (AOT) native compilation

Translate the whole program to machine code before it runs.

- Examples: C, Go, Rust, Swift. Back ends: write your own, or emit IR for **LLVM** (Clang, Rust, Swift,
  Julia), **Cranelift** (Wasmtime, an alternative Rust backend), or **QBE** (tiny, hobby-friendly).
- \+ Fastest execution; one self-contained binary; no interpreter to ship.
- − Needs static types to be really fast. A compile step before every run, hard REPLs, and a separate
  build for every OS and CPU.

### 5. Transpiling to another language

Emit C, JavaScript, Go, or similar, and let that toolchain do the rest.

- Examples: TypeScript → JS, Nim → C, early C++ (Cfront → C).
- \+ Cheap access to a mature optimizer and platform.
- − You debug through generated code, and the target's semantics leak through (overflow, strings, errors).

### 6. WebAssembly, two separate ideas

WASM is a portable bytecode that browsers and standalone runtimes (Wasmtime, Wasmer) run at near-native speed.

- Compile **our interpreter** to WASM → Pings runs in a browser. That's the Phase 2 playground.
- Compile **Pings programs** to WASM → a Phase 8 option.

## Summary

| | Effort | Speed | Startup | REPL | You ship |
|---|---|---|---|---|---|
| Tree-walker | low | baseline | instant | easy | the interpreter |
| Bytecode VM | medium | several × baseline | instant | easy | the interpreter |
| JIT | very high | much faster on hot code | warm-up | easy | a big runtime |
| AOT native | high | fastest | instant | hard | a binary per platform |
| Transpile | medium | depends on target | depends | hard | the target toolchain |

The speed column is a rule of thumb, not a measurement. Measuring our own numbers is a Phase 4 task.

## Where real languages sit

- **Python (CPython):** bytecode on a stack VM. 3.11 added a specializing adaptive interpreter (PEP 659);
  3.13 added an experimental JIT (PEP 744).
- **JavaScript (V8):** Ignition (bytecode interpreter) → Sparkplug → Maglev → TurboFan (optimizing JIT).
- **Go:** AOT to native with a deliberately fast compiler; no VM.
- **Java:** AOT to JVM bytecode, interpreted at first, then JIT-compiled (HotSpot).
- **Julia:** JIT via LLVM, specializing each method on its argument types.
- **Lua:** register VM; LuaJIT adds a tracing JIT.

## For Pings

See [ADR 0002](../decisions/0002-execution-model.md). In short: a tree-walker to learn, a bytecode VM to be
fast enough, and JIT or AOT only once measurements justify it.

## Questions to think about

- Python feels instant to run while Go has a compile step, yet Go programs run much faster. Why both?
- What does a JIT know at runtime that an AOT compiler can't know?
- Once the VM exists, what is the tree-walker still good for?
