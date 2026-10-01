# 🦀 Discovering Rust: REST API with Axum & PostgreSQL

This project is a first approach to the Rust programming language. It is a basic asynchronous REST API built with the **Axum** framework and connected to a **PostgreSQL** database. It serves as a sandbox to understand the fundamental concepts of the language before tackling more complex architectures.

## 🧠 Rust Language Overview

Rust is a compiled systems programming language. It was designed to offer the performance of C or C++, but while guaranteeing absolute memory safety, thus eliminating entire classes of bugs (like *segfaults*). 

Ideal for high-performance backends or intensive WebSocket management, it positions itself as an ultra-fast alternative to environments like Node.js or NestJS, particularly relevant for demanding real-time projects like a multiplayer game.

### ✅ Pros
- **Guaranteed memory safety:** The Ownership system prevents accessing freed memory.
- **Zero Garbage Collector:** Performance is predictable and extremely fast.
- **An exceptional ecosystem (Cargo):** The `cargo` tool unifies package management, compilation, testing, and formatting.
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

---

## 📖 Rust Syntax Cheat Sheet (API Special)

Here are the key concepts covered in this project:

### 1. Modularity and Visibility (`mod` and `pub`)
Unlike Node.js, files in Rust are not automatically modules. Everything is strictly private by default.
- `mod handlers;` in `main.rs` tells the compiler to load the `handlers.rs` file.
- `pub` (public) must be added before structs, their internal fields, and functions to allow other files to access them.

### 2. Error Handling (`Result`)
In Rust, **we don't throw exceptions**. A function that can fail returns a `Result<T, E>` enum.
* `Ok(T)`: Contains the value in case of success.
* `Err(E)`: Contains the error in case of failure.

### 3. Axum Extractors (`Json`, `State`, and `Path`)
Axum deduces what it needs to extract from the HTTP request based on the parameter's type:

```rust
// Extracts the JSON payload AND the shared database connection pool
async fn create_user(State(pool): State<PgPool>, Json(payload): Json<CreateUser>)

// Extracts the dynamic {id} from the URL (/user/1) - strictly uses {} in Axum 0.7+
async fn delete_user(Path(id): Path<i32>)
```

4. Method Chaining for Routes
Axum allows handling multiple HTTP methods on the exact same URL path by chaining the handlers:

```rust
.route("/user/{id}", delete(delete_user).put(update_user))
```

5. Compile-time SQL (sqlx)
SQLx allows us to write raw SQL while validating table structures and types against the database during compilation.

```rust
let user = sqlx::query_as::<_, Utilisateur>("SELECT * FROM utilisateurs")
    .fetch_all(&pool)
    .await;
🚀 Launching the Project
This project requires a PostgreSQL database to run. We use a local Docker container for this.
```

1. Start the PostgreSQL Database (Docker):

```bash
docker run --name ma-base-rust -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=monmotdepasse -e POSTGRES_DB=rust_api_db -p 5432:5432 -d postgres
```

2. Run the Rust Server:

```bash
cargo run
(Table creation is handled automatically on startup).
```

3. Test the Full CRUD (curl commands):

```bash
# CREATE
curl -X POST http://localhost:3000/create_user -H "Content-Type: application/json" -d '{"username": "marvin", "email": "marvin@42.fr"}'

# READ
curl http://localhost:3000/user

# UPDATE
curl -X PUT http://localhost:3000/user/1 -H "Content-Type: application/json" -d '{"username": "marvin_v2", "email": "marvin2@42.fr"}'

# DELETE
curl -X DELETE http://localhost:3000/user/1
```