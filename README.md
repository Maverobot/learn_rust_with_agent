# Learning Rust for C++ Robotics Engineers

Welcome to your personalized Rust learning journey! This repository is structured to leverage your C++ expertise while introducing Rust's unique features.

## 🎯 Learning Objectives

As a robotics software engineer, you'll appreciate Rust's:
- **Memory safety without garbage collection** (zero-cost abstractions)
- **Fearless concurrency** (no data races at compile time)
- **Performance comparable to C++**
- **Modern tooling** (Cargo, built-in testing, documentation)
- **Expressive type system** (algebraic data types, pattern matching)

## 📚 Module Structure

### [Module 1: Basics and Ownership](./modules/01_basics_ownership/README.md)
- Variables, mutability, and types
- **Ownership system** (Rust's killer feature vs C++ manual management)
- Move semantics (similar to C++11 move semantics)
- Stack vs Heap allocation
- **C++ comparisons included**

### [Module 2: Borrowing and Lifetimes](./modules/02_borrowing_lifetimes/README.md)
- References and borrowing rules
- Lifetime annotations
- How Rust prevents dangling pointers at compile time
- Comparison with C++ references and pointers

### [Module 3: Structs, Enums, and Pattern Matching](./modules/03_structs_enums/README.md)
- Structs (similar to C++ structs/classes)
- Enums (algebraic data types - more powerful than C++ enums)
- Pattern matching (like switch on steroids)
- Methods and associated functions

### [Module 4: Error Handling](./modules/04_error_handling/README.md)
- Result and Option types (no exceptions!)
- The `?` operator
- Recoverable vs unrecoverable errors
- Why Rust chose this approach

### [Module 5: Traits and Generics](./modules/05_traits_generics/README.md)
- Traits (similar to interfaces/type classes)
- Generic programming
- Trait bounds
- Comparison with C++ templates and concepts

### [Module 6: Collections and Iterators](./modules/06_collections_iterators/README.md)
- Vec, HashMap, HashSet
- Iterator patterns and adapters
- Zero-cost abstractions
- Functional programming style

### [Module 7: Concurrency and Smart Pointers](./modules/07_concurrency/README.md)
- Arc, Rc, Box, RefCell (vs C++ shared_ptr, unique_ptr)
- Threads and message passing
- Mutex and atomic types
- **Fearless concurrency** - data race prevention at compile time

## 🤖 Robotics-Specific Examples

The `examples/` directory contains practical robotics applications:
- Sensor data processing
- PID controllers
- Path planning algorithms
- Real-time systems considerations

## 🚀 Getting Started

### Prerequisites
✅ Rust installed (v1.93.0)
✅ Cargo workspace configured
✅ Ready to start learning!

### Running Examples
Each module contains runnable examples:
```bash
cd modules/01_basics_ownership
cargo run                              # Interactive exercises
cargo run --example ownership_basics   # Ownership demo
cargo run --example move_semantics     # Move semantics demo
```

Or from the workspace root:
```bash
cargo run -p basics_ownership
cargo test --workspace
```

### Running Tests
```bash
cargo test                  # Test current module
cargo test --workspace      # Test all modules
```

## 📖 How to Use This Repository

1. **Start with Module 1** - Even if concepts seem basic, ownership is crucial
2. **Read the documentation** in each module's README
3. **Run and modify the examples** - Learning by doing is key
4. **Complete the exercises** - Each module has practice problems
5. **Document your insights** - Add notes about C++ comparisons as you learn

## 🔗 Additional Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Official comprehensive guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by example
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises
- [Rust for C++ Programmers](https://github.com/nrc/r4cppp) - Direct comparisons

## 📝 Progress Tracking

Track your learning progress in [PROGRESS.md](./PROGRESS.md)

---

**Remember**: Rust's learning curve is steeper initially (especially ownership), but it pays off with safer, more maintainable code. Be patient with the borrow checker - it's teaching you to write better code!
