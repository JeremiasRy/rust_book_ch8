use std::{io, collections::HashMap, str::FromStr};

#[derive(PartialEq, Debug)]
enum Department {
    Sales,
    Engineering
}
#[derive(PartialEq, Debug)]
enum Action {
    Add,
    View
}

impl FromStr for Action {
    type Err = String;

    fn from_str(input: &str) -> Result<Action, Self::Err> {
        match input {
            "add" | "Add" => Ok(Action::Add),
            "view" | "View" => Ok(Action::View),
            _ => Err(String::from("Invalid action")),
        }
    }
}

impl FromStr for Department {
    type Err = String;

    fn from_str(input: &str) -> Result<Department, Self::Err> {
        match input {
            "sales" | "Sales" => Ok(Department::Sales),
            "engineering" | "Engineering" => Ok(Department::Engineering),
            _ => Err(String::from("Invalid department")),
        }
    }
}
struct EmpDb {
    employees: HashMap<String, Department>,
}

impl EmpDb {
    fn get_employees_from_department(&self, dep: &Department) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        
        for (k, v) in &self.employees {
            if v == dep {
                result.push(String::from(k));
            }
        }
        result.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        result
    }
    fn add_employee_to_department(&mut self, dep: Department, empl:String) {
        self.employees.insert(empl, dep);
    }
}

fn main() {
    let mut employee_db = EmpDb {
        employees: HashMap::new()
    };

    loop {
        println!("Choose action (write add, or view)");
        let action: Action = loop {
            match Action::from_str(get_input().as_str()) {
                Ok(action) => break action,
                Err(msg) => {
                    println!("{msg}\nTry again:");
                    continue;
                }
            }
        };
        println!("You chose: {:?}", action);

        println!("Choose department: (write sales or engineering)");

        let department:Department = loop {
            match Department::from_str(get_input().as_str()) {
                Ok(dep) => break dep,
                Err(msg) => {
                    println!("{msg} \nTry again:");
                    continue;
                },
            }
        };

        println!("You chose: {:?}", department);

        if action == Action::View {
            for employee in employee_db.get_employees_from_department(&department) {
                println!("{employee}");
            }
            continue;
        }

        println!("Who do you want to add?: ");
        let employee = get_input();

        employee_db.add_employee_to_department(department, employee);
    }
    
    
}

fn get_input() -> String {
    let user_input = loop {
        let mut result = String::new();

        io::stdin()
            .read_line(&mut result)
            .expect("Something went wrong");

        let result = match result
            .trim()
            .parse() {
                Ok(string) => string,
                Err(_) => {
                    println!("Invalid input");
                    continue;
                }
            };
            break result;
        };
    user_input
}
