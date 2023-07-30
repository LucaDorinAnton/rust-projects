use std::collections::HashMap;
use std::io;

fn add_employee_to_department(
    company: &mut HashMap<String, Vec<String>>,
    department: String,
    employee: String,
) -> bool {
    let employee_vec = company.entry(department).or_insert(Vec::new());

    if !employee_vec.contains(&employee) {
        employee_vec.push(employee);
        true
    } else {
        false
    }
}

fn list_employees_in_department(
    company: &HashMap<String, Vec<String>>,
    department: &String,
) -> String {
    let maybe_dept = company.get(department);

    match maybe_dept {
        Some(v) => format!("Department {department} has employees: {}.", v.join(", ")),
        None => format!("Department {department} is not registered with the company."),
    }
}

fn list_all_departments(company: &HashMap<String, Vec<String>>) -> String {
    let mut text: Vec<String> = Vec::new();
    let departments: Vec<&str> = company.keys().map(String::as_str).collect();

    text.push(format!(
        "The company has {} departments: {}",
        departments.len(),
        departments.join(", ")
    ));

    for dept in departments {
        text.push(list_employees_in_department(company, &String::from(dept)));
    }

    text.join("\n")
}

fn help() -> String {
    String::from(
        "
    ? - print help
    a [e] [d] - add employee [e] to department [d]
    l [d] - list employees of department [d]. If [d] is ommitted, all departments will be listed
    e - exit the program",
    )
}

fn cli() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("HR Management CLI. Type '?' for help.");
    loop {
        let mut prompt = String::new();

        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to read line");

        let args: Vec<&str> = prompt.split_whitespace().collect();

        match args.get(0) {
            None => println!("Please provide a command"),
            Some(x) => match *x {
                "?" => println!("{}", &help()),
                "a" => {
                    if args.len() < 3 {
                        println!("Please specify a department and an employee.")
                    } else {
                        let result = add_employee_to_department(
                            &mut company,
                            String::from(args[2]),
                            String::from(args[1]),
                        );
                        match result {
                            true => println!("Operation successful."),
                            false => {
                                println!("Operaion failed. Employee already added to deparment")
                            }
                        }
                    }
                }
                "l" => match args.len() {
                    1 => println!("{}", &list_all_departments(&company)),
                    _ => println!(
                        "{}",
                        &list_employees_in_department(&company, &String::from(args[1]))
                    ),
                },
                "e" => {
                    println!("Exiting...");
                    break;
                }
                _ => println!("Unknown command. Please try again"),
            },
        }
    }
}

fn main() {
    /*
     *  let mut company: HashMap<String, Vec<String>> = HashMap::new();
     *
     *  add_employee_to_department(&mut company, String::from("Sales"), String::from("Amir"));
     *
     *  add_employee_to_department(&mut company, String::from("Engineering"), String::from("Maria"));
     *
     *  add_employee_to_department(&mut company, String::from("Sales"), String::from("Matthew"));
     *
     *  println!("{}", &list_employees_in_department(&company, &String::from("Sales")));
     *
     *  println!("{}", list_all_departments(&company));
     */

    cli();
}
