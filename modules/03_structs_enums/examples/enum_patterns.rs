// Example: Enum Patterns and Pattern Matching
// Run with: cargo run --example enum_patterns

fn main() {
    println!("=== Enum Patterns and Pattern Matching ===\n");

    basic_enums();
    enums_with_data();
    pattern_matching();
    match_guards();
    if_let_while_let();
}

// ============================================================================
// Basic Enums
// ============================================================================

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum RobotState {
    Idle,
    Moving,
    Charging,
    Error,
}

fn basic_enums() {
    println!("--- Basic Enums ---");

    let direction = Direction::North;
    println!("Direction: {:?}", direction);

    let state = RobotState::Moving;
    println!("Robot state: {:?}", state);

    // Pattern matching on simple enums
    match direction {
        Direction::North => println!("Heading north!"),
        Direction::South => println!("Heading south!"),
        Direction::East => println!("Heading east!"),
        Direction::West => println!("Heading west!"),
    }

    // Using if with equality
    if direction == Direction::North {
        println!("Confirmed: going north");
    }

    println!();
}

// ============================================================================
// Enums with Data
// ============================================================================

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: f64, y: f64 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

#[derive(Debug)]
enum SensorReading {
    Temperature(f64),
    Pressure(f64),
    GPS {
        lat: f64,
        lon: f64,
        altitude: f64,
    },
    Camera {
        filename: String,
        resolution: (u32, u32),
    },
    Error {
        code: u32,
        message: String,
    },
}

fn enums_with_data() {
    println!("--- Enums with Data ---");

    let messages = vec![
        Message::Move { x: 10.0, y: 20.0 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];

    for msg in messages {
        match msg {
            Message::Quit => println!("🛑 Quit message received"),
            Message::Move { x, y } => println!("➡️  Move to ({}, {})", x, y),
            Message::Write(text) => println!("✍️  Write: '{}'", text),
            Message::ChangeColor(r, g, b) => {
                println!("🎨 Change color to RGB({}, {}, {})", r, g, b)
            }
        }
    }

    println!();

    let readings = vec![
        SensorReading::Temperature(23.5),
        SensorReading::Pressure(1013.25),
        SensorReading::GPS {
            lat: 37.7749,
            lon: -122.4194,
            altitude: 16.0,
        },
        SensorReading::Camera {
            filename: String::from("img_001.jpg"),
            resolution: (1920, 1080),
        },
        SensorReading::Error {
            code: 404,
            message: String::from("Sensor not found"),
        },
    ];

    for reading in readings {
        process_sensor_reading(reading);
    }

    println!();
}

fn process_sensor_reading(reading: SensorReading) {
    match reading {
        SensorReading::Temperature(temp) => {
            println!("🌡️  Temperature: {:.1}°C", temp);
        }
        SensorReading::Pressure(pressure) => {
            println!("🔽 Pressure: {:.2} hPa", pressure);
        }
        SensorReading::GPS { lat, lon, altitude } => {
            println!("🌍 GPS: {:.4}°, {:.4}° (alt: {:.1}m)", lat, lon, altitude);
        }
        SensorReading::Camera {
            filename,
            resolution: (width, height),
        } => {
            println!("📷 Camera: {} ({}x{})", filename, width, height);
        }
        SensorReading::Error { code, message } => {
            eprintln!("❌ Sensor Error {}: {}", code, message);
        }
    }
}

// ============================================================================
// Pattern Matching Features
// ============================================================================

#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn pattern_matching() {
    println!("--- Pattern Matching Features ---");

    let events = vec![
        WebEvent::PageLoad,
        WebEvent::KeyPress('x'),
        WebEvent::Paste(String::from("Hello")),
        WebEvent::Click { x: 20, y: 80 },
        WebEvent::PageUnload,
    ];

    for event in events {
        inspect_event(event);
    }

    println!();
}

fn inspect_event(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("📄 Page loaded"),
        WebEvent::PageUnload => println!("📄 Page unloaded"),

        // Destructure and use the value
        WebEvent::KeyPress(c) => println!("⌨️  Key pressed: '{}'", c),

        // Destructure with @
        WebEvent::Paste(s) if s.len() < 10 => {
            println!("📋 Pasted short text: '{}'", s);
        }
        WebEvent::Paste(s) => {
            println!("📋 Pasted long text: {} chars", s.len());
        }

        // Destructure struct-like variant
        WebEvent::Click { x, y } => println!("🖱️  Mouse clicked at ({}, {})", x, y),
    }
}

// ============================================================================
// Match Guards
// ============================================================================

fn match_guards() {
    println!("--- Match Guards ---");

    let numbers = vec![-5, 0, 1, 5, 10, 15, 100];

    for &num in &numbers {
        let description = match num {
            n if n < 0 => "negative",
            0 => "zero",
            n if n % 2 == 0 => "positive and even",
            n if n % 2 == 1 => "positive and odd",
            _ => "unreachable",
        };
        println!("{:3} is {}", num, description);
    }

    println!();

    // Match with ranges
    for &num in &numbers {
        let category = match num {
            i32::MIN..=-1 => "Negative",
            0 => "Zero",
            1..=10 => "Small positive",
            11..=100 => "Medium positive",
            _ => "Large positive",
        };
        println!("{:3} -> {}", num, category);
    }

    println!();
}

// ============================================================================
// if let and while let
// ============================================================================

fn if_let_while_let() {
    println!("--- if let and while let ---");

    // if let - match one pattern, ignore the rest
    let config = Some("config.toml");

    if let Some(filename) = config {
        println!("Using config file: {}", filename);
    } else {
        println!("No config file");
    }

    // With enums
    let message = Message::Write(String::from("Test"));

    if let Message::Write(text) = message {
        println!("Message text: {}", text);
    }

    // Can use else
    let message = Message::Quit;
    if let Message::Write(text) = message {
        println!("Message: {}", text);
    } else {
        println!("Not a Write message");
    }

    println!();

    // while let - keep matching until pattern fails
    let mut stack = vec![1, 2, 3, 4, 5];

    println!("Popping values:");
    while let Some(value) = stack.pop() {
        println!("  Popped: {}", value);
    }
    println!("Stack is now empty");

    println!();

    // Practical example: processing messages
    let mut messages = vec![
        Message::Write(String::from("Hello")),
        Message::Move { x: 1.0, y: 2.0 },
        Message::Write(String::from("World")),
        Message::Quit,
        Message::Write(String::from("This won't be processed")),
    ];

    println!("Processing messages until Quit:");
    while let Some(msg) = messages.pop() {
        match msg {
            Message::Quit => {
                println!("  Quit received, stopping");
                break;
            }
            Message::Write(text) => println!("  Processing: {}", text),
            Message::Move { x, y } => println!("  Moving to ({}, {})", x, y),
            _ => println!("  Other message"),
        }
    }

    println!();
}

// ============================================================================
// Nested Enums Example (Realistic)
// ============================================================================

#[derive(Debug)]
enum Command {
    Single(Action),
    Sequence(Vec<Action>),
}

#[derive(Debug)]
enum Action {
    Move { x: f64, y: f64 },
    Rotate(f64),
    Wait(u64),
}

fn _execute_command(cmd: Command) {
    match cmd {
        Command::Single(action) => {
            println!("Executing single action: {:?}", action);
        }
        Command::Sequence(actions) => {
            println!("Executing sequence of {} actions:", actions.len());
            for action in actions {
                println!("  - {:?}", action);
            }
        }
    }
}
