# 🦀 Discovering Rust: REST API with Axum

This project is a first approach to the Rust programming language. It is a basic asynchronous REST API built with the **Axum** framework. It serves as a sandbox to understand the fundamental concepts of the language before tackling more complex architectures.

## 🧠 Rust Language Overview

Rust is a compiled systems programming language. It was designed to offer the performance of C or C++, but while guaranteeing absolute memory safety, thus eliminating entire classes of bugs (like *segfaults*). 

Ideal for high-performance backends or intensive WebSocket management, it positions itself as an ultra-fast alternative to environments like Node.js or NestJS, particularly relevant for demanding real-time projects like a multiplayer game (e.g., ft_transcendence).

### ✅ Pros
- **Guaranteed memory safety:** The Ownership system prevents accessing freed memory.
- **Zero Garbage Collector:** Performance is predictable and extremely fast.
- **An exceptional ecosystem (Cargo):** The `cargo` tool unifies package management, compilation, testing, and formatting. It's the `npm`, `webpack`, and `jest` of Rust, all in one.
- **Strong and rigorous typing:** If the code compiles, there is a very high chance it will work in production without unexpected errors.

### ❌ Cons
- **Steep learning curve:** The compiler is very strict (the "Borrow Checker") and forces you to rethink how you structure data.
- **Compilation times:** Longer than interpreted or transpiled languages (like TypeScript).
- **No "magic":** You often have to be explicit (like specifying exactly how to serialize JSON with `Serde`).

---

## 🛠️ Embracing the Compiler (How to Debug)

The Rust compiler can feel intimidating at first, but it is designed to be a helpful pair-programmer rather than an obstacle.

- **Read the terminal output:** When Rust refuses to compile, it doesn't just throw a stack trace. It points directly to the exact line, explains what went wrong (e.g., "expected `struct Json`, found `tuple`"), and often provides the exact code to fix it.
- **`cargo check`:** This is your most used command. It verifies if your code compiles without spending time generating the actual executable. Run this constantly while writing code to catch errors early.
- **`rustc --explain EXXXX`:** Whenever you get a specific error code (like `E0425`), typing `rustc --explain E0425` in your terminal will print a full, detailed explanation of the underlying concept and provide examples of how to resolve it.

---

## 📖 Rust Syntax Cheat Sheet (API Special)

Here are the key concepts covered in this project:

### 1. Macros (`#[...]` and `!`)
Macros generate code for us before compilation.
- `#[tokio::main]`: Transforms a classic function into an asynchronous engine (equivalent to a built-in *Event Loop*).
- `#[derive(Serialize, Deserialize)]`: Allows (like a decorator) to tell the Serde library how to automatically convert a `struct` into JSON, and vice versa.
- `println!()`: The `!` indicates that this is a macro and not a simple function.

### 2. Structures (`struct`)
The equivalent of `interface` or `class` in TypeScript. They define the shape of the data.
```rust
struct CreateUser {
    username: String,
    email: String,
}

```

### 3. Error Handling (`Result`)

In Rust, **we don't throw exceptions** (no `try/catch` or `throw new Error`). A function that can fail returns a `Result<T, E>` enum.

* `Ok(T)`: Contains the value in case of success (e.g., a 201 code with the JSON).
* `Err(E)`: Contains the error in case of failure (e.g., a 400 code with a message).

```rust
// Typical signature of an Axum handler managing an error:
async fn my_handler() -> Result<(StatusCode, Json<Utilisateur>), (StatusCode, String)> { ... }

```

### 4. Axum Extractors

Instead of using tools like `@Body()` in function parameters, Axum deduces what it needs to extract from the HTTP request by reading the parameter's type:

```rust
// Axum automatically extracts the JSON and puts it in "payload"
async fn create_user(Json(payload): Json<CreateUser>)

```

### 5. Implicit Return Operator

If the last expression of a function or a block does not end with a semicolon (`;`), Rust considers it to be the return value.

```rust
// No "return" keyword needed here
Ok((StatusCode::CREATED, Json(new_user)))

```

---

## 🚀 Launching the Project

The project is natively compatible with macOS, Linux, or WSL2.

**Useful commands:**

* `cargo run`: Compiles and runs the server in development mode (on port 3000).
* `cargo check`: Very quickly checks if the code compiles, without generating the executable (very handy for fixing errors).
* `cargo build --release`: Compiles the code with all speed optimizations for production.