// Define the Square struct
pub struct Square {
    pub side: f64,
}

impl Square {
    // Constructor to create a new Square
    pub fn new(side: f64) -> Self {
        Square { side }
    }
}

// Define the Triangle struct
pub struct Triangle {
    pub side1: f64,
    pub side2: f64,
    pub side3: f64,
}

impl Triangle {
    // Constructor to create a new Triangle
    pub fn new(side1: f64, side2: f64, side3: f64) -> Self {
        Triangle {
            side1,
            side2,
            side3,
        }
    }
}

// Define the Circle struct
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    // Constructor to create a new Circle
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

// Example usage of the structs
fn main() {
    let square = Square::new(4.0);
    let triangle = Triangle::new(3.0, 4.0, 5.0);
    let circle = Circle::new(2.5);

    println!("Square side length: {}", square.side);
    println!(
        "Triangle sides: {}, {}, {}",
        triangle.side1, triangle.side2, triangle.side3
    );
    println!("Circle radius: {}", circle.radius);
}

