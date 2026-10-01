# 🦀 Discovering Rust: REST API with Axum & PostgreSQL

This project is a first approach to the Rust programming language. It is a basic asynchronous REST API built with the **Axum** framework and connected to a **PostgreSQL** database. It serves as a sandbox to understand the fundamental concepts of the language before tackling more complex architectures.

## 🧠 Rust Language Overview

Rust is a compiled systems programming language. It was designed to offer the performance of C or C++, but while guaranteeing absolute memory safety, thus eliminating entire classes of bugs (like *segfaults*). 

Ideal for high-performance backends or intensive WebSocket management, it positions itself as an ultra-fast alternative to environments like Node.js or NestJS, particularly relevant for demanding real-time projects like a multiplayer game (e.g., ft_transcendence).

### ✅ Pros
- **Guaranteed memory safety:** The Ownership system prevents accessing freed memory.
- **Zero Garbage Collector:** Performance is predictable and extremely fast.
- **An exceptional ecosystem (Cargo):** The `cargo` tool unifies package management, compilation, testing, and formatting. It's the `npm`, `webpack`, and `jest` of Rust, all in one.
- **Strong and rigorous typing:** If the code compiles, there is a very high chance it will work in production without unexpected errors (even SQL queries are checked at compile time!).

### ❌ Cons
- **Steep learning curve:** The compiler is very strict (the "Borrow Checker") and forces you to rethink how you structure data.
- **Compilation times:** Longer than interpreted or transpiled languages (like TypeScript).
- **No "magic":** You often have to be explicit (like specifying exactly how to serialize JSON with `Serde` or managing visibility with `pub`).

---

## 🛠️ Embracing the Compiler (How to Debug)

The Rust compiler can feel intimidating at first, but it is designed to be a helpful pair-programmer rather than an obstacle.

- **Read the terminal output:** When Rust refuses to compile, it doesn't just throw a stack trace. It points directly to the exact line, explains what went wrong, and often provides the exact code to fix it.
- **`cargo check`:** This is your most used command. It verifies if your code compiles without spending time generating the actual executable. Run this constantly while writing code to catch errors early.
- **`rustc --explain EXXXX`:** Whenever you get a specific error code, typing it in your terminal will print a full, detailed explanation of the underlying concept.

---

## 📖 Rust Syntax Cheat Sheet (API Special)

Here are the key concepts covered in this project:

### 1. Modularity and Visibility (`mod` and `pub`)
Unlike Node.js, files in Rust are not automatically modules. Everything is strictly private by default.
- `mod handlers;` in `main.rs` tells the compiler to load the `handlers.rs` file.
- `pub` (public) must be added before structs, their internal fields, and functions to allow other files to access them.

### 2. Macros (`#[...]` and `!`)
Macros generate code for us before compilation.
- `#[tokio::main]`: Transforms a classic function into an asynchronous engine (Event Loop).
- `#[derive(Serialize, Deserialize)]`: Tells `Serde` how to convert a `struct` into JSON, and vice versa.
- `println!()`: The `!` indicates that this is a macro and not a simple function.

### 3. Error Handling (`Result`)
In Rust, **we don't throw exceptions**. A function that can fail returns a `Result<T, E>` enum.
* `Ok(T)`: Contains the value in case of success.
* `Err(E)`: Contains the error in case of failure.

### 4. Axum Extractors (`Json` and `State`)
Axum deduces what it needs to extract from the HTTP request based on the parameter's type:
```rust
// Extracts the JSON payload AND the shared database connection pool
async fn create_user(State(pool): State<PgPool>, Json(payload): Json<CreateUser>)
5. Compile-time SQL (sqlx)
SQLx allows us to write raw SQL while validating table structures and types against the database during compilation.

Rust
let user = sqlx::query_as::<_, Utilisateur>("SELECT * FROM utilisateurs")
    .fetch_all(&pool)
    .await;
6. Implicit Return Operator
If the last expression of a function or a block does not end with a semicolon (;), Rust considers it to be the return value. No "return" keyword needed.

🚀 Launching the Project
This project requires a PostgreSQL database to run. We use a local Docker container for this.

1. Start the PostgreSQL Database (Docker):

Bash
docker run --name ma-base-rust -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=monmotdepasse -e POSTGRES_DB=rust_api_db -p 5432:5432 -d postgres
2. Run the Rust Server:

cargo run: Compiles and runs the server in development mode (on port 3000). Table creation is handled automatically on startup.

cargo check: Very quickly checks if the code compiles.