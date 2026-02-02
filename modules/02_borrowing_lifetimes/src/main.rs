// Module 2: Borrowing and Lifetimes
// Interactive exercises - uncomment sections and fix the code!

fn main() {
    println!("=== Module 2: Borrowing and Lifetimes ===\n");

    // Exercise 1: Immutable Borrowing
    exercise_1_immutable_borrowing();

    // Exercise 2: Mutable Borrowing
    exercise_2_mutable_borrowing();

    // Exercise 3: Borrowing Rules
    exercise_3_borrowing_rules();

    // Exercise 4: Reference Scope
    exercise_4_reference_scope();

    // Exercise 5: Lifetimes
    exercise_5_lifetimes();

    println!("\n✅ All exercises completed!");
}

// Exercise 1: Immutable Borrowing
// Task: Understand how to borrow without taking ownership
fn exercise_1_immutable_borrowing() {
    println!("Exercise 1: Immutable Borrowing");

    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Borrow s1

    println!("The length of '{}' is {}", s1, len); // s1 still valid!

    // Multiple immutable borrows are allowed
    let r1 = &s1;
    let r2 = &s1;
    let r3 = &s1;
    println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);

    println!("✓ Immutable borrowing demonstrated\n");
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but doesn't Drop because we don't own it

// Exercise 2: Mutable Borrowing
// Task: Learn to modify borrowed data
fn exercise_2_mutable_borrowing() {
    println!("Exercise 2: Mutable Borrowing");

    let mut s = String::from("hello");
    println!("Before: {}", s);

    change(&mut s); // Mutable borrow
    println!("After: {}", s);

    // C++ Comparison:
    // In C++: void change(std::string& s)
    // In Rust: fn change(s: &mut String)
    // Rust is more explicit about mutability!

    println!("✓ Mutable borrowing demonstrated\n");
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// Exercise 3: Borrowing Rules
// Task: Understand the "one writer OR many readers" rule
fn exercise_3_borrowing_rules() {
    println!("Exercise 3: Borrowing Rules");

    let mut s = String::from("hello");

    // Rule 1: Multiple immutable borrows are OK
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    // r1 and r2 go out of scope here (last use)

    // Rule 2: Only ONE mutable borrow at a time
    let r3 = &mut s;
    r3.push('!');
    println!("r3: {}", r3);
    // r3 goes out of scope here

    // Rule 3: Can't mix mutable and immutable borrows
    let r4 = &s;
    // let r5 = &mut s;  // ERROR: can't borrow as mutable while immutable borrow exists
    println!("r4: {}", r4);

    println!("✓ Borrowing rules demonstrated\n");
}

// Exercise 4: Reference Scope (Non-Lexical Lifetimes)
// Task: Understand when borrows end
fn exercise_4_reference_scope() {
    println!("Exercise 4: Reference Scope");

    let mut s = String::from("hello");

    let r1 = &s; // Immutable borrow starts
    let r2 = &s; // Another immutable borrow
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // OK! Immutable borrows ended
    r3.push_str(", world");
    println!("{}", r3);

    // This is Non-Lexical Lifetimes (NLL) - Rust 2018 feature
    // References end at their last use, not at the end of the scope

    println!("✓ Reference scope demonstrated\n");
}

// Exercise 5: Lifetimes
// Task: Understand basic lifetime annotations
fn exercise_5_lifetimes() {
    println!("Exercise 5: Lifetimes");

    let string1 = String::from("long string");
    let string2 = String::from("short");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is: {}", result);

    // Uncomment to see lifetime error:
    // let result;
    // {
    //     let string2 = String::from("short");
    //     result = longest(string1.as_str(), string2.as_str());
    // }  // string2 dropped here
    // println!("The longest string is: {}", result);  // ERROR: string2 doesn't live long enough

    println!("✓ Lifetimes demonstrated\n");
}

// Lifetime annotation: 'a means "all these references have the same lifetime"
// The return value lives as long as the shortest of x or y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Bonus: Demonstrating what Rust prevents
// Uncomment to see dangling reference prevention:
//
// fn dangling() -> &String {
//     let s = String::from("hello");
//     &s  // ERROR: s will be dropped, returning a dangling reference!
// }
//
// fn no_dangling() -> String {
//     let s = String::from("hello");
//     s  // OK: return ownership
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        assert_eq!(calculate_length(&s), 5);
        assert_eq!(s, "hello"); // s still valid after borrowing
    }

    #[test]
    fn test_change() {
        let mut s = String::from("hello");
        change(&mut s);
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_longest() {
        let a = "long string";
        let b = "short";
        assert_eq!(longest(a, b), "long string");
        assert_eq!(longest(b, a), "long string");
    }

    #[test]
    fn test_longest_equal_length() {
        let a = "same";
        let b = "same";
        // Should return y when equal
        assert_eq!(longest(a, b), "same");
    }
}
