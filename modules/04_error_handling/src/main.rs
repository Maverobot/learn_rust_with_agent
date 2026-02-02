// Module 4: Error Handling
// Interactive exercises exploring Result, ?, custom errors, and error patterns

fn main() {
    println!("=== Module 4: Error Handling ===\n");

    exercise_1_result_basics();
    exercise_2_question_mark_operator();
    exercise_3_custom_errors();
    exercise_4_error_conversion();
    exercise_5_result_methods();
    exercise_6_when_to_panic();

    println!("\n✅ All exercises completed!");
}

// ============================================================================
// Exercise 1: Result Basics
// ============================================================================

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

fn exercise_1_result_basics() {
    println!("--- Exercise 1: Result Basics ---");

    // Handle Result with match
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Handle with if let
    if let Ok(result) = divide(20, 4) {
        println!("20 / 4 = {}", result);
    }

    // Check is_ok() and is_err()
    let result = divide(15, 3);
    if result.is_ok() {
        println!("Division succeeded!");
    }

    let result = divide(10, 0);
    if result.is_err() {
        println!("Division failed!");
    }

    // Parse numbers
    match parse_number("42") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Parse error: {}", e),
    }

    match parse_number("not a number") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Parse error: {}", e),
    }

    println!();
}

// ============================================================================
// Exercise 2: The ? Operator
// ============================================================================

fn calculate_sum(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let num_a = a.parse::<i32>()?; // Early return if parse fails
    let num_b = b.parse::<i32>()?; // Early return if parse fails
    Ok(num_a + num_b)
}

fn calculate_average(numbers: &[&str]) -> Result<f64, std::num::ParseIntError> {
    let mut sum = 0;
    for num_str in numbers {
        sum += num_str.parse::<i32>()?; // Propagate error
    }
    Ok(sum as f64 / numbers.len() as f64)
}

fn exercise_2_question_mark_operator() {
    println!("--- Exercise 2: The ? Operator ---");

    // Success case
    match calculate_sum("10", "20") {
        Ok(sum) => println!("10 + 20 = {}", sum),
        Err(e) => println!("Error: {}", e),
    }

    // Error case - first argument fails
    match calculate_sum("abc", "20") {
        Ok(sum) => println!("Result: {}", sum),
        Err(e) => println!("Error parsing first number: {}", e),
    }

    // Error case - second argument fails
    match calculate_sum("10", "xyz") {
        Ok(sum) => println!("Result: {}", sum),
        Err(e) => println!("Error parsing second number: {}", e),
    }

    // Calculate average
    let numbers = vec!["10", "20", "30", "40"];
    match calculate_average(&numbers) {
        Ok(avg) => println!("Average of {:?} = {:.2}", numbers, avg),
        Err(e) => println!("Error: {}", e),
    }

    let bad_numbers = vec!["10", "20", "bad", "40"];
    match calculate_average(&bad_numbers) {
        Ok(avg) => println!("Average: {:.2}", avg),
        Err(e) => println!("Error calculating average: {}", e),
    }

    println!();
}

// ============================================================================
// Exercise 3: Custom Error Types
// ============================================================================

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot(f64),
    Overflow,
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::NegativeSquareRoot(n) => {
                write!(f, "Cannot take square root of negative number: {}", n)
            }
            MathError::Overflow => write!(f, "Mathematical overflow occurred"),
        }
    }
}

impl std::error::Error for MathError {}

fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot(x))
    } else {
        Ok(x.sqrt())
    }
}

fn exercise_3_custom_errors() {
    println!("--- Exercise 3: Custom Error Types ---");

    // Division
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("10.0 / 2.0 = {:.2}", result),
        Err(e) => println!("Error: {}", e),
    }

    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("Result: {:.2}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Square root
    match safe_sqrt(16.0) {
        Ok(result) => println!("√16 = {:.2}", result),
        Err(e) => println!("Error: {}", e),
    }

    match safe_sqrt(-4.0) {
        Ok(result) => println!("Result: {:.2}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Pattern matching on different error types
    let result = safe_divide(5.0, 0.0);
    match result {
        Ok(val) => println!("Success: {}", val),
        Err(MathError::DivisionByZero) => println!("⚠️  Division by zero detected!"),
        Err(MathError::NegativeSquareRoot(n)) => {
            println!("⚠️  Tried to sqrt negative: {}", n)
        }
        Err(MathError::Overflow) => println!("⚠️  Overflow!"),
    }

    println!();
}

// ============================================================================
// Exercise 4: Error Type Conversion
// ============================================================================

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Math(MathError),
    Custom(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::Math(e) => write!(f, "Math error: {}", e),
            AppError::Custom(s) => write!(f, "Error: {}", s),
        }
    }
}

impl std::error::Error for AppError {}

// Implement From for automatic conversion
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::Parse(err)
    }
}

impl From<MathError> for AppError {
    fn from(err: MathError) -> Self {
        AppError::Math(err)
    }
}

fn complex_operation(a: &str, b: &str) -> Result<f64, AppError> {
    // Parse strings (ParseIntError automatically converted to AppError)
    let num_a: i32 = a.parse()?;
    let num_b: i32 = b.parse()?;

    // Perform division (MathError automatically converted to AppError)
    let result = safe_divide(num_a as f64, num_b as f64)?;

    Ok(result)
}

fn exercise_4_error_conversion() {
    println!("--- Exercise 4: Error Type Conversion ---");

    // Success
    match complex_operation("10", "2") {
        Ok(result) => println!("10 / 2 = {:.2}", result),
        Err(e) => println!("{}", e),
    }

    // Parse error
    match complex_operation("abc", "2") {
        Ok(result) => println!("Result: {:.2}", result),
        Err(e) => println!("{}", e),
    }

    // Math error
    match complex_operation("10", "0") {
        Ok(result) => println!("Result: {:.2}", result),
        Err(e) => println!("{}", e),
    }

    // Pattern match on specific error types
    let result = complex_operation("bad", "5");
    match result {
        Ok(val) => println!("Success: {}", val),
        Err(AppError::Parse(e)) => println!("📝 Parse error: {}", e),
        Err(AppError::Math(e)) => println!("🔢 Math error: {}", e),
        Err(e) => println!("Other error: {}", e),
    }

    println!();
}

// ============================================================================
// Exercise 5: Result Methods
// ============================================================================

fn exercise_5_result_methods() {
    println!("--- Exercise 5: Result Methods ---");

    // unwrap_or - provide default
    let good: Result<i32, String> = Ok(10);
    let bad: Result<i32, String> = Err(String::from("error"));

    println!("good.unwrap_or(0) = {}", good.unwrap_or(0));
    println!("bad.unwrap_or(0) = {}", bad.clone().unwrap_or(0));

    // unwrap_or_else - compute default
    let value = bad.unwrap_or_else(|e| {
        println!("  Error occurred: '{}', using default", e);
        -1
    });
    println!("value = {}", value);

    println!();

    // map - transform Ok value
    let result: Result<i32, String> = Ok(5);
    let doubled = result.map(|x| x * 2);
    println!("Ok(5).map(|x| x * 2) = {:?}", doubled);

    let result: Result<i32, String> = Err(String::from("error"));
    let doubled = result.map(|x| x * 2);
    println!("Err.map(|x| x * 2) = {:?}", doubled);

    println!();

    // map_err - transform Err value
    let result: Result<i32, u32> = Err(404);
    let with_message = result.map_err(|code| format!("Error code: {}", code));
    println!("map_err result: {:?}", with_message);

    println!();

    // and_then - chain Result-returning functions
    let result = Ok(10)
        .and_then(|x| Ok(x * 2))
        .and_then(|x| Ok(x + 5))
        .and_then(|x| if x > 20 { Ok(x) } else { Err("Too small") });
    println!("Chained operations: {:?}", result);

    println!();

    // or_else - provide fallback
    let result: Result<i32, &str> = Err("first failed")
        .or_else(|_| Err("second failed"))
        .or_else(|_| Ok(42));
    println!("Fallback result: {:?}", result);

    println!();
}

// ============================================================================
// Exercise 6: When to Panic vs Result
// ============================================================================

fn must_be_positive(x: i32) {
    // Use panic! for programming errors
    assert!(x > 0, "x must be positive, got: {}", x);
    println!("Processing positive number: {}", x);
}

fn validate_positive(x: i32) -> Result<i32, String> {
    // Use Result for validation that might fail
    if x > 0 {
        Ok(x)
    } else {
        Err(format!("Expected positive number, got: {}", x))
    }
}

fn exercise_6_when_to_panic() {
    println!("--- Exercise 6: When to Panic vs Result ---");

    // Using assertions (panics on failure)
    must_be_positive(5);
    must_be_positive(100);

    // Uncomment to see panic:
    // must_be_positive(-1);  // PANICS!

    println!();

    // Using Result for validation
    match validate_positive(5) {
        Ok(n) => println!("Valid: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match validate_positive(-1) {
        Ok(n) => println!("Valid: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    println!();

    // unwrap() - use only when you're SURE it won't fail
    let always_ok: Result<i32, String> = Ok(42);
    let value = always_ok.unwrap(); // Safe here
    println!("Unwrapped value: {}", value);

    // expect() - better than unwrap for debugging
    let value: i32 = Ok::<i32, String>(100).expect("This should never fail");
    println!("Expected value: {}", value);

    println!();
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(100, 10), Ok(10));
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10, 0).is_err());
    }

    #[test]
    fn test_calculate_sum() {
        assert_eq!(calculate_sum("10", "20"), Ok(30));
        assert!(calculate_sum("abc", "20").is_err());
    }

    #[test]
    fn test_safe_sqrt() {
        assert_eq!(safe_sqrt(16.0).unwrap(), 4.0);
        assert!(safe_sqrt(-1.0).is_err());
    }

    #[test]
    fn test_error_conversion() {
        let result = complex_operation("10", "2");
        assert!(result.is_ok());

        let result = complex_operation("bad", "2");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_positive() {
        assert_eq!(validate_positive(5), Ok(5));
        assert!(validate_positive(-1).is_err());
        assert!(validate_positive(0).is_err());
    }

    #[test]
    #[should_panic(expected = "must be positive")]
    fn test_must_be_positive_panics() {
        must_be_positive(-1);
    }
}
