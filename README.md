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

# Rust Macro Features vs. C++ Comparison

This guide provides an exhaustive breakdown of Rust's macro system, categorized by declarative and procedural features, and how they map to C++ metaprogramming concepts (Macros, Templates, and C++26 Reflection).

---

## 1. Declarative Macros (`macro_rules!`)
Declarative macros use structural pattern matching on tokens. Unlike C++ macros, they are **hygienic** and operate on the **Abstract Syntax Tree (AST)** fragments rather than raw text.

### Feature & Designator Mapping

| Feature | Rust Syntax Example | Description | C++ Equivalent |
| :--- | :--- | :--- | :--- |
| **Simple Fragment** | `($e:expr)` | Captures a single code expression. | `#define MACRO(e) (e)` |
| **Variadic Repetition** | `$($x:expr),*` | Matches zero or more items (separated by commas). | Variadic Templates `Args&&...` |
| **Internal Matching** | `(ident => $i:ident)` | Literal keywords to create custom DSL syntax. | Template Specialization / SFINAE |
| **Hygiene** | N/A (Automatic) | Macro-local variables do not collide with callers. | Manual naming (e.g., `__tmp_##__LINE__`) |
| **Multiple Overloads** | `() => {}; ($e:expr) => {};` | Logic branches based on the input pattern. | Function Overloading or `if constexpr` |
| **Token Tree (tt)** | `($($t:tt)*)` | Matches any valid token or delimited group. | No equivalent (C++ macros are raw text) |
| **Recursive Calls** | `my_macro!($($rest)*)` | A macro calling itself to process nested data. | Template Metaprogramming (TMP) |

### Designator Types (The "What" of Capturing)

| Designator | Matches... | C++ Approach |
| :--- | :--- | :--- |
| **`expr`** | Expressions (`2 + 2`, `func()`) | Macro arguments or `auto` params |
| **`ident`** | Identifiers/Names (`my_var`) | Token pasting using `##` |
| **`ty`** | Types (`i32`, `Vec<u8>`) | `template <typename T>` |
| **`path`** | Paths (`std::collections::HashMap`) | Namespaced strings or `type_traits` |
| **`stmt`** | Statements (`let x = 1;`) | `do { ... } while(0)` macro wrappers |
| **`block`** | Code blocks (`{ ... }`) | Lambdas or raw macro blocks |
| **`meta`** | Attributes (`#[test]`) | C++20 Attributes `[[attribute]]` |
| **`literal`** | Constants (`"hi"`, `42`) | `constexpr` values or literals |

---

## 2. Procedural Macros
Procedural macros are functions that act as compiler plugins. They accept a `TokenStream` and return a `TokenStream`.

| Feature | Rust Mechanism | C++ Equivalent |
| :--- | :--- | :--- |
| **Derive Macros** | `#[derive(Serialize)]` | C++26 Reflection + Injection (Proposed) |
| **Attribute Macros** | `#[my_attr] fn func() {}` | No direct equivalent; requires external codegen |
| **Function-like** | `sqlx::query!("...")` | External build tools or complex `constexpr` |
| **Introspection** | Can iterate over struct fields/names. | C++26 `template for` and `^^T` reflection |

---

## 3. Key Philosophical Differences

> **Hygiene:** Rust macros have "Scope Safety." If a Rust macro defines a variable `x`, it will not overwrite a variable `x` in the function that calls it. C++ macros are "unhygienic" text-replacements that frequently cause name collisions.

> **Syntax vs. Types:** Rust macros run **before** type checking. They see the code as structure (tokens). C++ templates run **during** type checking. They see the code as semantics (types).

---

### Comparison Summary Table

| Capability | Rust (2026) | C++ (2026/Future) |
| :--- | :--- | :--- |
| **Reflective Access** | Via Procedural Macros (parsing AST) | Via Native Reflection (`std::meta`) |
| **Variadics** | Structural Repetition (`$()*`) | Variadic Templates (`...`) |
| **Safety** | High (Parsed as Tokens) | Low (Parsed as Text/Strings) |
| **DSL Support** | Excellent (Custom Syntax) | Limited (Operator Overloading/Templates) |

# Rust Built-in Attributes: Detailed Reference

Built-in attributes are pre-defined by the Rust compiler (`rustc`). They are used for everything from conditional compilation to fine-tuning how the CPU handles your functions.

---

## 1. Core Built-in Attributes & C++ Equivalents

| Attribute | Syntax Example | Purpose | C++ Equivalent |
| :--- | :--- | :--- | :--- |
| **Conditional** | `#[cfg(target_os = "linux")]` | Only compiles the item if the condition is met. | `#ifdef __linux__` |
| **Testing** | `#[test]` | Marks a function as a test runner entry point. | GTest / Catch2 macros |
| **Optimization** | `#[inline(always)]` | Forces or suggests function inlining. | `__attribute__((always_inline))` |
| **FFI Safety** | `#[no_mangle]` | Disables name mangling for C compatibility. | `extern "C"` |
| **Diagnostics** | `#[must_use]` | Warns if the return value is ignored. | `[[nodiscard]]` |
| **Deprecation** | `#[deprecated]` | Issues a warning when the item is used. | `[[deprecated]]` |
| **Layout** | `#[repr(C)]` | Forces a specific memory layout (C-compatible). | `#pragma pack` / `alignas` |
| **Panic Behavior**| `#[panic_handler]` | Defines the function to call on a program panic. | Custom `std::terminate` handler |

---

## 2. Advanced Compiler Controls

These attributes give you "low-level" control over how Rust interacts with the hardware and the linker.



### Memory Layout (`#[repr(...)]`)
Rust's default memory layout is undefined (the compiler can reorder fields to save space).
* **`#[repr(C)]`**: Guarantees the same layout as a C struct. Essential for FFI.
* **`#[repr(transparent)]`**: Guarantees a struct with one field has the exact same layout as that single field.
* **`#[repr(packed)]`**: Removes all padding between fields (similar to C++ `#pragma pack(1)`).

### Linkage and Symbols
* **`#[link(name = "readline")]`**: Tells the compiler to link against a specific native library.
* **`#[export_name = "foo"]`**: Sets the exact symbol name in the resulting binary.
* **`#[used]`**: Prevents the linker from optimizing away a variable even if it seems unused.

---

## 3. Tool and Lint Attributes

Rust includes built-in attributes to manage the "human" side of the code, such as warnings and formatting.

| Attribute | Description | C++ Equivalent |
| :--- | :--- | :--- |
| `#[allow(lint)]` | Suppresses a specific compiler warning. | `#pragma GCC diagnostic ignore` |
| `#[warn(lint)]` | Turns a specific check into a warning. | `-Wall` / `-Wextra` |
| `#[deny(lint)]` | Turns a specific warning into a compile error. | `-Werror` |
| `#[rustfmt::skip]` | Tells the formatter not to touch the item. | `// clang-format off` |

---

## 4. The `cfg` System (Conditional Compilation)

Unlike C++ which uses a text-based preprocessor (`#ifdef`), Rust uses a logical gate system.

```rust
// Only compiled if the "web_server" feature is enabled in Cargo.toml
#[cfg(feature = "web_server")]
fn start_server() { ... }

// Only compiled on 64-bit Windows
#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
fn windows_specific_logic() { ... }
```