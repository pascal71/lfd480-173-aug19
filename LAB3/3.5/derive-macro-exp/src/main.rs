#[derive(Debug)]
struct Course {
    name: String,
    lenght: u8,
    trainer: String,
}
fn main() {
    let my_course = Course {
        trainer: "Kooberstein".to_string(),
        lenght: 4,
        name: "Rusty Realms".to_string(),
    };
    println!("{:#?}", my_course)
}

