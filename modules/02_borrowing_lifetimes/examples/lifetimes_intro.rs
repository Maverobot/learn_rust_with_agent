// Example: Lifetimes Introduction
// Run with: cargo run --example lifetimes_intro

fn main() {
    println!("=== Lifetimes Introduction ===\n");

    demo_implicit_lifetimes();
    demo_explicit_lifetimes();
    demo_lifetime_in_structs();
}

fn demo_implicit_lifetimes() {
    println!("1. Implicit Lifetimes (Rust Infers Them):");

    let s = String::from("Hello, Rust!");
    let first_word = get_first_word(&s);

    println!("  Original: {}", s);
    println!("  First word: {}", first_word);
    println!("  ✓ Compiler knows the returned reference lives as long as input\n");
}

// Implicit lifetime: compiler infers 'a
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }

    s
}

fn demo_explicit_lifetimes() {
    println!("2. Explicit Lifetimes (When Compiler Needs Help):");

    let string1 = String::from("long string is long");
    let result;

    {
        let string2 = String::from("short");
        result = longest(string1.as_str(), string2.as_str());
        println!("  The longest string is: {}", result);
    } // string2 dropped here, but result is still valid!

    // This works because result holds a reference to string1, not string2
    // The lifetime annotation tells Rust this is OK

    println!("  ✓ Lifetime annotation ensures returned reference is valid\n");
}

// Explicit lifetime: 'a means "all these references have the same lifetime"
// Return value lives as long as the SHORTEST of x or y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn demo_lifetime_in_structs() {
    println!("3. Lifetimes in Structs:");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("  Excerpt: {}", excerpt.part);
    println!("  Level: {}", excerpt.level());

    let returned = excerpt.announce_and_return_part("Breaking news");
    println!("  Returned part: {}", returned);

    println!("  ✓ Struct holds a reference with specified lifetime\n");
}

// Struct with lifetime annotation
// 'a means: "this struct cannot outlive the reference it holds"
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Implementing methods with lifetimes
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // Lifetime elision: compiler infers the lifetime
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

/* C++ Comparison: Lifetimes

// C++ has no lifetime tracking!
const char* dangerous() {
    std::string s = "temporary";
    return s.c_str();  // UNDEFINED BEHAVIOR - returns dangling pointer!
}

// Rust prevents this:
fn safe() -> &str {
    let s = String::from("temporary");
    &s  // ERROR: s doesn't live long enough!
}

// C++ with references:
class Excerpt {
    const std::string& text;  // Reference member
public:
    Excerpt(const std::string& t) : text(t) {}
};

std::string make_excerpt() {
    std::string s = "temporary";
    return Excerpt(s);  // UNDEFINED BEHAVIOR - dangling reference!
}

// Rust equivalent is safe:
struct Excerpt<'a> {
    text: &'a str,
}

fn make_excerpt() -> Excerpt<'static> {
    Excerpt { text: "temporary" }  // OK - string literal has 'static lifetime
}

Key Insight:
- C++ lets you create dangling references - runtime errors or worse!
- Rust's lifetime system prevents this at compile time
- 'static means "lives for entire program duration"
- Most lifetimes are inferred, you rarely write them explicitly
*/
