use std::env;

fn main() {
    // Collect command line arguments into a vector
    let args: Vec<String> = env::args().collect();
    println!("Command line arguments: {:?}", args);

    // Read the environment variable MY_ENV_VAR
    match env::var("MY_ENV_VAR") {
        Ok(val) => println!("MY_ENV_VAR: {:?}", val),
        Err(e) => println!("Couldn't read MY_ENV_VAR ({})", e),
    };

    // Handle optional arguments with defaults
    let mut timeout = 30; // Default timeout value
    for i in 0..args.len() {
        if args[i] == "-t" || args[i] == "--timeout" {
            if i + 1 < args.len() {
                timeout = args[i + 1].parse().unwrap_or(timeout);
            }
        }
    }
    println!("Timeout value: {} seconds", timeout);

    // Check for help flag
    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        print_help();
    }
}

fn print_help() {
    println!("Usage: env_and_args [OPTIONS]");
    println!("\nOptions:");
    println!("  -t, --timeout <TIMEOUT>  Sets the timeout value (default is 30 seconds)");
    println!("  -h, --help               Prints this help information");
    println!("\nEnvironment Variables:");
    println!("  MY_ENV_VAR  Overrides the command line argument for timeout if set.");
}
