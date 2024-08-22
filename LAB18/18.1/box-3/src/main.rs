// Define a trait with multiple methods
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// Implement the trait for a struct
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

fn main() {
    // Using Box to store different types that implement the Shape trait
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle {
            width: 4.0,
            height: 5.0,
        }),
    ];

    for shape in shapes.iter() {
        println!("Area: {}, Perimeter: {}", shape.area(), shape.perimeter());
    }
}
