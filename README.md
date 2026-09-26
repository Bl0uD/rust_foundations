# 🦀 Rust Foundations: A Learning Journey

This repository documents my progression into systems programming with Rust. Coming from high-level, single-threaded web frameworks, this workspace serves as a structured exploration of Rust's core paradigms: memory safety, strict typing, low-level system interactions, and fearless concurrency.

This progression builds the foundation necessary for migrating complex, real-time architectures—such as live chat systems or multiplayer game backends—into a highly performant, memory-safe, and natively multithreaded environment.

## 📂 Repository Structure

This mono-repo is divided into three distinct projects, each focusing on a specific domain of the Rust ecosystem. 

### 1. [REST API with Axum](./axum-rest-api/)
A foundational asynchronous web service.
* **Focus:** Routing, asynchronous runtimes (`tokio`), JSON serialization (`serde`), and structural error handling without exceptions (`Result<T, E>`).
* **Why it matters:** Establishes the baseline for building fast, reliable HTTP backends.

### 2. [CLI Process Manager](./rs-process-manager/)
A cross-platform system utility tool interacting directly with OS APIs.
* **Focus:** Immutability by default, variable lifetimes, standard library utilities (thread pausing for delta calculations), and complex string formatting.
* **Why it matters:** Demonstrates how Rust replaces C/C++ for low-level system tools while maintaining cross-platform compatibility without sacrificing memory safety.

### 3. [Atomic Key-Value Store](./atomic-kv-db/)
A fully multithreaded, in-memory database.
* **Focus:** Concurrency, preventing Data Races, Shared State (`Arc`), and Smart Locks (`RwLock`).
* **Why it matters:** This is the core architecture required for managing global state in high-performance network services (like handling dozens of concurrent WebSocket connections) without memory leaks or segmentation faults.

## 🚀 Getting Started

Each project is completely self-contained. To run or explore a specific project, navigate to its respective directory and consult its local `README.md` for detailed technical notes and launch instructions.

```bash
cd axum-rest-api
cargo run