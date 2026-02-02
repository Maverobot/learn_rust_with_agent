// Example: Option and Result Types
// Run with: cargo run --example option_result

fn main() {
    println!("=== Option and Result Types ===\n");

    option_basics();
    option_methods();
    result_basics();
    result_methods();
    error_propagation();
    combining_option_result();
}

// ============================================================================
// Option Basics
// ============================================================================

#[derive(Debug)]
struct Robot {
    id: u32,
    name: String,
}

fn find_robot(id: u32) -> Option<Robot> {
    // Simulated database lookup
    match id {
        1 => Some(Robot {
            id: 1,
            name: String::from("Alpha"),
        }),
        2 => Some(Robot {
            id: 2,
            name: String::from("Beta"),
        }),
        3 => Some(Robot {
            id: 3,
            name: String::from("Gamma"),
        }),
        _ => None,
    }
}

fn option_basics() {
    println!("--- Option Basics ---");

    // Pattern matching
    match find_robot(1) {
        Some(robot) => println!("✅ Found: {:?}", robot),
        None => println!("❌ Not found"),
    }

    match find_robot(99) {
        Some(robot) => println!("✅ Found: {:?}", robot),
        None => println!("❌ Not found"),
    }

    println!();

    // if let
    if let Some(robot) = find_robot(2) {
        println!("Robot 2: {:?}", robot);
    }

    // Checking with is_some() and is_none()
    let result = find_robot(3);
    if result.is_some() {
        println!("Robot 3 exists!");
    }

    let result = find_robot(100);
    if result.is_none() {
        println!("Robot 100 doesn't exist");
    }

    println!();
}

// ============================================================================
// Option Methods
// ============================================================================

fn option_methods() {
    println!("--- Option Methods ---");

    // unwrap() - panics if None
    let robot = find_robot(1).unwrap();
    println!("Unwrapped: {:?}", robot);

    // Uncommenting this will panic!
    // let robot = find_robot(99).unwrap();

    // expect() - panics with custom message
    let robot = find_robot(2).expect("Robot 2 should exist");
    println!("Expected: {:?}", robot);

    println!();

    // unwrap_or() - provide default value
    let robot = find_robot(99).unwrap_or(Robot {
        id: 0,
        name: String::from("Default"),
    });
    println!("With default: {:?}", robot);

    // unwrap_or_else() - compute default value
    let robot = find_robot(99).unwrap_or_else(|| {
        println!("  Computing default robot...");
        Robot {
            id: 0,
            name: String::from("Computed Default"),
        }
    });
    println!("With computed default: {:?}", robot);

    println!();

    // map() - transform the Some value
    let name = find_robot(1).map(|r| r.name);
    println!("Mapped name: {:?}", name); // Some("Alpha")

    let name = find_robot(99).map(|r| r.name);
    println!("Mapped name (None): {:?}", name); // None

    // and_then() - chain Option-returning operations
    let uppercase_name = find_robot(1).and_then(|r| Some(r.name.to_uppercase()));
    println!("Chained: {:?}", uppercase_name);

    // or() - provide alternative Option
    let robot = find_robot(99).or(find_robot(1));
    println!("With alternative: {:?}", robot);

    println!();
}

// ============================================================================
// Result Basics
// ============================================================================

#[derive(Debug)]
enum SensorError {
    NotFound,
    Timeout,
    InvalidData(String),
}

fn read_sensor(id: u32) -> Result<f64, SensorError> {
    match id {
        1..=5 => Ok(20.0 + id as f64),
        6 => Err(SensorError::Timeout),
        7 => Err(SensorError::InvalidData(String::from("NaN"))),
        _ => Err(SensorError::NotFound),
    }
}

fn result_basics() {
    println!("--- Result Basics ---");

    // Pattern matching
    match read_sensor(1) {
        Ok(value) => println!("✅ Sensor 1: {:.2}", value),
        Err(e) => println!("❌ Error: {:?}", e),
    }

    match read_sensor(99) {
        Ok(value) => println!("✅ Sensor 99: {:.2}", value),
        Err(e) => println!("❌ Error: {:?}", e),
    }

    println!();

    // Handling different errors
    for id in [1, 6, 7, 99] {
        match read_sensor(id) {
            Ok(value) => println!("Sensor {}: {:.2}", id, value),
            Err(SensorError::NotFound) => println!("Sensor {}: Not found", id),
            Err(SensorError::Timeout) => println!("Sensor {}: Timeout!", id),
            Err(SensorError::InvalidData(msg)) => {
                println!("Sensor {}: Invalid data ({})", id, msg)
            }
        }
    }

    println!();

    // is_ok() and is_err()
    if read_sensor(1).is_ok() {
        println!("Sensor 1 is working");
    }

    if read_sensor(99).is_err() {
        println!("Sensor 99 has an error");
    }

    println!();
}

// ============================================================================
// Result Methods
// ============================================================================

fn result_methods() {
    println!("--- Result Methods ---");

    // unwrap() and expect()
    let value = read_sensor(1).unwrap();
    println!("Unwrapped: {:.2}", value);

    let value = read_sensor(2).expect("Sensor 2 should work");
    println!("Expected: {:.2}", value);

    // unwrap_or() and unwrap_or_else()
    let value = read_sensor(99).unwrap_or(0.0);
    println!("With default: {:.2}", value);

    let value = read_sensor(99).unwrap_or_else(|e| {
        println!("  Error occurred: {:?}, using default", e);
        -1.0
    });
    println!("With computed default: {:.2}", value);

    println!();

    // map() - transform Ok value
    let doubled = read_sensor(1).map(|v| v * 2.0);
    println!("Doubled: {:?}", doubled);

    // map_err() - transform Err value
    let result = read_sensor(99).map_err(|e| format!("Sensor error: {:?}", e));
    println!("Mapped error: {:?}", result);

    println!();
}

// ============================================================================
// Error Propagation with ?
// ============================================================================

fn calculate_average() -> Result<f64, SensorError> {
    let s1 = read_sensor(1)?; // Propagates error if Err
    let s2 = read_sensor(2)?;
    let s3 = read_sensor(3)?;

    Ok((s1 + s2 + s3) / 3.0)
}

fn calculate_average_with_error() -> Result<f64, SensorError> {
    let s1 = read_sensor(1)?;
    let s2 = read_sensor(99)?; // This will error!
    let s3 = read_sensor(3)?;

    Ok((s1 + s2 + s3) / 3.0)
}

fn error_propagation() {
    println!("--- Error Propagation with ? ---");

    match calculate_average() {
        Ok(avg) => println!("✅ Average: {:.2}", avg),
        Err(e) => println!("❌ Error: {:?}", e),
    }

    match calculate_average_with_error() {
        Ok(avg) => println!("✅ Average: {:.2}", avg),
        Err(e) => println!("❌ Error calculating average: {:?}", e),
    }

    println!();
}

// ============================================================================
// Combining Option and Result
// ============================================================================

fn get_robot_sensor(robot_id: u32, sensor_id: u32) -> Result<Option<f64>, SensorError> {
    // First check if robot exists
    let robot = find_robot(robot_id);

    if robot.is_none() {
        return Ok(None); // Robot doesn't exist, but not an error
    }

    // Robot exists, read sensor
    let value = read_sensor(sensor_id)?; // Propagate sensor errors
    Ok(Some(value))
}

fn combining_option_result() {
    println!("--- Combining Option and Result ---");

    // Success case
    match get_robot_sensor(1, 1) {
        Ok(Some(value)) => println!("✅ Robot 1, Sensor 1: {:.2}", value),
        Ok(None) => println!("⚠️  Robot not found"),
        Err(e) => println!("❌ Error: {:?}", e),
    }

    // Robot not found
    match get_robot_sensor(99, 1) {
        Ok(Some(value)) => println!("✅ Robot 99, Sensor 1: {:.2}", value),
        Ok(None) => println!("⚠️  Robot 99 not found"),
        Err(e) => println!("❌ Error: {:?}", e),
    }

    // Sensor error
    match get_robot_sensor(1, 99) {
        Ok(Some(value)) => println!("✅ Robot 1, Sensor 99: {:.2}", value),
        Ok(None) => println!("⚠️  Robot not found"),
        Err(e) => println!("❌ Sensor error: {:?}", e),
    }

    println!();

    // Converting between Option and Result
    let opt: Option<i32> = Some(42);
    let res: Result<i32, &str> = opt.ok_or("Value was None");
    println!("Option -> Result: {:?}", res);

    let opt: Option<i32> = None;
    let res: Result<i32, &str> = opt.ok_or("Value was None");
    println!("Option (None) -> Result: {:?}", res);

    println!();

    // Result -> Option (discards error)
    let res: Result<i32, &str> = Ok(42);
    let opt = res.ok();
    println!("Result -> Option: {:?}", opt);

    let res: Result<i32, &str> = Err("Error!");
    let opt = res.ok();
    println!("Result (Err) -> Option: {:?}", opt);

    println!();
}

// ============================================================================
// Practical Example: Configuration Loading
// ============================================================================

#[derive(Debug)]
struct Config {
    robot_id: u32,
    max_speed: f64,
    timeout: u64,
}

impl Config {
    fn load(id: u32) -> Result<Self, String> {
        // Simulate loading config
        if id == 0 {
            return Err(String::from("Invalid robot ID"));
        }

        if id > 100 {
            return Err(String::from("Robot ID out of range"));
        }

        Ok(Config {
            robot_id: id,
            max_speed: 10.0,
            timeout: 5000,
        })
    }

    fn validate(&self) -> Result<(), String> {
        if self.max_speed <= 0.0 {
            return Err(String::from("Max speed must be positive"));
        }
        if self.timeout == 0 {
            return Err(String::from("Timeout must be non-zero"));
        }
        Ok(())
    }
}

fn _initialize_robot(id: u32) -> Result<Config, String> {
    let config = Config::load(id)?;
    config.validate()?;
    Ok(config)
}
