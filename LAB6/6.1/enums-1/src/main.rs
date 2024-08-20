// Step 1: Define the TrafficLight enum
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// Step 2: Implement the print_light function
fn print_light(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("The light is Red. Stop!"),
        TrafficLight::Yellow => println!("The light is Yellow. Get ready!"),
        TrafficLight::Green => println!("The light is Green. Go!"),
    }
}

// Step 3: Test in the main function
fn main() {
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    print_light(red_light);
    print_light(yellow_light);
    print_light(green_light);
}
