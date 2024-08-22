// Define the Faculty struct
pub struct Faculty {
    curr: u64,
    fact: u64,
}

impl Faculty {
    // Create a new instance of Faculty iterator
    pub fn new() -> Faculty {
        Faculty { curr: 1, fact: 1 }
    }
}

impl Iterator for Faculty {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // Calculate the next factorial number
        let current_fact = self.fact;
        self.fact *= self.curr;
        self.curr += 1;
        Some(current_fact)
    }
}

fn main() {
    // Testing Faculty iterator
    let mut faculty = Faculty::new();
    println!("First 10 Faculty numbers:");
    for i in 1..=10 {
        if let Some(v) = faculty.next() {
            println!("Faculty {} = {}", i, v);
        }
    }
}
