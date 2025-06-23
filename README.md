# kedge-wx

**kedge-wx** is a lightweight WebAssembly interpreter written entirely in Rust.
Designed for offline, peer-to-peer, and resource-constrained environments, it enables verifiable execution of WASM logic without relying on cloud or centralized infrastructure.

It is intended to power decentralized applications within the **d-verse** ecosystem — where execution happens locally, and logic remains portable.

---

## ✦ Features

* Pure Rust implementation with no external dependencies
* Compatible with iOS, Android, embedded systems, and command-line interfaces
* Supports a minimal, auditable subset of the WASM MVP spec: integer arithmetic, control flow, and linear memory
* Fully deterministic interpreter (no JIT) for verifiability and consistency
* Extensible hostcall interface (e.g., `log`, `get_metadata`, `eval_trust`)
* Optimized for edge use cases with low memory, limited bandwidth, and intermittent connectivity

---

## 🚀 Use Cases

* Executing local logic in mesh-connected or offline-first applications
* Embedding WASM routines inside decentralized or autonomous agents
* Applying region-specific rules in federated systems
* Serving as the execution layer for `dverse-runtime` in future releases

---

## 🛠 Getting Started

To run a simple WebAssembly module with `kedge-wx`:

```bash
cargo build --release
./target/release/kedge-wx examples/simple.wasm
```

Use Rust to compile a WASM binary with:

```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```bash
rustc --target=wasm32-unknown-unknown -O add.rs
```

---

## 📄 License

MIT © 2024 D-Verse Project
kedge-wx is developed to support local execution, digital autonomy, and decentralized logic.
