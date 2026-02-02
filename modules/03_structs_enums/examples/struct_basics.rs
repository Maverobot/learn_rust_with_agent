// Example: Struct Basics
// Run with: cargo run --example struct_basics

fn main() {
    println!("=== Struct Basics Example ===\n");

    // Basic struct definition and creation
    basic_struct_usage();

    // Tuple structs
    tuple_struct_usage();

    // Struct update syntax
    struct_update_syntax();

    // Methods
    method_usage();
}

// ============================================================================
// Basic Structs
// ============================================================================

#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Robot {
    id: u32,
    name: String,
    location: Point,
    battery_level: u8,
}

fn basic_struct_usage() {
    println!("--- Basic Struct Usage ---");

    // Create a point
    let origin = Point { x: 0.0, y: 0.0 };
    println!("Origin: {:?}", origin);

    // Create a robot
    let robot = Robot {
        id: 1,
        name: String::from("Explorer-1"),
        location: Point { x: 10.0, y: 20.0 },
        battery_level: 100,
    };

    println!("Robot: {:?}", robot);
    println!("Robot name: {}", robot.name);
    println!(
        "Robot location: ({}, {})",
        robot.location.x, robot.location.y
    );

    // Field init shorthand
    let id = 2;
    let name = String::from("Scout-2");
    let robot2 = Robot {
        id,
        name,
        location: origin.clone(),
        battery_level: 85,
    };
    println!("Robot 2: {:?}", robot2);

    println!();
}

// ============================================================================
// Tuple Structs
// ============================================================================

#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Meters(f64);

#[derive(Debug)]
struct Seconds(f64);

fn tuple_struct_usage() {
    println!("--- Tuple Struct Usage ---");

    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);

    println!("Red: {:?}", red);
    println!("Green: RGB({}, {}, {})", green.0, green.1, green.2);

    // Type safety with newtype pattern
    let distance = Meters(100.0);
    let time = Seconds(10.0);

    println!("Distance: {:?}", distance);
    println!("Time: {:?}", time);

    // This demonstrates type safety - uncomment to see the error:
    // let speed = distance + time;  // Error! Can't add different types

    let speed = distance.0 / time.0;
    println!("Speed: {:.2} m/s", speed);

    println!();
}

// ============================================================================
// Struct Update Syntax
// ============================================================================

fn struct_update_syntax() {
    println!("--- Struct Update Syntax ---");

    let robot1 = Robot {
        id: 1,
        name: String::from("Original"),
        location: Point { x: 0.0, y: 0.0 },
        battery_level: 100,
    };

    println!("Robot 1: {:?}", robot1);

    // Create a new robot with some fields from robot1
    // Note: This moves robot1.name, so robot1 can't be fully used after
    let robot2 = Robot {
        id: 2,
        name: String::from("Modified"),
        ..robot1 // Use robot1's location and battery_level
    };

    println!("Robot 2: {:?}", robot2);

    // robot1 can't be used anymore because name was moved
    // Uncomment to see the error:
    // println!("Robot 1: {:?}", robot1);

    // But we can still access Copy fields
    println!("Robot 1's battery level: {}", robot1.battery_level);

    println!();
}

// ============================================================================
// Methods
// ============================================================================

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

impl Robot {
    fn new(id: u32, name: String) -> Self {
        Robot {
            id,
            name,
            location: Point::new(0.0, 0.0),
            battery_level: 100,
        }
    }

    fn move_to(&mut self, x: f64, y: f64) {
        self.location = Point::new(x, y);
        self.consume_battery(1);
    }

    fn consume_battery(&mut self, amount: u8) {
        self.battery_level = self.battery_level.saturating_sub(amount);
    }

    fn status(&self) -> String {
        format!(
            "Robot '{}' (ID: {}) at ({:.1}, {:.1}) - Battery: {}%",
            self.name, self.id, self.location.x, self.location.y, self.battery_level
        )
    }

    fn needs_charging(&self) -> bool {
        self.battery_level < 20
    }
}

fn method_usage() {
    println!("--- Method Usage ---");

    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);

    println!("P1: {:?}", p1);
    println!("P2: {:?}", p2);
    println!("Distance from origin: {:.2}", p2.distance_from_origin());
    println!("Distance between points: {:.2}", p1.distance_to(&p2));

    let mut p3 = Point::new(10.0, 10.0);
    println!("P3 before: {:?}", p3);
    p3.translate(5.0, -3.0);
    println!("P3 after translate: {:?}", p3);

    println!();

    let mut robot = Robot::new(100, String::from("Worker-100"));
    println!("{}", robot.status());

    robot.move_to(50.0, 30.0);
    println!("{}", robot.status());

    // Drain battery
    for _ in 0..20 {
        robot.consume_battery(5);
    }
    println!("{}", robot.status());

    if robot.needs_charging() {
        println!("⚠️  Robot needs charging!");
    }

    println!();
}
