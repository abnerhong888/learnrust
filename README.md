# create new project
```bash
cargo new <project-name>
```

# commands
| Command | Action | C++ Equivalent |
| :--- | :--- | :--- |
| `cargo build` | Compiles the project (Debug) | `make` or `cmake --build .` |
| `cargo run` | Compiles + executes the binary | `./my_app` |
| `cargo check` | Quickly checks for errors (No binary) | `g++ -fsyntax-only` |
| `cargo build --release` | Compiles with optimizations | `g++ -O3` |
| `cargo clean` | Deletes the target/ folder | `make clean` |

# Rust Keywords Reference for C++ Developers

## 1. Variable & Data Declaration
| Keyword | Description | C++ Equivalent |
| :--- | :--- | :--- |
| `let` | Binds a value to a variable name. | `auto` (mostly) |
| `mut` | Allows a variable to be modified (Variables are immutable by default). | Opposite of `const` |
| `const` | Constant value that is replaced at compile-time. | `constexpr` |
| `static` | A global variable with a fixed memory address. | `static` (global) |
| `type` | Defines a type alias. | `using` or `typedef` |

## 2. Functions & Structures
| Keyword | Description | C++ Equivalent |
| :--- | :--- | :--- |
| `fn` | Defines a function or function pointer. | `void` / `auto` func |
| `struct` | Defines a custom data structure. | `struct` / `class` |
| `enum` | Defines an enumeration (Sum Type / Algebraic Data Type). | `std::variant` |
| `impl` | Implements functionality for a struct or enum. | Member functions / Methods |
| `trait` | Defines shared behavior/interfaces. | Pure Virtual Class / Concepts |
| `self` | Represents the current instance of a method. | `this` pointer |
| `Self` | A type alias for the type implementing the trait. | `decltype(*this)` |

## 3. Control Flow & Pattern Matching
| Keyword | Description | C++ Equivalent |
| :--- | :--- | :--- |
| `if` / `else` | Conditional branching. | `if` / `else` |
| `match` | Pattern matching logic (Exhaustive). | `switch` |
| `loop` | An unconditional infinite loop. | `while(true)` |
| `while` | Loop while a condition is met. | `while` |
| `for` / `in` | Iterates over a range or collection. | Range-based `for` |
| `break` | Exit a loop immediately. | `break` |
| `continue` | Skip to the next iteration of a loop. | `continue` |
| `return` | Return a value from a function. | `return` |

## 4. Modules & Visibility
| Keyword | Description | C++ Equivalent |
| :--- | :--- | :--- |
| `mod` | Defines a module. | `namespace` + file link |
| `use` | Brings a path into scope. | `using namespace` |
| `pub` | Makes an item visible to other modules. | `public:` |
| `crate` | Refers to the current crate root. | Project Root |
| `super` | Refers to the parent module. | `..` in paths |
| `extern` | Links to an external library or crate. | `extern "C"` |

## 5. Memory Safety & Ownership
| Keyword | Description | C++ Equivalent |
| :--- | :--- | :--- |
| `ref` | Bind by reference in a pattern match. | `&` in bindings |
| `move` | Forces a closure to take ownership of its captures. | `std::move` (inside lambda) |
| `unsafe` | Bypasses memory safety checks. | Standard C++ behavior |
| `where` | Adds constraints to generic types. | `requires` (Concepts) |
| `dyn` | Indicates dynamic dispatch (trait objects). | Virtual table / `vtable` |

## 6. Async & Concurrency
| Keyword | Description | C++ Equivalent |
| :--- | :--- | :--- |
| `async` | Defines a function that returns a `Future`. | Coroutine setup |
| `await` | Suspends execution until a `Future` is ready. | `co_await` |


# Rust Macros Reference for C++ Developers

## 1. Declarative Macros (`macro_rules!`)
These are the most common macros, used for code repetition and creating "DSL-like" syntax. They use pattern matching.

| Feature | Description | C++ Equivalent |
| :--- | :--- | :--- |
| `!` | The "invoker" suffix (e.g., `println!`). | No direct equivalent |
| `$(...)` | Indicates a repeating pattern. | Variadic Macros (`...`) |
| `$x:expr` | Matches a Rust expression. | Macro argument |
| `$x:ident` | Matches an identifier (variable/func name). | Token pasting |
| `$x:ty` | Matches a Type. | Not possible in C++ macros |

### Common Built-in Declarative Macros
* `println! / print!`: Standard output with formatting.
* `format!`: Returns a `String` instead of printing.
* `vec!`: Shortcut to create a `Vec<T>` with initial values.
* `panic!`: Stops execution and starts "unwinding" (like throwing an uncatchable exception).
* `todo!`: A placeholder that compiles but panics if reached.

---

## 2. Procedural Macros (The "Hashtags")
These are Rust functions that consume a stream of tokens and output a new stream of tokens at compile-time.

| Type | Syntax | Purpose |
| :--- | :--- | :--- |
| **Derive** | `#[derive(Trait)]` | Automatically implements a Trait for a struct. |
| **Attribute** | `#[cxx::bridge]` | Can transform the code it is attached to. |
| **Function-like**| `my_macro!(...)` | Used for complex logic like SQL validation or HTML templates. |

---

## 3. Comparison: Rust Macros vs. C++ Preprocessor

| Feature | C++ (`#define`) | Rust Macros |
| :--- | :--- | :--- |
| **Logic** | Text Replacement | **AST Transformation** |
| **Hygiene** | None (Leaky scopes) | **Hygienic** (Safe namespacing) |
| **Type Awareness** | Blind to types | **Syntactically aware** |
| **Debugging** | Very difficult | **Better** (via `cargo expand`) |

---

## 4. Useful Macro Attributes
| Attribute | Description |
| :--- | :--- |
| `#[macro_export]` | Makes a macro available to other crates. |
| `#[macro_use]` | Imports macros from another module/crate. |
| `#[cfg(...)]` | Conditional compilation (The `#ifdef` replacement). |
| `#[test]` | Marks a function for the built-in test runner. |