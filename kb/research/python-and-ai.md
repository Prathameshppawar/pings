# How Python became the AI language, and what Pings can copy

*Written 2026-11-14. Based on: Harris et al., "Array programming with NumPy" (Nature, 2020); PEP 3118;
PEP 465; Bezanson et al., "Julia" (SIAM Review, 2017); NumPy, Apache Arrow, and DLPack documentation.*

## The paradox

Pure-Python loops are slow, often 10–100× slower than C for the same arithmetic. Yet nearly all machine
learning is written in Python. How?

## 1. Python orchestrates; native code computes

```python
c = a @ b    # one line of Python
```

That line crosses into C **once**. NumPy hands the work to a BLAS library (OpenBLAS, Intel MKL, Apple
Accelerate), which multiplies the matrices using SIMD instructions, cache blocking, and multiple threads.
The interpreter's overhead is paid once per *call*, not once per *element*. PyTorch works the same way:
Python → C++ (ATen) → CUDA/cuDNN on the GPU.

**Lesson:** a language for numeric work doesn't need a fast scalar loop first. It needs a fast *array* type
and cheap calls into native code.

## 2. The ndarray: memory layout is the API

A NumPy array is a pointer to one contiguous buffer, plus a dtype, a shape, and strides.

- A `(2, 3)` array of `float64` in row-major order has strides `(24, 8)`: move 24 bytes for the next row,
  8 bytes for the next column.
- Transposing swaps the strides; no data is copied. Slicing creates a new offset and strides; still no copy.
- Broadcasting uses a stride of 0 to repeat a dimension without copying it.

## 3. Zero-copy sharing between libraries

- The **buffer protocol** (PEP 3118) lets any Python library expose its memory to any other.
- **DLPack** does the same for tensors across frameworks (PyTorch ↔ JAX ↔ CuPy), including GPU memory.
- **Apache Arrow** standardizes columnar memory for tables. pandas 2, Polars, DuckDB, and Spark all speak it.

## 4. A C API that made wrapping easy

CPython's C API, and later Cython, pybind11, and PyO3 (for Rust), let library authors wrap decades of
existing C and Fortran numerics (BLAS, LAPACK) instead of rewriting them.

## 5. Syntax that reads like math

Operator overloading, slicing like `a[1:, ::2]`, and the `@` operator, which PEP 465 added in Python 3.5
just for matrix multiplication.

## 6. Interactive exploration

REPL → IPython → Jupyter notebooks. Data work is exploratory, and a compile step breaks the flow.

## 7. Ecosystem gravity

NumPy (2006, from Numeric and Numarray) → SciPy, pandas, scikit-learn → Theano, TensorFlow, PyTorch. Each
built on the last. This is the part no new language can copy quickly.

## Others who tried

- **Julia:** designed for numerics. JIT via LLVM plus multiple dispatch makes ordinary user-written loops
  fast too ("solving the two-language problem"). Smaller ecosystem.
- **R:** statistics first, vectorized by default.
- **Mojo:** Python-like syntax compiled through MLIR, aimed at AI hardware.
- **Swift for TensorFlow:** Google's attempt, archived in 2021. A good language didn't beat ecosystem gravity.

## What Pings can do (Phase 6)

1. A built-in `tensor` type with NumPy-style strided layout, implemented in Rust. We write our own first to
   learn; later it might sit on the `ndarray` crate.
2. Vectorized operations and broadcasting, so loops run in Rust, not in the VM.
3. BLAS for matrix multiply: Accelerate on macOS, OpenBLAS elsewhere.
4. Dataframes by binding **Polars** (Rust, Arrow-based) instead of writing our own.
5. Arrow and DLPack compatibility, so data moves to and from Python tools without copying.
6. A Jupyter kernel, by implementing the Jupyter messaging protocol (Evcxr does this for Rust).
7. Maybe Python interop via PyO3 (calling PyTorch from Pings), the cheapest way to borrow an ecosystem.
8. Pure-Rust ML frameworks to bind to: **candle** (Hugging Face) and **burn**.

## Questions to think about

- If a Pings `for` loop over a million numbers is slow, is that a problem? When?
- What does Julia do that makes user-written loops fast, and what does that cost it?
