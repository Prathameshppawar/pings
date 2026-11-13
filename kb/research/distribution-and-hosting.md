# Distribution and hosting

*Written 2026-11-14. Based on: wasm-bindgen and cargo-dist docs; Docker, GHCR, gVisor, and Firecracker
docs; the Go blog post on the playground moving to gVisor.*

How does someone get from "I heard about Pings" to running Pings code? Five paths, cheapest first.

## 1. Browser playground: zero install

Compile the *interpreter* (not Pings programs) to WebAssembly and run it **in the visitor's browser**.

- Rust → `wasm32-unknown-unknown` + `wasm-bindgen` → a `.wasm` file plus some JS glue.
- Host it as static files on GitHub Pages or Cloudflare Pages: no servers to run, pay for, or patch.
- Security win: code runs inside the visitor's own browser sandbox, not on our machines.
- Must-haves: run the interpreter in a **Web Worker** so an infinite loop can't freeze the page, kill the
  worker on timeout, and add an instruction budget ("fuel") inside the interpreter as a second guard.
- By contrast, the official Go and Rust playgrounds run code **on a server** in a sandbox, because their
  compilers are too big to ship to a browser.

## 2. Prebuilt binaries

- CI builds `pings` for each target: `x86_64`/`aarch64` Linux (musl, fully static), `x86_64`/`aarch64`
  macOS, and `x86_64` Windows.
- Static musl binaries run on any Linux distribution.
- **cargo-dist** (`dist`) generates the GitHub Actions release workflow, a `curl … | sh` installer, a
  PowerShell installer, and a Homebrew formula from one config.
- Publish checksums and GitHub artifact attestations (SLSA provenance), so users can verify that a binary
  came from our CI and wasn't tampered with.

## 3. Package managers

- `cargo install pings` for people who already have Rust (the crate name is free on crates.io).
- A Homebrew tap: a repo named `homebrew-tap`, then `brew install prathameshppawar/tap/pings`. Getting into
  homebrew-core comes later and requires notability.
- Windows: Scoop or winget. Linux: AUR and Nix, usually contributed by the community.
- A version manager (like rustup, pyenv, or nvm) only once people need several versions side by side.

## 4. Containers

A container image packages a binary together with the filesystem it needs. Our binary is static, so the
image can be almost empty. Sketch (the real Dockerfile comes in Phase 2):

```dockerfile
FROM rust:1.99 AS build
WORKDIR /src
COPY . .
RUN rustup target add x86_64-unknown-linux-musl \
 && cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=build /src/target/x86_64-unknown-linux-musl/release/pings /pings
ENTRYPOINT ["/pings"]
```

- Build multi-arch images (linux/amd64 and linux/arm64) with `docker buildx`, and publish to GitHub
  Container Registry as `ghcr.io/prathameshppawar/pings`.
- Usage: `docker run --rm -v "$PWD":/src ghcr.io/prathameshppawar/pings run /src/main.pi`
- Good for CI pipelines, servers, and trying Pings without installing anything on the host.
- **A container on its own isn't a security sandbox.** It shares the host's kernel; see the next section.

## 5. Running untrusted code on a server (only if we ever need it)

Reasons we might: long-running jobs, GPUs, shareable results. Isolation options, weakest to strongest:

| Layer | What it is | Used by |
|---|---|---|
| Container + seccomp + no network + cgroup limits | Linux kernel features | Docker defaults (weak alone) |
| nsjail, bubblewrap | process sandbox tools | Compiler Explorer (nsjail) |
| gVisor | a user-space kernel that intercepts syscalls | Go playground, Google Cloud Run |
| Firecracker microVM | a lightweight VM per job | AWS Lambda, Fly.io |
| WASM runtime with fuel | no syscalls unless explicitly granted | Fastly Compute (Wasmtime) |

Always: CPU, memory, and wall-time limits, a read-only filesystem, no network, capped output, and rate limiting.

## Docs hosting

mdBook (what the Rust Book uses) on GitHub Pages, next to the playground.

## Plan

Phase 2: a client-side playground (WASM), binaries via cargo-dist, a Docker image on GHCR, and an install
script. Running code server-side isn't planned.
