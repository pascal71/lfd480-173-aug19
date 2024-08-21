use std::f64::consts::PI;

// Define the Square struct
pub struct Square {
    pub side: f64,
}

impl Square {
    // Constructor to create a new Square
    pub fn new(side: f64) -> Self {
        Square { side }
    }

    // Method to calculate the perimeter of the square
    pub fn perimeter(&self) -> f64 {
        4.0 * self.side
    }

    // Method to calculate the area of the square
    pub fn area(&self) -> f64 {
        self.side * self.side
    }

    // Method to scale the square's size
    pub fn scale(&mut self, factor: f64) {
        self.side *= factor;
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

    // Method to calculate the perimeter of the triangle
    pub fn perimeter(&self) -> f64 {
        self.side1 + self.side2 + self.side3
    }

    // Method to calculate the area of the triangle using Heron's formula
    pub fn area(&self) -> f64 {
        let s = self.perimeter() / 2.0;
        (s * (s - self.side1) * (s - self.side2) * (s - self.side3)).sqrt()
    }

    // Method to scale the triangle's size
    pub fn scale(&mut self, factor: f64) {
        self.side1 *= factor;
        self.side2 *= factor;
        self.side3 *= factor;
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

    // Method to calculate the perimeter (circumference) of the circle
    pub fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }

    // Method to calculate the area of the circle
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    // Method to scale the circle's size
    pub fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

fn main() {
    // Square
    let mut square = Square::new(4.0);
    println!("Square before scaling:");
    println!("  Side: {}", square.side);
    println!("  Perimeter: {}", square.perimeter());
    println!("  Area: {}", square.area());

    square.scale(2.0);

    println!("Square after scaling by 2.0:");
    println!("  Side: {}", square.side);
    println!("  Perimeter: {}", square.perimeter());
    println!("  Area: {}", square.area());

    // Triangle
    let mut triangle = Triangle::new(3.0, 4.0, 5.0);
    println!("\nTriangle before scaling:");
    println!(
        "  Sides: {}, {}, {}",
        triangle.side1, triangle.side2, triangle.side3
    );
    println!("  Perimeter: {}", triangle.perimeter());
    println!("  Area: {}", triangle.area());

    triangle.scale(2.0);

    println!("Triangle after scaling by 2.0:");
    println!(
        "  Sides: {}, {}, {}",
        triangle.side1, triangle.side2, triangle.side3
    );
    println!("  Perimeter: {}", triangle.perimeter());
    println!("  Area: {}", triangle.area());

    // Circle
    let mut circle = Circle::new(2.0);
    println!("\nCircle before scaling:");
    println!("  Radius: {}", circle.radius);
    println!("  Perimeter: {}", circle.perimeter());
    println!("  Area: {}", circle.area());

    circle.scale(2.0);

    println!("Circle after scaling by 2.0:");
    println!("  Radius: {}", circle.radius);
    println!("  Perimeter: {}", circle.perimeter());
    println!("  Area: {}", circle.area());
}

