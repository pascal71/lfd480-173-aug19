use std::fmt;
use std::ops::Mul;

#[derive(Debug, PartialEq)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    // Constructor to create a new Vector3D
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    // Method to calculate the Euclidean distance between two vectors
    pub fn distance(&self, other: &Vector3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl fmt::Display for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.1}, {:.1}, {:.1})", self.x, self.y, self.z)
    }
}

impl Mul for Vector3D {
    type Output = f64;

    // Overloading the * operator for dot product
    fn mul(self, other: Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    // Overloading the * operator for scalar multiplication
    fn mul(self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let vec = Vector3D::new(3.0, 1.5, -2.0);
        assert_eq!(format!("{}", vec), "(3.0, 1.5, -2.0)");
    }

    #[test]
    fn test_distance() {
        let vec1 = Vector3D::new(1.0, 1.0, 2.0);
        let vec2 = Vector3D::new(2.0, 1.0, 2.0);
        assert!((vec1.distance(&vec2) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_dot_product() {
        let vec1 = Vector3D::new(1.0, 2.0, 3.0);
        let vec2 = Vector3D::new(4.0, -5.0, 6.0);
        assert_eq!(vec1 * vec2, 12.0);
    }

    #[test]
    fn test_scalar_multiplication() {
        let vec = Vector3D::new(1.0, -2.0, 3.0);
        let result = vec * 3.0;
        assert_eq!(result, Vector3D::new(3.0, -6.0, 9.0));
    }
}
