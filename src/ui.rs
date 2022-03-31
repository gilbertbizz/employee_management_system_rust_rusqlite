use std::io;
use std::io::Write;
use std::process::exit;

use rusqlite::Connection;

use crate::db::{db_init, add_employee, all_employees, update_employee, delete_employee };
use crate::employee::Employee;
use crate::config::DB_PATH;

// Application Header
pub fn app_header() {
  println!("\t\t\tEmployee Management System");
  println!("\t\t\t\tForm Ghana Limited");
  println!("\t\t=================================\n");

}
// This is the entry point of the app which displays the menus to be selected
pub fn main_menu() {
  menu();
  choice();
}

fn menu() {
   println!("\t\tOptions:\n");
  println!("\t\t1. Add Employee");
  println!("\t\t2. View all Employees");
  println!("\t\t3. Update Employee");
  println!("\t\t4. Delete Employee");
  println!("\t\t5. Exit");
}

fn choice() {
   let conn = match db_init(DB_PATH) {
    Ok(conn) => conn,
    Err(e) => {
      println!("Failed to initialized the database: {}", e);
      exit(1);
    }
  };
     
  loop { 
    print!("\n\t\tWhich option do you want to select: ");  
    let mut choice_str = String::new();
    io::stdout()
      .flush()
      .unwrap(); 
    
    io::stdin()
      .read_line(&mut choice_str)
      .ok()
      .expect("Failed to read the choice selected");

    match choice_str.trim() {
      "1" => add_employee_handler(&conn),
      "2" => view_all_employees(&conn),
      "3" =>  update_employee_handler(&conn),
      "4" => delete_employee_handler(&conn),
      "5" => exit_app(),
      _ => println!("No number entered"),
    }
  }
}

// To perform another operation if you are done with the current operation
fn perform_another_operation() {
  print!("Do you want to perform another operation? [Y/n]: ");
  
  let mut another_operation = String::new();

  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut another_operation).ok().expect("Failed to read operation selected");

  match another_operation.trim() {
    "Y" | "y" => {main_menu();}, 
    "N" | "n" => {exit_app();},
    _ => println!("Type y or Y for another operation or N or n to quit the app"),
  }
}


fn add_employee_handler(conn: &Connection) {
  print!("{esc}c", esc = 27 as char); // clear the console
  
  loop {
    app_header();

    println!("Add Employee: ");

   let mut first_name = String::new();
   print!("First Name: ");
   io::stdout().flush().unwrap();
   io::stdin().read_line(&mut first_name).ok().expect("Failed to read first name");

    let mut last_name = String::new();
    print!("Last Name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut last_name).ok().expect("Failed to read last name");

    let mut dob = String::new();
    print!("Date of birth: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut dob).ok().expect("Failed to read date of birth");

    let employee = Employee::new(first_name, last_name, dob); // employee object captured from the user input
      
    add_employee(conn, employee).expect("Failed to add the employee");

    perform_another_operation();

    break;
  }
}


fn view_all_employees(conn: &Connection) {
   print!("{esc}c", esc = 27 as char);
   app_header();
   println!("ALL EMPLOYEES:\n");

   match all_employees(&conn) {
     Ok(results) => display_employees(results),
     Err(e) => {
       println!("An error occurred while listing all employees: \"{}\".", e);
       exit(1);
     }
   }

  perform_another_operation();
}

fn display_employees(employees: Vec<Employee>) {
  for emp in employees.iter() {
    println!("- \tfirstname: \t\t{}\tlastname: \t\t{}\tdate of birth: \t{}", emp.first_name, emp.last_name, emp.date_of_birth);
  }
}

fn update_employee_handler(conn: &Connection) {
   print!("{esc}c", esc = 27 as char);

  println!("UPDATE EMPLOYEE");
  let mut name = String::new();
  let mut first_name = String::new();
  let mut last_name = String::new();
  let mut dob = String::new();

  print!("Select employee to be updated with the first name: ");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut name).ok().expect("Failed to read name");

  print!("First Name: ");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut first_name).ok().expect("Failed to read first name");

  print!("Last Name: ");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut last_name).ok().expect("Failed to read last name");

  print!("Date of birth: ");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut dob).ok().expect("Failed to read date of birth");

  let emp_update = Employee::new(first_name, last_name, dob);

  update_employee(conn, name, emp_update).expect("Failed to update the employee");
  

  perform_another_operation();
}

fn delete_employee_handler(conn: &Connection) {
   print!("{esc}c", esc = 27 as char);

  println!("DELETE EMPLOYEE");

  print!("Select employee to be deleted: ");
  let mut name = String::new();

  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut name).ok().expect("Failed to read name");

  delete_employee(conn, name).expect("Failed to delete the employee");

  perform_another_operation();
}


fn exit_app() {
  print!("{esc}c", esc = 27 as char);
  std::process::exit(256);
}