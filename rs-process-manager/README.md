# 🦀 Rust Process Manager: System APIs & Mutability

This second project explores how Rust interacts with the underlying operating system. It is a cross-platform command-line interface (CLI) tool that reads and displays active system processes, CPU usage, and RAM consumption in real-time, without invoking any external shell commands.

By utilizing the `sysinfo` crate, this code runs seamlessly across different environments, including macOS native systems and Linux-based environments like WSL2.

## 🧠 Key Concepts Explored

While the first project focused on asynchronous web routing, this project introduces fundamental Rust concepts regarding memory, state, and standard library utilities.

### 1. Immutability by Default (`mut`)
In Rust, variables are immutable by default (similar to `const`). To allow a variable to change its internal state over time, you must explicitly declare it with the `mut` keyword.
```rust
// The system object must be mutable because refreshing processes updates its internal state.
let mut sys = System::new();

```

### 2. Cross-Platform Abstraction (Crates)

Interacting with OS processes usually requires writing OS-specific C bindings. The `sysinfo` crate abstracts this complexity. The compiler handles downloading and linking the correct low-level APIs depending on where the code is compiled (e.g., Apple's `libproc` on macOS or reading `/proc` on Linux).

### 3. Variable Scope and Lifetimes

Variables created inside a block (like a `for` loop) only live within that block. If a calculation (like memory conversion) depends on a temporary loop variable, it must be executed inside that specific scope before the variable is dropped.

### 4. The Concept of "Deltas" and Thread Pausing

CPU usage is a measure of processing time over a specific interval. To calculate it, the program must take two snapshots of the system state. We achieve this by pausing the main execution thread using the standard library.

```rust
use std::thread;
use std::time::Duration;

// Snapshot A
sys.refresh_processes(ProcessesToUpdate::All, true);
// Pause the program for 200 milliseconds
thread::sleep(Duration::from_millis(200));
// Snapshot B (allows the system to calculate the CPU delta)
sys.refresh_processes(ProcessesToUpdate::All, true);

```

### 5. Advanced String Formatting

The `println!` macro contains a powerful micro-language for aligning and formatting text directly in the console, making complex string manipulations unnecessary.

* `{:<14}` : Left-align the variable and pad it to ensure it occupies exactly 14 characters.
* `{:>9.2}` : Right-align the variable on 9 characters, and round the floating-point number to 2 decimal places.
* `{:?}` : The "Debug" formatter. Used here to safely print the process name, even if the OS provides a string that isn't perfectly encoded in UTF-8.

### 6. Type Unwrapping (`as_u32`)

Sometimes, crates wrap primitive types inside custom structures (e.g., `sysinfo::Pid`). To format them properly as standard numbers, we call methods like `.as_u32()` to extract the raw 32-bit unsigned integer.

---

## 🚀 How to Run

Navigate to the project folder and run:

```bash
cargo run

```

*Note: Due to the number of background processes on modern OSes, the output will be extensive. You can scroll through your terminal to see the neatly aligned CPU and RAM metrics.*