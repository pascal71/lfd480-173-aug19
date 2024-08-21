use std::fmt;

// Define the Celsius type
pub struct Celsius(pub f64);

// Define the Fahrenheit type
pub struct Fahrenheit(pub f64);

// Implement Display for Celsius
impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} °C", self.0)
    }
}

// Implement Display for Fahrenheit
impl fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} °F", self.0)
    }
}

// Implement conversion from Celsius to Fahrenheit
impl Celsius {
    pub fn to_fahrenheit(&self) -> Fahrenheit {
        Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
    }
}

// Implement conversion from Fahrenheit to Celsius
impl Fahrenheit {
    pub fn to_celsius(&self) -> Celsius {
        Celsius((self.0 - 32.0) * 5.0 / 9.0)
    }
}

fn main() {
    // Example conversion from Celsius to Fahrenheit
    let temp_c = Celsius(25.0);
    let temp_f = temp_c.to_fahrenheit();
    println!("{} is equivalent to {}", temp_c, temp_f);

    // Example conversion from Fahrenheit to Celsius
    let temp_f = Fahrenheit(77.0);
    let temp_c = temp_f.to_celsius();
    println!("{} is equivalent to {}", temp_f, temp_c);
}

