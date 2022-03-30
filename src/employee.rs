use serde::{Serialize, Deserialize};

// Employee structure
#[derive(Serialize, Deserialize, Debug)]
  pub struct Employee {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
  }

impl Employee {
 pub fn new(first_name: String, last_name: String, dob: String) -> Self {
    Self {
      first_name: first_name,
      last_name: last_name,
      date_of_birth: dob,
    }
  }

  // fn birth_year(&self) -> &str {
  //   let dob_str = &self.date_of_birth;
  //   let split_dob = dob_str.as_str().split("-"); 
  //   let dob_vec: Vec<&str> = split_dob.collect();

  //   let year = dob_vec.get(2);

  //   year
  // }
}

