// Example: Borrowing Basics
// Run with: cargo run --example borrowing_basics

fn main() {
    println!("=== Borrowing Basics Demo ===\n");

    demo_immutable_borrows();
    demo_mutable_borrows();
    demo_ownership_vs_borrowing();
}

fn demo_immutable_borrows() {
    println!("1. Immutable Borrows (Many Readers):");

    let robot_name = String::from("R2-D2");

    // Multiple immutable borrows are allowed
    let ref1 = &robot_name;
    let ref2 = &robot_name;
    let ref3 = &robot_name;

    println!("  Robot names: {}, {}, {}", ref1, ref2, ref3);
    println!("  Original still valid: {}", robot_name);
    println!("  ✓ Many readers can coexist\n");
}

fn demo_mutable_borrows() {
    println!("2. Mutable Borrows (One Writer):");

    let mut sensor_data = vec![1.0, 2.0, 3.0];
    println!("  Initial data: {:?}", sensor_data);

    // Only ONE mutable borrow at a time
    let data_ref = &mut sensor_data;
    data_ref.push(4.0);
    data_ref.push(5.0);
    println!("  Modified data: {:?}", data_ref);

    // After data_ref goes out of scope, we can use sensor_data again
    println!("  Final data: {:?}", sensor_data);
    println!("  ✓ One writer at a time prevents data races\n");
}

fn demo_ownership_vs_borrowing() {
    println!("3. Ownership vs Borrowing:");

    let message = String::from("Hello, Rust!");

    // With borrowing, we can use the function multiple times
    print_message(&message);
    print_message(&message);
    print_message(&message);

    println!("  Original still valid: {}", message);
    println!("  ✓ Borrowing lets us use data without consuming it\n");
}

fn print_message(msg: &String) {
    println!("  Message: {}", msg);
}

/* C++ Comparison:

// C++: References and const
void print_message(const std::string& msg) {  // Similar to &String
    std::cout << msg << std::endl;
}

void modify_message(std::string& msg) {       // Similar to &mut String
    msg += "!";
}

std::string message = "Hello";
print_message(message);    // Const reference
modify_message(message);   // Mutable reference

Key Differences:
1. Rust enforces "one writer OR many readers" at compile time
2. C++ allows mixing const and non-const references (data races possible)
3. Rust's borrow checker prevents entire classes of bugs
*/
