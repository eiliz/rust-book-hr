// Using a hash map and vectors, create a text interface to allow a user
// to add employee names to a department in a company. For example, “Add
// Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve
// a list of all people in a department or all people in the company by
// department, sorted alphabetically.

use std::collections::HashMap;
use std::io;

// required trait for .lines()
use std::io::BufRead;

enum Command {
    Add { dept: String, name: String },
    List(String),
    All,
    Quit,
}

impl Command {
    fn from_input(input: &str) -> Option<Self> {
        let words: Vec<&str> = input.trim().split_whitespace().collect();

        match words.as_slice() {
            ["All"] => Some(Command::All),
            ["List", dept] => Some(Command::List(dept.to_string())),
            ["Quit"] => Some(Command::Quit),
            ["Add", name, "to", dept] => Some(Command::Add {
                dept: dept.to_string(),
                name: name.to_string(),
            }),
            _ => None,
        }
    }
}

fn main() {
    println!("Type 'Add <name> to <department>' to add an employee");
    println!("Type 'List <department>' to list the employees of a department");
    println!("Type 'All' to list all employees by department");
    println!("Type 'Quit' to quit");

    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let input = line.expect("Unable to read user input");

        match Command::from_input(&input) {
            Some(Command::Add { dept, name }) => employees.entry(dept).or_default().push(name),
            Some(Command::List(dept)) => match employees.get(&dept) {
                Some(names) => {
                    for name in names {
                        println!("{}: {}", dept, name);
                    }
                }
                None => println!("Department doesn't exist"),
            },
            Some(Command::All) => {
                for (dept, names) in &employees {
                    let mut names = names.clone();
                    names.sort();
                    for name in names {
                        println!("{}: {}", dept, name);
                    }
                }
            }
            Some(Command::Quit) => break,
            None => println!("Input error"),
        }
    }
}
