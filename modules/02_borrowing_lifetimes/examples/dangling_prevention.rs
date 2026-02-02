// Example: Dangling Reference Prevention
// Run with: cargo run --example dangling_prevention

fn main() {
    println!("=== Dangling Reference Prevention ===\n");

    demo_dangling_in_cpp();
    demo_safe_in_rust();
    demo_vector_reallocation();
}

fn demo_dangling_in_cpp() {
    println!("1. C++ Problem: Dangling References");
    println!("   In C++, this compiles but has undefined behavior:");
    println!("   ```cpp");
    println!("   const char* dangling() {{");
    println!("       std::string s = \"hello\";");
    println!("       return s.c_str();  // Returns pointer to destroyed string!");
    println!("   }}");
    println!("   ```");
    println!("   Result: Crashes, corruption, or \"works\" by accident\n");
}

fn demo_safe_in_rust() {
    println!("2. Rust Solution: Compile-Time Prevention");

    // This function returns ownership - safe!
    let s = no_dangling();
    println!("   Safe return: {}", s);

    // Uncomment to see the error Rust prevents:
    // fn dangling() -> &String {
    //     let s = String::from("hello");
    //     &s  // ERROR: s dropped here, can't return reference to it!
    // }

    println!("   ✓ Rust compiler prevents dangling references\n");
}

fn no_dangling() -> String {
    let s = String::from("hello");
    s // Return ownership - safe!
}

fn demo_vector_reallocation() {
    println!("3. Vector Reallocation Problem (C++ pitfall):");

    println!("   C++ problem:");
    println!("   ```cpp");
    println!("   std::vector<int> v = {{1, 2, 3}};");
    println!("   int& first = v[0];");
    println!("   v.push_back(4);        // May reallocate!");
    println!("   std::cout << first;     // UNDEFINED BEHAVIOR");
    println!("   ```\n");

    println!("   Rust prevention:");
    safe_vector_example();
    println!("   ✓ Borrow checker prevents use-after-reallocation\n");
}

fn safe_vector_example() {
    let mut v = vec![1, 2, 3];

    {
        let first = &v[0];
        println!("   First element: {}", first);
        // Uncommenting the next line would cause a compile error:
        // v.push(4);  // ERROR: can't modify while borrowed!
    } // first goes out of scope here

    // Now we can modify the vector
    v.push(4);
    println!("   After push: {:?}", v);
}

/* More Dangling Reference Examples

1. Iterator Invalidation (C++ Problem):
   ```cpp
   std::vector<int> v = {1, 2, 3};
   auto it = v.begin();
   v.push_back(4);        // Iterator invalidated!
   std::cout << *it;      // UNDEFINED BEHAVIOR
   ```

2. Reference to Local Variable (C++ Problem):
   ```cpp
   int& bad_ref() {
       int x = 42;
       return x;           // Returns reference to local variable!
   }
   ```

3. Reference to Temporary (C++ Problem):
   ```cpp
   const std::string& bad = std::string("temp");  // Dangling!
   std::cout << bad;      // UNDEFINED BEHAVIOR
   ```

Rust prevents ALL of these at compile time:

1. Iterator Invalidation:
   ```rust
   let mut v = vec![1, 2, 3];
   let first = &v[0];
   // v.push(4);         // ERROR: can't modify while borrowed
   println!("{}", first);
   ```

2. Reference to Local:
   ```rust
   fn bad_ref() -> &i32 {
       let x = 42;
       &x                 // ERROR: x dropped here
   }
   ```

3. Reference to Temporary:
   ```rust
   let s = String::from("temp");
   let reference = &s;    // OK - s lives as long as needed
   ```

Key Takeaway:
- C++: Trust the programmer, runtime errors
- Rust: Verify at compile time, prevent errors
- No runtime cost - all checks done by compiler
- Makes concurrent programming much safer
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_dangling() {
        let s = no_dangling();
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_safe_vector() {
        // This test demonstrates that the borrow checker works
        let mut v = vec![1, 2, 3];
        {
            let _first = &v[0];
            // Can't modify here
        }
        v.push(4); // OK now
        assert_eq!(v, vec![1, 2, 3, 4]);
    }
}
