# Pings: vision

*Draft, 2026-11-14. Everything here is open to change. Changes that matter get an ADR.*

## In one line

A small, readable language with Go's simplicity and Python's comfort for data work, built from scratch
so we understand every layer.

## Why build it

1. **To learn.** This is the main goal. When "make it faster or fancier" conflicts with "understand it",
   understanding wins. We use a library for something only after we've built it once or studied how it works.
2. **To ship something real.** A public playground and installable releases, so other people try it and
   break it. Real users find the bugs and questions textbooks don't.

## Goals, ranked

When two goals conflict, the higher one wins.

| # | Goal | Concretely |
|---|------|------------|
| 1 | Understandable implementation | Readable code, every decision recorded, tests for every feature. |
| 2 | Simple language | A grammar you can hold in your head; one obvious way to do common things. |
| 3 | Safe to run | Memory-safe implementation. Programs get no file, network, or environment access unless granted. |
| 4 | Easy to try | Browser playground with nothing to install; a single binary; a Docker image. |
| 5 | Good at data and AI work | Native n-dimensional arrays, vectorized operations, notebooks, calls into fast native libraries. |
| 6 | Fast enough | A bytecode VM before any JIT. Heavy numeric work runs in Rust or BLAS, not in our interpreter loop. |

## What "as flexible as Go" means

Go's flexibility doesn't come from having many features. It comes from a few that compose well:

- **Structural interfaces.** A type satisfies an interface just by having the methods; no `implements`.
- **Lightweight concurrency.** Goroutines and channels, from Hoare's CSP (1978).
- **Tooling in the box.** `go fmt`, `go test`, `go mod`, all in one binary.
- **Fast feedback.** Compiles fast, starts fast.

Pings wants all four. The last one is a big reason we start with an interpreter.

## What "AI/DS like Python" means

Python is slow at running Python. It wins at AI anyway because the heavy math runs in C, Fortran, and CUDA
libraries, Python only orchestrates them, and it has good notebooks, math-friendly syntax, and a huge
ecosystem. Details and what we can copy: [research/python-and-ai.md](../research/python-and-ai.md).

## Non-goals (for now)

- Beating C, Rust, or Go on raw speed.
- Manual memory management or systems programming. Pings has a garbage collector.
- Writing GPU kernels in Pings.
- Being a superset of Python or JavaScript.

## Strawman syntax

**Nothing below is decided.** It exists so we have something concrete to argue with. The numbers refer
to the open questions underneath.

```
// variables: let/var, Go's :=, or Python's plain x = 1?      (Q1)
let name = "world"
var count = 0

// functions: are type annotations optional or required?      (Q2, ADR 0004)
fn greet(who: str) -> str {
    return "hello, " + who
}

// closures and higher-order functions
let squares = [1, 2, 3, 4].map(fn(x) { x * x })

// structs and methods: Go-style receivers or impl blocks?    (Q3)
struct Point { x: f64, y: f64 }

fn (p: Point) norm() -> f64 {
    return sqrt(p.x * p.x + p.y * p.y)
}

// errors: exceptions, Go's (value, err), or Result + ?        (Q4)
let text = read_file("data.csv")?

// data: tensors are built in, @ is matrix multiply           (Phase 6)
let a = tensor([[1.0, 2.0], [3.0, 4.0]])
let b = a @ a.T

// concurrency: Go-style tasks and channels                   (Phase 7)
let ch = channel()
spawn worker(ch)
print(ch.recv())
```

## Open questions

1. Bindings: `let`/`var` (Rust, Swift), `:=` (Go), or plain `x = 1` (Python)?
2. Typing discipline: see [ADR 0004](../decisions/0004-typing.md).
3. Methods: Go receivers, `impl` blocks, or classes?
4. Errors: exceptions, multiple return values, or `Result` with `?`?
5. Braces (Go) or significant indentation (Python)? Braces are easier to parse and survive copy-paste
   into chats and web forms; indentation is less noisy. **The lexer depends on this one.**
6. Is there a `nil`, or an `Option` type instead?
7. Integers: arbitrary precision (Python) or fixed 64-bit (Go)? What happens on overflow?
