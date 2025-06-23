# dverse-wx

**dverse-wx** is a lightweight WebAssembly interpreter written entirely in Rust.
Built for local-first and peer-to-peer environments, it enables deterministic execution of WASM logic on mobile, embedded, and offline systems — without relying on centralized infrastructure.

It is a foundational component in the **d-verse** stack, designed to power verifiable logic at the edge.

---

## ✦ Features

* Pure Rust implementation with no external dependencies
* Runs on iOS, Android, embedded platforms, and CLI environments
* Supports a minimal subset of the WASM MVP spec: integer operations, control flow, linear memory
* Fully deterministic interpreter (no JIT), enabling transparent and auditable behavior
* Extensible hostcall system for logic like `log`, `get_metadata`, and `evaluate`
* Optimized for edge computing: small footprint, low memory, works offline

---

## 🚀 Use Cases

* Running embedded WASM logic in decentralized, mobile, or offline-first apps
* Executing autonomous routines in trust-aware peer-to-peer environments
* Supporting local decision-making, moderation, or policy enforcement
* Serving as the low-level engine for `dverse-runtime` in future releases

---

## 🛠 Getting Started

To build and run a WASM file with `dverse-wx`:

```sh
cargo build --release
./target/release/dverse-wx examples/add.wasm
```

Example source (compiled with `wasm32-unknown-unknown`):

```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```sh
rustc --target=wasm32-unknown-unknown -O add.rs
```

---

## 📄 License

MIT © 2024 D-Verse Project
`dverse-wx` is developed to support local-first software, decentralized logic, and digital autonomy.
