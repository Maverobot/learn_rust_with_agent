// Module 3: Structs, Enums, and Pattern Matching
// Interactive exercises - uncomment sections and fix the code!

fn main() {
    println!("=== Module 3: Structs, Enums, and Pattern Matching ===\n");

    // Uncomment each section as you work through the exercises
    exercise_1_basic_structs();
    exercise_2_methods();
    exercise_3_enums();
    exercise_4_pattern_matching();
    exercise_5_option();
    exercise_6_result();

    println!("\n✅ All exercises completed!");
}

// ============================================================================
// Exercise 1: Basic Structs
// ============================================================================

#[derive(Debug)]
struct Robot {
    id: u32,
    name: String,
    position: (f64, f64),
    active: bool,
}

fn exercise_1_basic_structs() {
    println!("--- Exercise 1: Basic Structs ---");

    // TODO: Create a Robot instance with:
    // - id: 1
    // - name: "Explorer-1"
    // - position: (0.0, 0.0)
    // - active: true
    let robot = Robot {
        id: 1,
        name: String::from("Explorer-1"),
        position: (0.0, 0.0),
        active: true,
    };

    println!("Robot created: {:?}", robot);

    // TODO: Access fields
    println!("Robot name: {}", robot.name);
    println!("Robot position: {:?}", robot.position);

    // TODO: Create a second robot using field init shorthand
    let id = 2;
    let name = String::from("Scout-2");
    let robot2 = Robot {
        id,
        name,
        position: (10.0, 5.0),
        active: false,
    };
    println!("Robot 2: {:?}", robot2);

    // TODO: Create a robot using struct update syntax
    let robot3 = Robot {
        id: 3,
        name: String::from("Clone-3"),
        ..robot // Use robot's other fields (but robot.name was moved!)
    };
    // Note: robot can't be used anymore because name was moved
    // Uncomment to see the error:
    // println!("Original robot: {:?}", robot);
    println!("Robot 3 (cloned): {:?}", robot3);

    println!();
}

// ============================================================================
// Exercise 2: Methods and Associated Functions
// ============================================================================

impl Robot {
    // Associated function (constructor-like)
    fn new(id: u32, name: String) -> Self {
        Robot {
            id,
            name,
            position: (0.0, 0.0),
            active: true,
        }
    }

    // Method that borrows self
    fn distance_from_origin(&self) -> f64 {
        let (x, y) = self.position;
        (x * x + y * y).sqrt()
    }

    // Method that mutates self
    fn move_to(&mut self, x: f64, y: f64) {
        self.position = (x, y);
    }

    // Method that takes ownership (consuming)
    fn deactivate(mut self) -> String {
        self.active = false;
        format!("Robot {} deactivated", self.name)
    }

    // Method returning info
    fn status(&self) -> String {
        format!(
            "Robot {} (ID: {}) at {:?} - {}",
            self.name,
            self.id,
            self.position,
            if self.active { "Active" } else { "Inactive" }
        )
    }
}

fn exercise_2_methods() {
    println!("--- Exercise 2: Methods ---");

    // TODO: Create a robot using the new() associated function
    let mut robot = Robot::new(10, String::from("MethodBot"));
    println!("Created: {}", robot.status());

    // TODO: Use methods
    println!("Distance from origin: {:.2}", robot.distance_from_origin());

    // TODO: Move the robot
    robot.move_to(3.0, 4.0);
    println!("After moving: {}", robot.status());
    println!("New distance: {:.2}", robot.distance_from_origin());

    // TODO: Deactivate (consumes robot)
    let message = robot.deactivate();
    println!("{}", message);

    // Uncomment to see the error - robot was consumed!
    // println!("{}", robot.status());

    println!();
}

// ============================================================================
// Exercise 3: Enums with Data
// ============================================================================

#[derive(Debug)]
enum SensorReading {
    Camera(String),
    Lidar(Vec<f64>),
    GPS(f64, f64),
    IMU { roll: f64, pitch: f64, yaw: f64 },
    Error(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: f64, y: f64 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn exercise_3_enums() {
    println!("--- Exercise 3: Enums with Data ---");

    // TODO: Create different sensor readings
    let readings = vec![
        SensorReading::Camera(String::from("image_001.png")),
        SensorReading::Lidar(vec![1.2, 3.4, 5.6, 7.8]),
        SensorReading::GPS(37.7749, -122.4194),
        SensorReading::IMU {
            roll: 0.1,
            pitch: 0.2,
            yaw: 90.0,
        },
        SensorReading::Error(String::from("Sensor timeout")),
    ];

    for reading in readings {
        println!("Reading: {:?}", reading);
    }

    // TODO: Create messages
    let messages = vec![
        Message::Move { x: 10.0, y: 20.0 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];

    for msg in messages {
        println!("Message: {:?}", msg);
    }

    println!();
}

// ============================================================================
// Exercise 4: Pattern Matching
// ============================================================================

fn process_reading(reading: SensorReading) {
    match reading {
        SensorReading::Camera(filename) => {
            println!("📷 Processing camera image: {}", filename);
        }
        SensorReading::Lidar(distances) => {
            println!("📡 Got {} distance readings", distances.len());
            if let Some(&first) = distances.first() {
                println!("   First reading: {:.2}m", first);
            }
        }
        SensorReading::GPS(lat, lon) => {
            println!("🌍 GPS Position: {:.4}°N, {:.4}°W", lat, lon);
        }
        SensorReading::IMU { roll, pitch, yaw } => {
            println!("🧭 IMU Orientation:");
            println!(
                "   Roll: {:.2}°, Pitch: {:.2}°, Yaw: {:.2}°",
                roll, pitch, yaw
            );
        }
        SensorReading::Error(msg) => {
            eprintln!("❌ Sensor error: {}", msg);
        }
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("🛑 Quitting..."),
        Message::Move { x, y } => println!("➡️  Moving to ({}, {})", x, y),
        Message::Write(text) => println!("✍️  Writing: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("🎨 Changing color to RGB({}, {}, {})", r, g, b)
        }
    }
}

fn classify_distance(distance: f64) -> &'static str {
    match distance {
        d if d < 0.0 => "Invalid (negative)",
        0.0 => "At origin",
        d if d <= 1.0 => "Very close",
        d if d <= 5.0 => "Close",
        d if d <= 10.0 => "Medium distance",
        _ => "Far away",
    }
}

fn exercise_4_pattern_matching() {
    println!("--- Exercise 4: Pattern Matching ---");

    // TODO: Process sensor readings
    let reading = SensorReading::GPS(37.7749, -122.4194);
    process_reading(reading);

    let reading = SensorReading::Lidar(vec![1.5, 2.3, 5.1]);
    process_reading(reading);

    let reading = SensorReading::Error(String::from("Connection lost"));
    process_reading(reading);

    println!();

    // TODO: Process messages
    process_message(Message::Move { x: 100.0, y: 200.0 });
    process_message(Message::Write(String::from("Hello from Rust!")));
    process_message(Message::Quit);

    println!();

    // TODO: Classify distances
    let distances = vec![0.0, 0.5, 3.0, 7.0, 15.0, -1.0];
    for &dist in &distances {
        println!("Distance {:.1}: {}", dist, classify_distance(dist));
    }

    println!();
}

// ============================================================================
// Exercise 5: Option<T>
// ============================================================================

fn find_robot_by_id(id: u32) -> Option<Robot> {
    match id {
        1 => Some(Robot::new(1, String::from("Alpha"))),
        2 => Some(Robot::new(2, String::from("Beta"))),
        3 => Some(Robot::new(3, String::from("Gamma"))),
        _ => None,
    }
}

fn exercise_5_option() {
    println!("--- Exercise 5: Option<T> ---");

    // TODO: Use match with Option
    let result = find_robot_by_id(2);
    match result {
        Some(robot) => println!("Found: {}", robot.status()),
        None => println!("Robot not found"),
    }

    // TODO: Use if let
    if let Some(robot) = find_robot_by_id(3) {
        println!("Found with if let: {}", robot.status());
    }

    // TODO: Try to find non-existent robot
    if let Some(robot) = find_robot_by_id(99) {
        println!("Found: {}", robot.status());
    } else {
        println!("Robot 99 not found");
    }

    // TODO: Use unwrap_or and unwrap_or_else
    let robot = find_robot_by_id(1).unwrap_or(Robot::new(0, String::from("Default")));
    println!("Got robot: {}", robot.status());

    let robot = find_robot_by_id(99).unwrap_or_else(|| {
        println!("Creating default robot...");
        Robot::new(0, String::from("Fallback"))
    });
    println!("Got robot: {}", robot.status());

    // TODO: Use map and and_then
    let distance = find_robot_by_id(1).map(|r| r.distance_from_origin());
    println!("Distance: {:?}", distance);

    println!();
}

// ============================================================================
// Exercise 6: Result<T, E>
// ============================================================================

#[derive(Debug)]
enum SensorError {
    InvalidId,
    Timeout,
    ConnectionLost,
}

fn read_sensor(id: u32) -> Result<f64, SensorError> {
    match id {
        1..=5 => Ok(42.0 + id as f64),
        6 => Err(SensorError::Timeout),
        7 => Err(SensorError::ConnectionLost),
        _ => Err(SensorError::InvalidId),
    }
}

fn process_sensor_reading(id: u32) -> Result<String, SensorError> {
    let value = read_sensor(id)?; // Propagate error with ?
    Ok(format!("Sensor {} reading: {:.2}", id, value))
}

fn exercise_6_result() {
    println!("--- Exercise 6: Result<T, E> ---");

    // TODO: Handle Result with match
    let result = read_sensor(1);
    match result {
        Ok(value) => println!("✅ Sensor reading: {:.2}", value),
        Err(e) => println!("❌ Error: {:?}", e),
    }

    // TODO: Use if let for Ok case
    if let Ok(value) = read_sensor(2) {
        println!("✅ Got value: {:.2}", value);
    }

    // TODO: Handle different error cases
    for id in 1..=8 {
        match read_sensor(id) {
            Ok(value) => println!("Sensor {}: {:.2}", id, value),
            Err(SensorError::InvalidId) => println!("Sensor {}: Invalid ID", id),
            Err(SensorError::Timeout) => println!("Sensor {}: Timeout!", id),
            Err(SensorError::ConnectionLost) => println!("Sensor {}: Connection lost!", id),
        }
    }

    println!();

    // TODO: Use ? operator for error propagation
    match process_sensor_reading(3) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error processing: {:?}", e),
    }

    match process_sensor_reading(6) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error processing: {:?}", e),
    }

    println!();
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_creation() {
        let robot = Robot::new(1, String::from("TestBot"));
        assert_eq!(robot.id, 1);
        assert_eq!(robot.name, "TestBot");
        assert_eq!(robot.position, (0.0, 0.0));
        assert!(robot.active);
    }

    #[test]
    fn test_robot_movement() {
        let mut robot = Robot::new(1, String::from("MoveBot"));
        robot.move_to(3.0, 4.0);
        assert_eq!(robot.position, (3.0, 4.0));
        assert_eq!(robot.distance_from_origin(), 5.0);
    }

    #[test]
    fn test_option_some() {
        let result = find_robot_by_id(1);
        assert!(result.is_some());
        if let Some(robot) = result {
            assert_eq!(robot.id, 1);
        }
    }

    #[test]
    fn test_option_none() {
        let result = find_robot_by_id(999);
        assert!(result.is_none());
    }

    #[test]
    fn test_result_ok() {
        let result = read_sensor(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 43.0);
    }

    #[test]
    fn test_result_error() {
        let result = read_sensor(999);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_propagation() {
        let result = process_sensor_reading(1);
        assert!(result.is_ok());

        let result = process_sensor_reading(6);
        assert!(result.is_err());
    }
}
