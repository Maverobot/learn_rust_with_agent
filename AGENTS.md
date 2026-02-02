# Agent Guidelines for Rust Learning Repository

This repository is a structured Rust learning curriculum designed for C++ engineers transitioning to Rust. Follow these guidelines when contributing code or creating new modules.

## 🏗️ Repository Structure

```
learn_rust_with_agent/
├── Cargo.toml                         # Workspace root
├── modules/                           # Learning modules (numbered)
│   ├── 01_basics_ownership/          # Each module is a cargo package
│   │   ├── Cargo.toml
  │   │   ├── README.md                 # Module documentation
  │   │   ├── src/main.rs               # Interactive exercises
  │   │   └── examples/*.rs             # Runnable examples
  │   └── 02_borrowing_lifetimes/       # Future modules...
  └── examples/                          # Cross-module examples (future)
  ```

  ## 🔧 Build, Lint, and Test Commands

  ### Building
  ```bash
  # Build entire workspace
  cargo build

  # Build specific module
  cargo build -p basics_ownership

  # Check without building (faster)
  cargo check
  cargo check --workspace
  ```

  ### Running Code
  ```bash
  # Run a module's main.rs
  cargo run -p basics_ownership

  # Run a specific example
  cargo run -p basics_ownership --example ownership_basics

  # From inside a module directory
  cd modules/01_basics_ownership
  cargo run
  cargo run --example ownership_basics
  ```

  ### Testing
  ```bash
  # Run all tests in workspace
  cargo test --workspace

  # Run tests in specific module
  cargo test -p basics_ownership

  # Run a single test by name (exact match)
  cargo test test_copy_types

  # Run tests matching a pattern
  cargo test ownership

  # Show test output (stdout/stderr)
  cargo test -- --nocapture

  # Run single test with output
  cargo test test_copy_types -- --nocapture --exact
  ```

  ### Linting and Formatting
  ```bash
  # Format all code (REQUIRED before commits)
  cargo fmt --all

  # Check formatting without changing files
  cargo fmt --all -- --check

  # Run Clippy linter (catch common mistakes)
  cargo clippy --all-targets --workspace

  # Clippy with all warnings
  cargo clippy --all-targets --workspace -- -D warnings
  ```

  ### Documentation
  ```bash
  # Generate and open documentation
  cargo doc --open

  # Check documentation examples
  cargo test --doc
  ```

  ## 📝 Code Style Guidelines

  ### File Headers
  Every `.rs` file should have a descriptive header:
  ```rust
  // Module 1: Basics and Ownership
  // Interactive exercises - uncomment sections and fix the code!
  ```

  For examples:
  ```rust
  // Example: Ownership Basics
  // Run with: cargo run --example ownership_basics
  ```

  ### Imports
  ```rust
  // Standard library first
  use std::collections::HashMap;
  use std::fmt;

  // External crates next (when added)
  // use serde::{Serialize, Deserialize};

  // Local/super imports last
  use super::*;
  ```

  ### Formatting
  - **Indentation**: 4 spaces (enforced by `cargo fmt`)
  - **Line length**: 100 characters (rustfmt default)
  - **Trailing commas**: Use them in multi-line expressions
  - **Let rustfmt do the work**: Always run `cargo fmt`

  ### Naming Conventions
  ```rust
  // Functions and variables: snake_case
  fn calculate_velocity() {}
  let sensor_data = vec![1, 2, 3];

  // Types (structs, enums, traits): PascalCase
  struct RobotController {}
  enum SensorType {}
  trait Movable {}

  // Constants: SCREAMING_SNAKE_CASE
  const MAX_SPEED: f64 = 10.0;
  const DEFAULT_TIMEOUT: u64 = 5000;

  // Lifetimes: short, descriptive lowercase
  fn process<'a, 'b>(data: &'a str, buffer: &'b mut String) {}
  ```

  ### Comments
  ```rust
  // Use // for single-line comments
  // Explain WHY, not WHAT (code should be self-documenting)

  /// Use /// for documentation comments
  /// 
  /// # Arguments
  /// * `data` - The sensor data to process
  /// 
  /// # Returns
  /// Processed data as a Vec<f64>
  fn process_data(data: &[f64]) -> Vec<f64> {
      // Implementation
  }

  // C++ Comparison comments for learning purposes:
  // In C++, after std::move(), the moved-from object is still "valid"
  // but in an unspecified state. Rust is stricter - you simply cannot
  // use s1 anymore after it's been moved.
  ```

  ### Error Handling
  ```rust
  // In early modules: Use simple unwrap() with clear comments
  let data = read_file("data.txt").unwrap();  // OK for examples

  // In later modules: Proper error handling
  let data = read_file("data.txt")?;  // Propagate errors
  let data = read_file("data.txt").expect("Failed to read data file");

  // Pattern: Result<T, E> for recoverable errors
  fn parse_sensor_data(input: &str) -> Result<SensorData, ParseError> {
      // ...
  }

  // Pattern: Option<T> for optional values (no null pointers!)
  fn find_robot(id: u32) -> Option<Robot> {
      // ...
  }
  ```

  ### Type Annotations
  ```rust
  // Explicit types when it improves clarity
  let count: usize = data.len();
  let velocity: f64 = 5.0;

  // Let inference work when obvious
  let s = String::from("hello");  // Type is obvious
  let numbers = vec![1, 2, 3];    // Type is obvious

  // Always annotate function signatures
  fn calculate_distance(p1: &Point, p2: &Point) -> f64 {
      // ...
  }
  ```

  ### Testing
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_descriptive_name() {
          // Arrange
          let input = 42;

          // Act
          let result = process(input);

          // Assert
          assert_eq!(result, expected);
      }

      #[test]
      #[should_panic(expected = "overflow")]
      fn test_error_condition() {
          // Test panic conditions
      }
  }
  ```

  ## 🎓 Educational Guidelines

  ### Module Structure
  Each module should include:
  1. **README.md** - Concepts, C++ comparisons, learning objectives
  2. **src/main.rs** - Interactive exercises with commented-out errors
  3. **examples/*.rs** - Working demos showing concepts in action
  4. **Tests** - Verify exercises work correctly

  ### C++ Comparisons
  Always provide C++ context for concepts:
  ```rust
  // C++ equivalent comments
  /*
  C++ Equivalent:

  std::unique_ptr<std::string> ptr = std::make_unique<std::string>("hello");
  auto ptr2 = std::move(ptr);  // Explicit move
  // ptr is now in valid but unspecified state
  */
  ```

  ### Intentional Errors
  Use commented-out code to demonstrate compiler errors:
  ```rust
  // This will cause a compilation error - uncomment to see:
  // println!("s1 = {}", s1);  // Error! s1 no longer valid
  ```

  ### Progressive Complexity
  - Start simple, build gradually
  - Reference future modules for advanced topics
  - Reinforce previous concepts

  ## 🚫 Common Mistakes to Avoid

  1. **Don't use `clone()` unnecessarily** - Prefer borrowing (teach this!)
  2. **Don't ignore Clippy warnings** - They teach best practices
  3. **Don't use `unwrap()` in production code** - Only in examples with comments
  4. **Don't fight the borrow checker** - Understand ownership first
  5. **Don't add dependencies without reason** - Keep examples self-contained

  ## ✅ Checklist for New Modules

  - [ ] Add module to workspace `Cargo.toml` members list
  - [ ] Create `README.md` with learning objectives and C++ comparisons
  - [ ] Write `src/main.rs` with interactive exercises
  - [ ] Add at least 2 runnable examples in `examples/`
  - [ ] Include tests in `src/main.rs`
  - [ ] Run `cargo fmt --all`
  - [ ] Run `cargo clippy --workspace`
  - [ ] Run `cargo test --workspace`
  - [ ] Verify all examples run: `cargo run --example <name>`
  - [ ] Update main `README.md` with module link

  ## 🎯 Learning Philosophy

  This repository teaches through:
  - **Hands-on practice** - Runnable, modifiable code
  - **Comparative learning** - Leverage C++ knowledge
  - **Failing forward** - Learn from compiler errors
  - **Progressive disclosure** - Simple to complex
  - **Reinforcement** - Document learning in PROGRESS.md

  ---

  **Remember**: Code should be educational, clear, and idiomatic. When in doubt, prioritize teaching value over brevity.
