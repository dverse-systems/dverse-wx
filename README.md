# kedge-wx

kedge-wx is a lightweight, capsule-native WebAssembly runtime written entirely in Rust.  
Built from scratch, it powers the execution of trust-aware, signed `.kapsule` logic in decentralized, offline-first, and edge-computing environments.

It is part of the Kriyo platform — a local-first ecosystem for commerce, communities, and autonomy.

---

## Features

- Pure Rust implementation with no external dependencies, compatible with iOS, Android, CLI, and embedded systems.
- Capsule-aware execution designed specifically for running WASM logic inside cryptographically signed `.kapsule` files.
- Supports a minimal subset of the WASM MVP instruction set including integer arithmetic and control flow.
- Fully deterministic and interpretable runtime without JIT, designed for simplicity, verifiability, and auditability.
- Optimized for edge environments with constrained memory, limited connectivity, and peer-to-peer synchronization.
- Extensible hostcall interface for executing custom functions like `log`, `get_metadata`, or `trust_eval`.

---

## Use Cases

- Executing embedded logic in `.kapsule` files such as ranking, trust evaluation, or moderation.
- Running autonomous agents and validators in decentralized, offline mesh apps.
- Applying region-specific policy logic within federated systems.
- Enabling mobile-first applications that operate independently of centralized cloud services.

---

## Getting Started

To use `kedge-wx`, clone the repository, build it with Cargo, and run it against a compiled WebAssembly file.

You will need a valid `.wasm` file compiled from a trusted source using the `wasm32-unknown-unknown` target. This file should implement a simple capsule logic function, for example, a mathematical operation or policy evaluation routine.

---

## License

kedge-wx is released under the MIT license, © 2024 Kriyaetive Verse.  
It is developed to support local-first software, decentralized trust systems, and digital independence.

---

## Part of the Kriyo Capsule Framework

kedge-wx is a foundational component of the Kriyo architecture. It enables portable, verifiable execution of user-defined logic through signed capsules. This allows Kriyo to support systems for local commerce, regional governance, autonomous collaboration, and peer-to-peer knowledge sharing — without relying on centralized infrastructure.

For more information, visit kriyaetive.com