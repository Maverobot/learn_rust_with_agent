# Module 1: Basics and Ownership

## 🎯 Learning Objectives

By the end of this module, you will understand:
- Rust's ownership system and why it's revolutionary
- Move semantics and how they differ from C++
- Stack vs heap allocation in Rust
- When values are copied vs moved
- The Drop trait (Rust's RAII equivalent)

## 📖 Core Concepts

### 1. Ownership Rules (The Foundation)

Rust's ownership system has three fundamental rules:

1. **Each value has a single owner**
2. **There can only be one owner at a time**
3. **When the owner goes out of scope, the value is dropped**

These rules are enforced at **compile time** with zero runtime cost!

### 2. C++ vs Rust: Memory Management

#### C++ Approach:
```cpp
// Manual memory management
int* ptr = new int(42);
// ... easy to forget delete, double free, use after free
delete ptr;

// Or RAII with smart pointers
std::unique_ptr<int> smart_ptr = std::make_unique<int>(42);
// Automatically cleaned up
```

#### Rust Approach:
```rust
// Ownership is automatic and enforced by the compiler
let x = Box::new(42);  // Heap allocated
// Automatically cleaned up when x goes out of scope
// Compiler prevents use-after-free, double-free, etc.
```

### 3. Move Semantics

#### In C++ (C++11+):
```cpp
std::vector<int> v1 = {1, 2, 3};
std::vector<int> v2 = std::move(v1);  // Explicit move
// v1 is in valid but unspecified state
```

#### In Rust:
```rust
let v1 = vec![1, 2, 3];
let v2 = v1;  // Move happens automatically!
// Compiler error if you try to use v1 now
```

**Key Difference**: Rust moves by default for heap-allocated types, preventing accidental copies and use-after-move at compile time.

### 4. Copy Types

Some types are trivial to copy (like integers):

```rust
let x = 5;
let y = x;  // Copy, not move (integers are Copy)
println!("{}, {}", x, y);  // Both still valid
```

Types that implement the `Copy` trait:
- All integer types (`i32`, `u64`, etc.)
- Boolean (`bool`)
- Floating point types (`f32`, `f64`)
- Character (`char`)
- Tuples containing only Copy types

### 5. Stack vs Heap

```rust
// Stack: fast, fixed size, automatically managed
let x = 42;                    // i32 on stack
let y = (1, 2, 3);            // tuple on stack

// Heap: flexible size, manual allocation (but automatic cleanup)
let s = String::from("hello"); // String data on heap
let v = vec![1, 2, 3];        // Vec data on heap
```

## 🔬 Examples

See the `examples/` directory for runnable code:
- `ownership_basics.rs` - Basic ownership demonstration
- `move_semantics.rs` - Move vs copy comparison
- `cpp_comparison.rs` - Side-by-side C++/Rust examples (as comments)

Run examples with:
```bash
cargo run --example ownership_basics
cargo run --example move_semantics
```

## 💪 Exercises

Work through the exercises in `src/main.rs` by uncommenting sections and fixing compilation errors.

## 🤔 Common Questions from C++ Developers

### Q: Why can't I use a value after moving it?
**A**: This prevents use-after-move bugs. In C++, moved-from objects are in a valid but unspecified state. Rust is more strict - once moved, the old binding is invalid.

### Q: Isn't copying better than moving?
**A**: For large objects (like vectors, strings), moving is cheaper. Rust's default move behavior prevents expensive accidental copies.

### Q: How is this different from std::unique_ptr?
**A**: Similar concept, but Rust enforces ownership for ALL types, not just pointers. The ownership rules apply to stack-allocated data too.

### Q: What about shared ownership?
**A**: That's coming in Module 7 with `Rc`, `Arc` (like `shared_ptr`), but single ownership is the default and preferred pattern.

## 🔑 Key Takeaways

1. ✅ Ownership eliminates entire classes of bugs at compile time
2. ✅ No garbage collector needed - deterministic cleanup like C++ RAII
3. ✅ Move by default for heap types - prevents expensive copies
4. ✅ Compiler enforces memory safety - no dangling pointers
5. ✅ Zero runtime cost - all checks at compile time

## 📚 Further Reading

- [The Rust Book - Chapter 4: Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust for C++ Programmers - Ownership](https://github.com/nrc/r4cppp/blob/master/graphs/README.md)

---

**Next**: [Module 2: Borrowing and Lifetimes](../02_borrowing_lifetimes/README.md)
