use std::process::Command;
use std::env;

fn main() {
  Command::new("make").args(&["-f","makefile.cargo"]).status().unwrap();

  let out_dir = env::var("OUT_DIR").unwrap();
  let v8_target = "x64.debug";

  println!("cargo:rustc-flags=-L {}/{}", out_dir, v8_target);
}
