// Step 1: Define the CoffeeSize and MilkOption enums
enum CoffeeSize {
    Small,
    Medium,
    Large,
}

enum MilkOption {
    None,
    Regular,
    Soy,
}

// Define a struct to hold the coffee order
struct CoffeeOrder {
    size: CoffeeSize,
    milk: MilkOption,
}

fn main() {
    // Step 2: Create a sample order
    let my_order = CoffeeOrder {
        size: CoffeeSize::Medium,
        milk: MilkOption::Soy,
    };

    // Step 3: Describe the order using a match expression
    describe_order(my_order);
}

// Function to describe the order
fn describe_order(order: CoffeeOrder) {
    let size = match order.size {
        CoffeeSize::Small => "Small",
        CoffeeSize::Medium => "Medium",
        CoffeeSize::Large => "Large",
    };

    let milk = match order.milk {
        MilkOption::None => "no milk",
        MilkOption::Regular => "regular milk",
        MilkOption::Soy => "soy milk",
    };

    println!("You've ordered a {} coffee with {}.", size, milk);
}

