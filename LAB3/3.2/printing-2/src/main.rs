// printing-2
fn main() {
    let fname = "Linus";
    let lname = "Torvalds";
    // Add printing of firstname lastname and lastname, firstname in two ways
    // 1) Using {} placeholders
    // 2) Using positional placeholders {n}
    //

    // println!("Change me!")

    // Method-1; using pos. format parameters
    println!("{1} {0}", fname, lname);

    // Method-2; changing input parameter order
    println!("{} {}", lname, fname);
}
