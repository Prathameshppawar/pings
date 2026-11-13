# Glossary

Terms in roughly the order we'll meet them. Add one whenever a new word comes up.

- **Lexer (scanner):** turns source text into a list of tokens.
- **Token:** a categorized piece of source text: `let` (keyword), `x` (identifier), `=` (operator), `42`
  (number literal). The exact characters are its **lexeme**.
- **Span:** where a token or AST node sits in the source (start and end byte offsets, which map to line and
  column). Error messages need it.
- **Parser:** turns tokens into a tree that reflects the program's structure.
- **AST (abstract syntax tree):** that tree, without punctuation noise. `1 + 2 * 3` becomes `Add(1, Mul(2, 3))`.
- **Grammar:** the rules for which token sequences are valid programs.
- **Precedence:** which operator binds tighter (`*` before `+`). **Associativity:** how a chain of the same
  operator groups (`a - b - c` is `(a - b) - c`).
- **Interpreter:** a program that runs another program directly.
- **Compiler:** a program that translates a program into another form: bytecode, machine code, or another language.
- **Bytecode:** compact instructions for a virtual machine rather than a real CPU.
- **VM (virtual machine):** a program that executes bytecode.
- **JIT:** compiling to machine code while the program runs.
- **AOT:** compiling to machine code before the program runs.
- **IR (intermediate representation):** a compiler's internal form, between the AST and the final output.
- **GC (garbage collector):** automatically frees memory the program can no longer reach.
- **FFI (foreign function interface):** calling code written in another language, usually C, from ours.
- **ABI (application binary interface):** the binary-level calling convention that two pieces of compiled code agree on.
- **WASM (WebAssembly):** a portable bytecode that browsers and standalone runtimes execute.
- **REPL:** Read-Eval-Print Loop, the interactive prompt.
- **ADR (architecture decision record):** a short document recording one decision, its options, and its consequences.
