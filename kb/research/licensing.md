# Licensing

*Written 2026-11-14. A learning summary, not legal advice. Based on: choosealicense.com; the SPDX license
list; GitHub Terms of Service §D.6; the GCC Runtime Library Exception; the LLVM exception; the Rust
project's licensing FAQ.*

## Default: all rights reserved

Copyright exists automatically the moment code is written. A public GitHub repo without a license lets
people *view* and fork it on GitHub (per GitHub's terms), but not legally use, modify, or redistribute it.
A license is the permission slip.

## Two families

**Permissive:** do almost anything; keep the copyright notice.

- **MIT:** very short. No explicit patent grant. Used by Node.js, Julia, Lua.
- **BSD-2/3-Clause:** like MIT; the 3-clause version adds "don't use our name to endorse your product". Used by Go.
- **Apache-2.0:** longer. Adds an **explicit patent grant** and patent retaliation (sue over patents and you
  lose your license), and requires stating changes. Used by Kotlin, TypeScript, Swift.

**Copyleft:** share and modify freely, but derivatives must carry the same license.

- **GPL-3.0:** strong. If you distribute a program that includes GPL code, the whole program must be GPL. Used by GCC.
- **LGPL:** weaker. Linking to the library is fine; changes to the library itself stay LGPL.
- **MPL-2.0:** file-level. Modified MPL files stay MPL; the rest of your project can be anything.
- **AGPL-3.0:** like GPL, but also triggered by offering the software over a network (it closes the "SaaS loophole").

## Things specific to programming languages

- **Programs written in Pings are not derivative works of the interpreter.** Running code through an
  interpreter doesn't put that code under the interpreter's license.
- **But the runtime and standard library often end up *inside* user programs**, whether compiled in,
  bundled, or copied. If they were GPL, every Pings program could be forced to be GPL. That's why GCC has
  the *GCC Runtime Library Exception*, Swift has a *Runtime Library Exception*, and LLVM uses
  *Apache-2.0 WITH LLVM-exception*. Under a permissive license the problem doesn't arise.
- **Dual license, "MIT OR Apache-2.0"** (Rust and most of its ecosystem): each user picks one. Apache gives
  patent protection; MIT stays compatible with GPLv2-only projects, which Apache-2.0 isn't.

## Compatibility

Our license must allow everything our dependencies' licenses require. Permissive dependencies (MIT, Apache,
BSD) are fine under anything; a GPL dependency would force the whole binary to be GPL. `cargo-deny` can
enforce an allow-list in CI. SPDX identifiers (`MIT OR Apache-2.0`) make licenses machine-readable.

## Contributions

- **DCO** (Developer Certificate of Origin): contributors add a `Signed-off-by:` line (`git commit -s`),
  certifying they have the right to submit the code. Lightweight; the Linux kernel uses it.
- **CLA** (Contributor License Agreement): contributors sign an agreement, often giving the project owner
  broad rights, sometimes including relicensing. Heavier; it puts off casual contributors.
- **Inbound = outbound:** contributions come in under the project's own license. This is GitHub's default
  (Terms of Service §D.6).

## What a code license doesn't cover

- **Trademarks:** the *name* "Pings" and any logo. Mature projects publish a trademark policy (the Rust
  Foundation, the Python Software Foundation).
- **Docs and this knowledge base:** these could use CC BY 4.0 instead of a code license.
- **Patents:** only Apache-style grants address them.

## Naming is part of this

Choosing the name took four tries, which was itself a lesson (see the 2026-11-14 journal). Check a name
against GitHub, the package registries, GitHub Linguist's list of file extensions, and existing languages
*before* announcing it.

## Our decision

See [ADR 0003](../decisions/0003-license.md).
