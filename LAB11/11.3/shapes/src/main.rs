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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_perimeter() {
        let square = Square::new(4.0);
        assert_eq!(square.perimeter(), 16.0);
    }

    #[test]
    fn test_square_area() {
        let square = Square::new(4.0);
        assert_eq!(square.area(), 16.0);
    }

    #[test]
    fn test_triangle_perimeter() {
        let triangle = Triangle::new(3.0, 4.0, 5.0);
        assert_eq!(triangle.perimeter(), 12.0);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle::new(3.0, 4.0, 5.0);
        assert!((triangle.area() - 6.0).abs() < 1e-6);
    }

    #[test]
    fn test_circle_perimeter() {
        let circle = Circle::new(2.0);
        assert!((circle.perimeter() - 2.0 * PI * 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_circle_area() {
        let circle = Circle::new(2.0);
        assert!((circle.area() - PI * 4.0).abs() < 1e-6);
    }
}

