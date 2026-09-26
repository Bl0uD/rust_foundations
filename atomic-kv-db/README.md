# 🦀 Thread-Safe Key-Value Store: Concurrency & Shared State

This third project tackles one of the most notoriously difficult subjects in systems programming: multithreading and shared memory. It is a fully concurrent, in-memory Key-Value database where multiple threads can read and write data simultaneously without causing crashes or data corruption.

## 🧠 Key Concepts Explored

In single-threaded environments (like Node.js), you don't have to worry about two processes writing to the same variable at the exact same microsecond. In a multithreaded Rust application, the compiler structurally enforces thread safety, completely eliminating "Data Races".

### 1. The Core Database (`HashMap`)
The foundation is the standard library's `HashMap`, a simple dictionary associating `String` keys with `String` values. On its own, it is not thread-safe.

### 2. The Smart Lock (`RwLock`)
To protect the `HashMap`, we wrap it in a `RwLock` (Read-Write Lock).
- **Multiple Readers:** It allows an infinite number of threads to read the data simultaneously (`.read().unwrap()`).
- **Exclusive Writer:** As soon as a thread wants to write (`.write().unwrap()`), the lock blocks all other readers and writers until the operation is finished, guaranteeing atomic updates.

### 3. Shared Ownership (`Arc`)
Rust's strict ownership rules state that a value can only have one owner. To share our locked database across multiple threads, we wrap it in an `Arc` (Atomic Reference Counted).
Every time we spawn a new thread, we create a clone of this `Arc`. It acts as a smart pointer that keeps a tally of how many active threads are using the database. The memory is only freed when the final thread finishes and the counter hits zero.

```rust
// The ultimate thread-safe Russian doll:
let db: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));

```

### 4. Spawning Threads and `move`

We use `thread::spawn` to spin up parallel execution contexts. The `move` keyword forces the closure to take ownership of the cloned `Arc`, ensuring the thread has a valid reference to the database that will survive as long as the thread is running.

---

## 🌍 Real-World Application: WebSockets & State

This specific architecture is the backbone of high-performance network services. If you are building a real-time chat application or tracking live user presence over WebSockets, this setup is fundamental. It serves as a highly performant, backend equivalent to frontend state management tools like Zustand, ensuring that when dozens of clients connect and broadcast messages simultaneously, your server's global state remains perfectly synchronized without memory leaks.

---

## 🚀 How to Run

Navigate to the project folder and run:

```bash
cargo run

```

*Note: Run the program multiple times. You will notice that the output order changes constantly. This demonstrates the unpredictable nature of OS thread scheduling and proves that readers and writers are executing concurrently in real-time.*