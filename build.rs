use std::old_io::Command;

fn main() {
  Command::new("make").args(&["-f","makefile.cargo"]).status().unwrap();
}
