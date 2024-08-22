use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let mut child = Command::new("grep")
        .arg("hello")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    if let Some(ref mut stdin) = child.stdin {
        stdin
            .write_all("hello world\n".as_bytes())
            .expect("Failed to write to child stdin");
    }

    let output = child
        .wait_with_output()
        .expect("Failed to read child output");
    println!(
        "Child process filtered output: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}
