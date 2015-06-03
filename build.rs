use std::process::Command;
use std::env;

fn main() {
  Command::new("make").status().unwrap();

  let out_dir = env::var("OUT_DIR").unwrap();

  println!("cargo:rustc-flags=-L {}", out_dir);
  println!("cargo:rustc-link-search=native={}", out_dir);
}
