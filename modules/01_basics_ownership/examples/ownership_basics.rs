// Example: Ownership Basics
// Run with: cargo run --example ownership_basics

fn main() {
    println!("=== Ownership Basics Demo ===\n");

    // Demo 1: Simple ownership
    demo_simple_ownership();

    // Demo 2: Stack vs Heap
    demo_stack_vs_heap();

    // Demo 3: Drop trait (RAII)
    demo_drop_trait();
}

fn demo_simple_ownership() {
    println!("1. Simple Ownership:");

    {
        let s = String::from("hello"); // s comes into scope
        println!("  String created: {}", s);
    } // s goes out of scope and is dropped (memory freed)

    println!("  String dropped (out of scope)\n");
}

fn demo_stack_vs_heap() {
    println!("2. Stack vs Heap:");

    // Stack: fast, fixed size
    let x = 42; // i32 stored on stack
    let y = x; // Copy (cheap for small types)
    println!("  Stack - x: {}, y: {} (both valid after copy)", x, y);

    // Heap: dynamic size, ownership tracking
    let s1 = String::from("hello"); // Data stored on heap
    let s2 = s1; // Ownership moved (pointer, not data copied)
    // s1 is now invalid - prevents double free
    println!("  Heap - s2: {} (s1 moved, now invalid)\n", s2);
}

fn demo_drop_trait() {
    println!("3. Drop Trait (like C++ destructor):");

    struct Sensor {
        id: u32,
        name: String,
    }

    impl Drop for Sensor {
        fn drop(&mut self) {
            println!(
                "  Sensor '{}' (ID: {}) is being destroyed",
                self.name, self.id
            );
        }
    }

    {
        let imu = Sensor {
            id: 1,
            name: String::from("IMU"),
        };
        let lidar = Sensor {
            id: 2,
            name: String::from("LiDAR"),
        };
        println!("  Sensors created: {} and {}", imu.name, lidar.name);
    } // Drop called automatically in reverse order of creation

    println!("  (Sensors destroyed when leaving scope)\n");
}

// C++ Comparison (as comments):
/*
C++ Equivalent:

void demo_stack_vs_heap() {
    // Stack
    int x = 42;
    int y = x;  // Copy

    // Heap - manual management
    std::string* s1 = new std::string("hello");
    std::string* s2 = s1;  // Shallow copy - dangerous!
    delete s2;  // s1 is now dangling pointer

    // Heap - RAII with unique_ptr
    auto s3 = std::make_unique<std::string>("hello");
    auto s4 = std::move(s3);  // Explicit move
    // s3 is now in valid but unspecified state (Rust is stricter)
}

class Sensor {
public:
    ~Sensor() {
        std::cout << "Sensor destroyed\n";
    }
};
*/
