use lib::qeq_lib;

fn main() {
    match qeq_lib::solve(1.0, 3.0, 2.0) {
        Ok((sol1, sol2)) => {
            println!("sol1 = {}, sol2 = {}", sol1, sol2);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

