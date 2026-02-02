// Example: Error Handling Patterns
// Run with: cargo run --example error_patterns

use std::fmt;

fn main() {
    println!("=== Error Handling Patterns ===\n");

    basic_result_usage();
    question_mark_operator();
    custom_error_types();
    error_conversion();
    combining_results();
    box_dyn_error();
}

// ============================================================================
// Basic Result Usage
// ============================================================================

fn basic_result_usage() {
    println!("--- Basic Result Usage ---");

    // Simple Result
    fn parse_config(input: &str) -> Result<Config, String> {
        if input.is_empty() {
            return Err(String::from("Input cannot be empty"));
        }

        Ok(Config {
            name: input.to_string(),
            version: 1,
        })
    }

    #[derive(Debug)]
    struct Config {
        name: String,
        version: u32,
    }

    // Handle with match
    match parse_config("my-config") {
        Ok(config) => println!("✅ Config loaded: {:?}", config),
        Err(e) => println!("❌ Error: {}", e),
    }

    match parse_config("") {
        Ok(config) => println!("Config: {:?}", config),
        Err(e) => println!("❌ Error: {}", e),
    }

    println!();
}

// ============================================================================
// The ? Operator - Error Propagation
// ============================================================================

fn question_mark_operator() {
    println!("--- The ? Operator ---");

    fn read_and_process() -> Result<ProcessedData, std::num::ParseIntError> {
        let data = "42";
        let number = data.parse::<i32>()?; // Propagate error if parse fails
        let doubled = number * 2;

        Ok(ProcessedData {
            original: number,
            processed: doubled,
        })
    }

    #[derive(Debug)]
    struct ProcessedData {
        original: i32,
        processed: i32,
    }

    match read_and_process() {
        Ok(data) => println!("✅ Data: {:?}", data),
        Err(e) => println!("❌ Error: {}", e),
    }

    // Multiple ? in sequence
    fn multi_step() -> Result<i32, std::num::ParseIntError> {
        let a = "10".parse::<i32>()?;
        let b = "20".parse::<i32>()?;
        let c = "30".parse::<i32>()?;
        Ok(a + b + c)
    }

    match multi_step() {
        Ok(sum) => println!("✅ Sum: {}", sum),
        Err(e) => println!("❌ Error: {}", e),
    }

    println!();
}

// ============================================================================
// Custom Error Types
// ============================================================================

#[derive(Debug)]
enum RobotError {
    NotFound(u32),
    LowBattery { id: u32, level: u8 },
    ConnectionLost,
    InvalidCommand(String),
}

impl fmt::Display for RobotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RobotError::NotFound(id) => write!(f, "Robot {} not found", id),
            RobotError::LowBattery { id, level } => {
                write!(f, "Robot {} has low battery ({}%)", id, level)
            }
            RobotError::ConnectionLost => write!(f, "Connection to robot lost"),
            RobotError::InvalidCommand(cmd) => write!(f, "Invalid command: {}", cmd),
        }
    }
}

impl std::error::Error for RobotError {}

fn custom_error_types() {
    println!("--- Custom Error Types ---");

    fn get_robot(id: u32) -> Result<Robot, RobotError> {
        match id {
            1 => Ok(Robot {
                id: 1,
                name: String::from("Alpha"),
                battery: 80,
            }),
            2 => Err(RobotError::LowBattery { id: 2, level: 5 }),
            3 => Err(RobotError::ConnectionLost),
            _ => Err(RobotError::NotFound(id)),
        }
    }

    #[derive(Debug)]
    struct Robot {
        id: u32,
        name: String,
        battery: u8,
    }

    // Try different robot IDs
    for id in 1..=4 {
        match get_robot(id) {
            Ok(robot) => println!("✅ Found robot: {:?}", robot),
            Err(e) => println!("❌ {}", e),
        }
    }

    // Pattern match on specific errors
    match get_robot(2) {
        Ok(robot) => println!("Robot: {:?}", robot),
        Err(RobotError::LowBattery { id, level }) => {
            println!("⚠️  Robot {} needs charging! Battery at {}%", id, level);
        }
        Err(e) => println!("Other error: {}", e),
    }

    println!();
}

// ============================================================================
// Error Type Conversion
// ============================================================================

#[derive(Debug)]
enum AppError {
    Parse(std::num::ParseIntError),
    Robot(RobotError),
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::Robot(e) => write!(f, "Robot error: {}", e),
            AppError::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::Parse(err)
    }
}

impl From<RobotError> for AppError {
    fn from(err: RobotError) -> Self {
        AppError::Robot(err)
    }
}

fn error_conversion() {
    println!("--- Error Type Conversion ---");

    fn process_robot_command(robot_id: &str, command: &str) -> Result<String, AppError> {
        // Parse ID (ParseIntError automatically converted to AppError)
        let id: u32 = robot_id.parse()?;

        // Validate command
        if command.is_empty() {
            return Err(AppError::Custom(String::from("Command cannot be empty")));
        }

        // This would normally interact with robot
        // For demo, just return success
        Ok(format!("Robot {} executing: {}", id, command))
    }

    // Success
    match process_robot_command("1", "move_forward") {
        Ok(msg) => println!("✅ {}", msg),
        Err(e) => println!("❌ {}", e),
    }

    // Parse error
    match process_robot_command("abc", "move") {
        Ok(msg) => println!("✅ {}", msg),
        Err(e) => println!("❌ {}", e),
    }

    // Custom error
    match process_robot_command("1", "") {
        Ok(msg) => println!("✅ {}", msg),
        Err(e) => println!("❌ {}", e),
    }

    println!();
}

// ============================================================================
// Combining Results
// ============================================================================

fn combining_results() {
    println!("--- Combining Results ---");

    // Try multiple sources, use first success
    fn load_config() -> Result<String, String> {
        load_from_file()
            .or_else(|_| load_from_env())
            .or_else(|_| load_default())
    }

    fn load_from_file() -> Result<String, String> {
        Err(String::from("File not found"))
    }

    fn load_from_env() -> Result<String, String> {
        Err(String::from("Env var not set"))
    }

    fn load_default() -> Result<String, String> {
        Ok(String::from("default config"))
    }

    match load_config() {
        Ok(config) => println!("✅ Config loaded: {}", config),
        Err(e) => println!("❌ {}", e),
    }

    println!();

    // Process multiple items
    fn process_all(items: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
        items.iter().map(|s| s.parse::<i32>()).collect()
    }

    // All succeed
    match process_all(&["1", "2", "3"]) {
        Ok(numbers) => println!("✅ Processed: {:?}", numbers),
        Err(e) => println!("❌ {}", e),
    }

    // One fails - short circuits!
    match process_all(&["1", "bad", "3"]) {
        Ok(numbers) => println!("Processed: {:?}", numbers),
        Err(e) => println!("❌ Failed at first error: {}", e),
    }

    println!();
}

// ============================================================================
// Box<dyn Error> - For Multiple Error Types
// ============================================================================

fn box_dyn_error() {
    println!("--- Box<dyn Error> ---");

    fn do_many_things() -> Result<(), Box<dyn std::error::Error>> {
        // Can return any error type!
        let number = "42".parse::<i32>()?; // ParseIntError
        println!("Parsed: {}", number);

        if number > 50 {
            return Err(Box::new(RobotError::ConnectionLost)); // RobotError
        }

        Ok(())
    }

    match do_many_things() {
        Ok(()) => println!("✅ All operations succeeded"),
        Err(e) => println!("❌ Error: {}", e),
    }

    // Function that can fail in multiple ways
    fn complex_operation(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let n: i32 = input.parse()?;

        if n < 0 {
            return Err(Box::new(RobotError::InvalidCommand(
                "Negative numbers not allowed".to_string(),
            )));
        }

        Ok(n * 2)
    }

    match complex_operation("21") {
        Ok(result) => println!("✅ Result: {}", result),
        Err(e) => println!("❌ Error: {}", e),
    }

    match complex_operation("-5") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("❌ Error: {}", e),
    }

    println!();
}
