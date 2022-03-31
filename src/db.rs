use std::path::Path;
use rusqlite::{Connection, Result, params};

use crate::employee::Employee;

pub fn db_init(db_path: &str) -> Result<Connection> {

  if Path::new(db_path).exists() {
   return Ok(create_connection(db_path)?);
  }
  
  let conn = create_connection(db_path)?;
  
  conn.execute("CREATE TABLE IF NOT EXISTS employee (
    id INTEGER PRIMARY KEY AUTOINCREMENT, 
    first_name TEXT NOT NULL, 
    last_name TEXT NOT NULL, 
    date_of_birth TEXT NOT NULL)", params!([]))?;


  return Ok(conn);
}

fn create_connection(db_path: &str) -> Result<Connection> {
  return Connection::open(db_path);
}



pub fn all_employees(conn: &Connection) -> Result<Vec<Employee>> {
  
  let mut stmt = conn.prepare("SELECT first_name, last_name, date_of_birth FROM employee")?;

  let employee_iter = stmt.query_map([], |row| {
    Ok(Employee {
      first_name: row.get(0)?,
      last_name: row.get(1)?,
      date_of_birth: row.get(2)?,
    })
  })?;

  let mut employees = vec![];

  for emp in employee_iter {
    match emp {
      Ok(employee) => employees.push(employee),
      Err(e) => println!("Error while fetching some employee: \"{}\".", e),
    }
  }

  Ok(employees)
}

pub fn add_employee(conn: &Connection, employee: Employee) -> Result<()>{
  conn.execute("INSERT INTO employee(first_name, last_name, date_of_birth) VALUES(?1, ?2, ?3)", params![employee.first_name, employee.last_name, employee.date_of_birth])?;

  Ok(())
}

// pub fn search_employee(conn: &Connection, name: String) -> Result<()> {
//   let mut stmt = conn.prepare("SELECT * FROM employee WHERE first_name = ?1", params![name])?;
//   let emp_iter = stmt.query_map([], |row| {
//     Ok(Employee {
//       first_name: row.get(0)?,
//       last_name: row.get(1)?,
//       date_of_birth: row.get(2)?,
//     })
//   })?;

//   for emp in emp_iter {
//     match emp {
//       Ok(emp) => emp,
//       Err(e) => println!("Error searching: {}", e),
//     }
//   }

//   Ok(())
// }

pub fn update_employee(conn: &Connection, name: String, employee: Employee) -> Result<(), String>{
  let conn = conn.execute("UPDATE employee SET first_name = ?1, last_name = ?2, date_of_birth = ?3 WHERE first_name = ?4", params![employee.first_name, employee.last_name, employee.date_of_birth, name]);

  match conn {
    Ok(0) => Err(format!("No row found for name: {}.", name)),
    Err(e) => Err(format!("{}", e)),
    _ => Ok(()),
  }
}



pub fn delete_employee(conn: &Connection, name: String) -> Result<(), String> {
  let conn = conn.execute("DELETE FROM employee WHERE first_name = ?1", params![name]);

  match conn {
    Ok(0) => Err(format!("No row found for name: {}.", name)),
    Err(e) => Err(format!("{}", e)),
    _ => Ok(()),
  }
}

