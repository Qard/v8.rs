#![feature(os)]
#![feature(io)]

use std::old_io::Command;
use std::os;

fn main() {
  Command::new("make").args(&["-f","makefile.cargo"]).status().unwrap();

  let out_dir = os::getenv("OUT_DIR").unwrap();
  let v8_target = "x64.debug";

  println!("cargo:rustc-flags=-L {}/{}", out_dir, v8_target);
}
