use std::process::Command;

fn main() {
    let output = Command::new("echo")
        .arg("Hello from the child process!")
        .output()
        .expect("Failed to execute command");

    println!(
        "Child process output: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}
