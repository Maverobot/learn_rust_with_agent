// Example: Borrowing Rules
// Run with: cargo run --example borrowing_rules

fn main() {
    println!("=== Borrowing Rules Demo ===\n");

    demo_rule_many_readers();
    demo_rule_one_writer();
    demo_rule_no_mixing();
    demo_scope_and_nll();
}

fn demo_rule_many_readers() {
    println!("Rule 1: Many Immutable Borrows (Readers) Are OK");

    let data = vec![1, 2, 3, 4, 5];

    let r1 = &data;
    let r2 = &data;
    let r3 = &data;
    let r4 = &data;

    println!("  r1: {:?}", r1);
    println!("  r2: {:?}", r2);
    println!("  r3: {:?}", r3);
    println!("  r4: {:?}", r4);
    println!("  ✓ All readers can access data simultaneously\n");
}

fn demo_rule_one_writer() {
    println!("Rule 2: Only ONE Mutable Borrow (Writer) at a Time");

    let mut position = Point { x: 0.0, y: 0.0 };

    {
        let pos_ref = &mut position;
        pos_ref.x = 10.0;
        pos_ref.y = 20.0;
        println!("  Modified position: ({}, {})", pos_ref.x, pos_ref.y);

        // Uncomment to see error:
        // let another_ref = &mut position;  // ERROR: can't have two mutable borrows!
    } // pos_ref goes out of scope here

    // Now we can borrow mutably again
    let pos_ref2 = &mut position;
    pos_ref2.x += 5.0;
    println!("  Further modified: ({}, {})", pos_ref2.x, pos_ref2.y);
    println!("  ✓ One writer at a time prevents data races\n");
}

fn demo_rule_no_mixing() {
    println!("Rule 3: Can't Mix Immutable and Mutable Borrows");

    let mut values = vec![10, 20, 30];

    let read_ref = &values;
    println!("  Reading: {:?}", read_ref);

    // Uncomment to see error:
    // let write_ref = &mut values;  // ERROR: can't borrow as mutable while immutable borrow exists
    // write_ref.push(40);

    // After read_ref is done, we can borrow mutably
    let write_ref = &mut values;
    write_ref.push(40);
    println!("  After writing: {:?}", write_ref);
    println!("  ✓ No simultaneous readers and writers\n");
}

fn demo_scope_and_nll() {
    println!("Rule 4: Borrow Scope Ends at Last Use (NLL)");

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("  Immutable borrows: {} and {}", r1, r2);
    // r1 and r2 scope ends here (last use)

    // This works because r1 and r2 are no longer used
    let r3 = &mut s;
    r3.push_str(", world");
    println!("  Mutable borrow: {}", r3);

    println!("  ✓ Non-Lexical Lifetimes (NLL) make borrowing more flexible\n");
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

/* C++ Comparison: Why Rust's Rules Matter

// C++ allows data races:
std::vector<int> v = {1, 2, 3};
const int& first = v[0];      // Immutable reference
v.push_back(4);                // Mutable operation - may reallocate!
std::cout << first;            // UNDEFINED BEHAVIOR - dangling reference

// C++ allows simultaneous readers and writers:
int x = 42;
const int& reader = x;
int& writer = x;
writer = 100;                  // Writer modifies
std::cout << reader;           // Reader sees modified value (or worse!)

// Rust prevents both at compile time!
let mut v = vec![1, 2, 3];
let first = &v[0];
// v.push(4);                  // ERROR: can't modify while borrowed
println!("{}", first);

Key Insight:
- C++ trusts you to avoid data races
- Rust enforces safety at compile time
- Zero runtime cost - all checks done by compiler!
*/
