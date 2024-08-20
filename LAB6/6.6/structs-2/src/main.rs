// Step 1: Define an Employee struct
struct Employee {
    first_name: String,
    last_name: String,
    job_title: String,
}

// Step 2: Define a Department struct
struct Department {
    department_name: String,
    employees: [Employee; 5], // Fixed list of 5 employees
}

fn main() {
    // Step 3: Instantiate the Employee structs
    let employee1 = Employee {
        first_name: String::from("Alice"),
        last_name: String::from("Smith"),
        job_title: String::from("Software Engineer"),
    };

    let employee2 = Employee {
        first_name: String::from("Bob"),
        last_name: String::from("Johnson"),
        job_title: String::from("Data Analyst"),
    };

    let employee3 = Employee {
        first_name: String::from("Charlie"),
        last_name: String::from("Williams"),
        job_title: String::from("Product Manager"),
    };

    // Additional employees to fill the fixed list
    let employee4 = Employee {
        first_name: String::from("David"),
        last_name: String::from("Brown"),
        job_title: String::from("HR Manager"),
    };

    let employee5 = Employee {
        first_name: String::from("Eve"),
        last_name: String::from("Davis"),
        job_title: String::from("Sales Representative"),
    };

    // Instantiate the Department struct
    let department = Department {
        department_name: String::from("Technology"),
        employees: [employee1, employee2, employee3, employee4, employee5],
    };

    // Step 4: Print the department info and the Employees in that department
    println!("Department: {}", department.department_name);
    println!("Employees:");

    for employee in department.employees.iter() {
        println!(
            "- {} {}, {}",
            employee.first_name, employee.last_name, employee.job_title
        );
    }
}

