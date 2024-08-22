// Define the City struct
struct City {
    name: String,
    province: String,
    population: u64,
}

fn main() {
    // Create a list of cities using a vector
    let mut cities = vec![
        City {
            name: "Bergen".to_string(),
            province: "Limburg".to_string(),
            population: 5105,
        },
        City {
            name: "Nijmegen".to_string(),
            province: "Gelderland".to_string(),
            population: 81002,
        },
        City {
            name: "Maastricht".to_string(),
            province: "Limburg".to_string(),
            population: 63201,
        },
        City {
            name: "Amsterdam".to_string(),
            province: "Noordholland".to_string(),
            population: 982102,
        },
        City {
            name: "Rotterdam".to_string(),
            province: "Zuidholland".to_string(),
            population: 1030101,
        },
    ];

    // Sort cities by province in ascending order using a closure
    cities.sort_by(|a, b| a.province.cmp(&b.province));

    // Print the sorted list by province
    println!("Cities sorted by province in ascending order:");
    for city in &cities {
        println!("{} - {} - {}", city.name, city.province, city.population);
    }

    println!(); // Adding a blank line for separation

    // Sort cities by population in descending order using a closure
    cities.sort_by(|a, b| b.population.cmp(&a.population));

    // Print the sorted list by population
    println!("Cities sorted by population in descending order:");
    for city in &cities {
        println!("{} - {} - {}", city.name, city.province, city.population);
    }
}
