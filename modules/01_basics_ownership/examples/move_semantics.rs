// Example: Move Semantics
// Run with: cargo run --example move_semantics

fn main() {
    println!("=== Move Semantics Demo ===\n");

    demo_move_vs_copy();
    demo_function_ownership();
    demo_clone();
}

fn demo_move_vs_copy() {
    println!("1. Move vs Copy:");

    // Types that implement Copy (stack-only data)
    let x = 5;
    let y = x; // Copy
    println!("  Integers (Copy): x={}, y={} - both valid", x, y);

    let tuple = (1, 2);
    let tuple2 = tuple; // Copy (contains only Copy types)
    println!("  Tuple (Copy): {:?}, {:?} - both valid", tuple, tuple2);

    // Types that don't implement Copy (heap data)
    let s1 = String::from("robot");
    let s2 = s1; // Move (not Copy)
    // println!("{}", s1);  // ERROR: s1 is moved
    println!("  String (Move): s2={} - s1 is moved", s2);

    let v1 = vec![1, 2, 3];
    let v2 = v1; // Move
    // println!("{:?}", v1);  // ERROR: v1 is moved
    println!("  Vec (Move): v2={:?} - v1 is moved\n", v2);
}

fn demo_function_ownership() {
    println!("2. Function Ownership:");

    let s = String::from("hello");
    println!("  Before call: s={}", s);

    takes_ownership(s);
    // s is moved into the function
    // println!("{}", s);  // ERROR: s is no longer valid

    let x = 42;
    println!("  Before call: x={}", x);
    makes_copy(x);
    println!("  After call: x={} - still valid (Copy type)\n", x);
}

fn takes_ownership(some_string: String) {
    println!("  Inside function: {}", some_string);
    // some_string is dropped here when function ends
}

fn makes_copy(some_integer: i32) {
    println!("  Inside function: {}", some_integer);
    // some_integer is copied, original still valid
}

fn demo_clone() {
    println!("3. Explicit Clone (Deep Copy):");

    let s1 = String::from("rust");
    let s2 = s1.clone(); // Explicit deep copy

    println!("  s1={}, s2={} - both valid after clone", s1, s2);
    println!("  (clone() creates a deep copy of heap data)\n");

    // In robotics: cloning large sensor data can be expensive!
    let sensor_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let backup = sensor_data.clone(); // Expensive for large data
    println!("  Original: {:?}", sensor_data);
    println!("  Backup: {:?}", backup);
    println!("  (Consider borrowing instead of cloning for large data)\n");
}

// Return ownership pattern (common in Rust)
#[allow(dead_code)]
fn gives_ownership() -> String {
    String::from("ownership") // Ownership moved to caller
}

#[allow(dead_code)]
fn takes_and_gives_back(s: String) -> String {
    s // Ownership moved in, then moved out
}

/* C++ Comparison:

// C++11 Move Semantics
std::string s1 = "hello";
std::string s2 = std::move(s1);  // Explicit move
// s1 is now in valid but unspecified state (can still be used carefully)

// Rust Move Semantics
let s1 = String::from("hello");
let s2 = s1;  // Implicit move for non-Copy types
// s1 is INVALID - compiler error if you try to use it

Key Difference:
- C++: Move is explicit, moved-from object still exists
- Rust: Move is implicit for heap types, moved variable is invalid
- Rust prevents use-after-move at compile time!
*/
