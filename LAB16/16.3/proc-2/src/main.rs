use std::process::Command;

fn main() {
    let output = Command::new("ls")
        .output()
        .expect("Failed to execute command");

    let lines = String::from_utf8_lossy(&output.stdout);
    lines
        .split('\n')
        .filter(|line| line.contains(".rs"))
        .for_each(|line| println!("{}", line));
}
