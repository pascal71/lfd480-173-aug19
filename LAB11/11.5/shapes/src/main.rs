use std::f64::consts::PI;

// Define the Shape trait
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn scale(&mut self, factor: f64);
}

// Define the Square struct
pub struct Square {
    pub side: f64,
}

impl Square {
    pub fn new(side: f64) -> Self {
        Square { side }
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }

    fn scale(&mut self, factor: f64) {
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
    pub fn new(side1: f64, side2: f64, side3: f64) -> Self {
        Triangle {
            side1,
            side2,
            side3,
        }
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        let s = self.perimeter() / 2.0;
        (s * (s - self.side1) * (s - self.side2) * (s - self.side3)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        self.side1 + self.side2 + self.side3
    }

    fn scale(&mut self, factor: f64) {
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
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }

    fn scale(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

// Generic function to calculate the area of a Shape
fn area<T: Shape>(shape: &T) -> f64 {
    shape.area()
}

// Generic function to calculate the perimeter of a Shape
fn perimeter<T: Shape>(shape: &T) -> f64 {
    shape.perimeter()
}

// Generic function to scale a Shape
fn scale<T: Shape>(shape: &mut T, factor: f64) {
    shape.scale(factor);
}

fn main() {
    let mut square = Square::new(4.0);
    let mut triangle = Triangle::new(3.0, 4.0, 5.0);
    let mut circle = Circle::new(2.0);

    // Testing area function
    println!("Square area: {}", area(&square));
    println!("Triangle area: {}", area(&triangle));
    println!("Circle area: {}", area(&circle));

    // Testing perimeter function
    println!("Square perimeter: {}", perimeter(&square));
    println!("Triangle perimeter: {}", perimeter(&triangle));
    println!("Circle perimeter: {}", perimeter(&circle));

    // Testing scale function
    scale(&mut square, 2.0);
    scale(&mut triangle, 2.0);
    scale(&mut circle, 2.0);

    println!("\nAfter scaling by 2.0:");

    // Testing area function after scaling
    println!("Square area: {}", area(&square));
    println!("Triangle area: {}", area(&triangle));
    println!("Circle area: {}", area(&circle));

    // Testing perimeter function after scaling
    println!("Square perimeter: {}", perimeter(&square));
    println!("Triangle perimeter: {}", perimeter(&triangle));
    println!("Circle perimeter: {}", perimeter(&circle));
}
