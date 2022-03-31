mod db;
mod employee;
mod config;
mod ui;

use ui::{app_header, main_menu};



fn main(){
  print!("{esc}c", esc = 27 as char);
  app_header();
  main_menu();
}