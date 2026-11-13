# Security

*Written 2026-11-14. Based on: the Rust reference on `unsafe`; the Deno permissions docs; the WASI
capability model; the RustSec advisory database; GitHub's guide to hardening Actions; SLSA; Ken Thompson,
"Reflections on Trusting Trust" (1984).*

A programming language has four separate attack surfaces. Most new languages only think about the first.

## 1. The implementation: our Rust code

Threat: a malicious Pings program or input file crashes the interpreter or takes it over.

- **Memory safety:** Rust prevents most memory bugs, except inside `unsafe`. The workspace denies
  `unsafe_code`. The GC (Phase 4) is the likeliest place we'll need an exception, and each one gets a
  justification and extra tests.
- **Panics are bugs:** any input that makes the interpreter panic is a bug to fix (and a denial-of-service
  attack in a server context).
- **Stack overflow:** a tree-walker recurses on the host's stack, so `fn f() { f() }` or deeply nested
  `((((…))))` can overflow it. The parser and the evaluator both need depth limits.
- **Resource exhaustion:** infinite loops, huge allocations (`"a" * 10**12`), pathological regexes. Sandboxed
  runs need instruction budgets ("fuel") and memory limits.
- **Hash flooding (HashDoS):** if an attacker picks keys that all collide, dictionary operations become
  quadratic. Rust's `HashMap` defaults to randomly keyed SipHash; keep that for Pings maps that hold user data.
- **Fuzzing:** feed random and mutated input to the lexer, parser, and VM to find crashes (`cargo-fuzz` with
  libFuzzer). Starts in Phase 1.

## 2. Programs written in Pings: what code is allowed to do

Python, Node, and Go give every program the full power of the user account: read `~/.ssh`, open sockets,
start processes. One malicious dependency means full compromise.

The **capability-based** alternative (Deno, WASI) gives a program no access to files, network, environment,
or subprocesses unless you grant it:

```
pings run --allow-read=./data --allow-net=api.example.com main.pi
```

**Proposal:** Pings is **sandboxed by default**. That's far easier to build in from the start than to
retrofit; Node.js only gained an experimental permission model in v20 (2023).

## 3. The supply chain: dependencies, CI, releases, packages

- **Our dependencies:** `cargo-audit` (known vulnerabilities from the RustSec database), `cargo-deny`
  (licenses, banned crates, duplicate versions), Dependabot.
- **Our CI:** pin GitHub Actions to commit SHAs (tags can be moved), use least-privilege `permissions:`,
  and never expose secrets to pull-request workflows. Already done in `.github/workflows/ci.yml`.
- **Our releases:** checksums, artifact attestations and SLSA provenance, eventually Sigstore signatures,
  and reproducible builds.
- **Pings packages (Phase 5):** lockfile hashes, no install-time scripts by default (npm's `postinstall` is
  a major malware vector), trusted publishing, and typosquatting checks.
- **Bootstrapping:** Thompson's "Reflections on Trusting Trust" shows why you can't fully trust a compiler
  you didn't build yourself, and why reproducible builds matter.

## 4. Hosted services: playground, docs, registry

- The playground runs client-side, so our server only serves static files.
- If we ever run code server-side, see [distribution-and-hosting.md §5](distribution-and-hosting.md).

## Project process (Phase 2)

- A `SECURITY.md` explaining how to report a vulnerability privately; turn on GitHub's private
  vulnerability reporting.
- Publish a GitHub Security Advisory (with a CVE if users are affected) for every security fix.
