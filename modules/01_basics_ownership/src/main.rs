// Module 1: Basics and Ownership
// Interactive exercises - uncomment sections and fix the code!

fn main() {
    println!("=== Module 1: Ownership Basics ===\n");

    // Exercise 1: Understanding Move Semantics
    exercise_1_move_semantics();

    // Exercise 2: Copy vs Move
    exercise_2_copy_vs_move();

    // Exercise 3: Ownership Transfer
    exercise_3_ownership_transfer();

    println!("\n✅ All exercises completed!");
}

// Exercise 1: Move Semantics
// Task: Fix the compilation error by understanding ownership
fn exercise_1_move_semantics() {
    println!("Exercise 1: Move Semantics");

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2

    // This will cause a compilation error - uncomment to see:
    // println!("s1 = {}", s1);  // Error! s1 no longer valid

    println!("s2 = {}", s2); // This works fine

    // C++ Comparison:
    // In C++, after std::move(), the moved-from object is still "valid"
    // but in an unspecified state. Rust is stricter - you simply cannot
    // use s1 anymore after it's been moved.

    println!("✓ Move semantics demonstrated\n");
}

// Exercise 2: Copy vs Move
// Task: Understand which types are Copy and which are Move
fn exercise_2_copy_vs_move() {
    println!("Exercise 2: Copy vs Move");

    // Integers are Copy (like C++ primitives)
    let x = 5;
    let y = x; // x is copied, not moved
    println!("x = {}, y = {} (both valid - Copy type)", x, y);

    // Strings are Move (like C++ unique_ptr)
    let s1 = String::from("world");
    let s2 = s1; // s1 is moved, not copied
    // println!("s1 = {}", s1);  // Uncomment to see error
    println!("s2 = {} (s1 is no longer valid - Move type)", s2);

    // To make a copy of String, use .clone() (explicit like in C++)
    let s3 = String::from("rust");
    let s4 = s3.clone(); // Deep copy
    println!("s3 = {}, s4 = {} (both valid after clone)", s3, s4);

    println!("✓ Copy vs Move demonstrated\n");
}

// Exercise 3: Ownership Transfer
// Task: Pass ownership to functions and get it back
fn exercise_3_ownership_transfer() {
    println!("Exercise 3: Ownership Transfer");

    let s = String::from("robotics");
    println!("Original: {}", s);

    let s = takes_ownership(s); // s moved into function, returned back
    println!("After return: {}", s);

    // Alternative: pass by reference (covered in Module 2)
    // For now, we're returning ownership back

    println!("✓ Ownership transfer demonstrated\n");
}

fn takes_ownership(s: String) -> String {
    println!("Inside function: {}", s);
    // In C++, you might see: std::string func(std::string s)
    // But in Rust, ownership semantics are more explicit
    s // Return ownership back to caller
}

// Bonus: Understanding Drop (like C++ destructor)
#[allow(dead_code)]
struct Robot {
    name: String,
}

impl Drop for Robot {
    fn drop(&mut self) {
        println!("Robot {} is being dropped (destroyed)", self.name);
    }
}

// Uncomment to see Drop in action:
// fn demo_drop() {
//     let r = Robot {
//         name: String::from("R2-D2"),
//     };
//     println!("Robot created");
//     // Drop is called automatically when r goes out of scope
//     // Similar to C++ RAII and destructors
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_types() {
        let x = 5;
        let y = x;
        assert_eq!(x, 5);
        assert_eq!(y, 5);
    }

    #[test]
    fn test_ownership_transfer() {
        let s = String::from("test");
        let s = takes_ownership(s);
        assert_eq!(s, "test");
    }
}
