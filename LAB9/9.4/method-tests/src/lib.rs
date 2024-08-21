pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to calculate the area of the rectangle
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to calculate the perimeter of the rectangle
    pub fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    // Method to determine if one rectangle can completely contain another
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_basic() {
        let rect = Rectangle {
            width: 4,
            height: 5,
        };
        assert_eq!(rect.area(), 20);

        let rect = Rectangle {
            width: 10,
            height: 10,
        };
        assert_eq!(rect.area(), 100);
    }

    #[test]
    fn test_perimeter_basic() {
        let rect = Rectangle {
            width: 4,
            height: 5,
        };
        assert_eq!(rect.perimeter(), 18);

        let rect = Rectangle {
            width: 10,
            height: 10,
        };
        assert_eq!(rect.perimeter(), 40);
    }

    #[test]
    fn test_area_table_driven() {
        let test_cases = vec![
            (
                Rectangle {
                    width: 1,
                    height: 1,
                },
                1,
            ),
            (
                Rectangle {
                    width: 5,
                    height: 10,
                },
                50,
            ),
            (
                Rectangle {
                    width: 7,
                    height: 3,
                },
                21,
            ),
            (
                Rectangle {
                    width: 0,
                    height: 10,
                },
                0,
            ),
            (
                Rectangle {
                    width: 6,
                    height: 6,
                },
                36,
            ),
        ];

        for (rect, expected_area) in test_cases {
            assert_eq!(rect.area(), expected_area);
        }
    }

    #[test]
    fn test_perimeter_table_driven() {
        let test_cases = vec![
            (
                Rectangle {
                    width: 1,
                    height: 1,
                },
                4,
            ),
            (
                Rectangle {
                    width: 5,
                    height: 10,
                },
                30,
            ),
            (
                Rectangle {
                    width: 7,
                    height: 3,
                },
                20,
            ),
            (
                Rectangle {
                    width: 0,
                    height: 10,
                },
                20,
            ),
            (
                Rectangle {
                    width: 6,
                    height: 6,
                },
                24,
            ),
        ];

        for (rect, expected_perimeter) in test_cases {
            assert_eq!(rect.perimeter(), expected_perimeter);
        }
    }

    #[test]
    fn test_can_hold() {
        let rect1 = Rectangle {
            width: 8,
            height: 7,
        };
        let rect2 = Rectangle {
            width: 5,
            height: 4,
        };
        let rect3 = Rectangle {
            width: 8,
            height: 7,
        };
        let rect4 = Rectangle {
            width: 9,
            height: 10,
        };

        assert!(rect1.can_hold(&rect2));
        assert!(rect1.can_hold(&rect3)); // identical dimensions
        assert!(!rect1.can_hold(&rect4));
        assert!(rect4.can_hold(&rect1));
    }

    #[test]
    fn test_can_hold_edge_cases() {
        let rect1 = Rectangle {
            width: 4,
            height: 4,
        };
        let rect2 = Rectangle {
            width: 4,
            height: 3,
        };
        let rect3 = Rectangle {
            width: 3,
            height: 4,
        };
        let rect4 = Rectangle {
            width: 4,
            height: 4,
        }; // identical

        assert!(rect1.can_hold(&rect2)); // width is the same, height is greater
        assert!(rect1.can_hold(&rect3)); // height is the same, width is greater
        assert!(rect1.can_hold(&rect4)); // identical rectangles
        assert!(!rect2.can_hold(&rect1)); // rect2 is smaller
        assert!(!rect3.can_hold(&rect1)); // rect3 is smaller
    }
}
