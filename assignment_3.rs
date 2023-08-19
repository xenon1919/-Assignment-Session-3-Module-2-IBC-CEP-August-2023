use std::io;

#[derive(Debug)]
struct Employee {
    employee_id: String,
    employee_name: String,
    email: String,
    age: u32,
    phone_number: String,
}

impl Employee {
    fn new() -> Self {
        let mut input = String::new();
        println!("Enter Employee ID:");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let employee_id = input.trim().to_string();
        
        println!("Enter Employee Name:");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let employee_name = input.trim().to_string();
        
        println!("Enter Email:");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let email = input.trim().to_string();
        
        println!("Enter Age:");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let age: u32 = input.trim().parse().expect("Invalid age");

        println!("Enter Phone Number:");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let phone_number = input.trim().to_string();

        Employee {
            employee_id,
            employee_name,
            email,
            age,
            phone_number,
        }
    }
}

fn main() {
    let mut employees: Vec<Employee> = Vec::new();

    loop {
        println!("\nOptions:");
        println!("1. Add Employee");
        println!("2. Find Employee by ID");
        println!("3. Find Employees by Age");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => employees.push(Employee::new()),
            "2" => {
                println!("Enter Employee ID to search:");
                let employee_id = input_string();
                if let Some(employee) = employees.iter().find(|e| e.employee_id == employee_id) {
                    println!("Employee Found:\n{:?}", employee);
                } else {
                    println!("Employee not found.");
                }
            }
            "3" => {
                println!("Enter Age to search:");
                let age: u32 = input_string().parse().expect("Invalid input");
                let matching_employees: Vec<_> = employees.iter().filter(|e| e.age == age).collect();
                if matching_employees.is_empty() {
                    println!("No employees found with age {}.", age);
                } else {
                    println!("Employees with age {}:\n{:?}", age, matching_employees);
                }
            }
            "4" => break,
            _ => println!("Invalid choice. Please enter 1, 2, 3, or 4."),
        }
    }
}

fn input_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
