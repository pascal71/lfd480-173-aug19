// Step 1: Define an enum named Day
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn main() {
    // Step 2: Create a variable for each day of the week
    let monday = Day::Monday;
    let tuesday = Day::Tuesday;
    let wednesday = Day::Wednesday;
    let thursday = Day::Thursday;
    let friday = Day::Friday;
    let saturday = Day::Saturday;
    let sunday = Day::Sunday;

    // Step 3: Use a match expression to print a small piece of information for each day
    print_activity(monday);
    print_activity(tuesday);
    print_activity(wednesday);
    print_activity(thursday);
    print_activity(friday);
    print_activity(saturday);
    print_activity(sunday);
}

// A function that takes a Day enum and prints the corresponding activity
fn print_activity(day: Day) {
    match day {
        Day::Monday => println!("Monday is Rust practice day!"),
        Day::Tuesday => println!("Tuesday is gym day!"),
        Day::Wednesday => println!("Wednesday is study day!"),
        Day::Thursday => println!("Thursday is coding project day!"),
        Day::Friday => println!("Friday is movie night!"),
        Day::Saturday => println!("Saturday is relaxation day!"),
        Day::Sunday => println!("Sunday is family time!"),
    }
}

