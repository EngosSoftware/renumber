use renumber::renumber;
use std::process::exit;
use std::{env, fs};

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() == 2 {
    let file_name = &args[1];
    match fs::read_to_string(file_name) {
      Ok(input) => {
        let output = renumber(input, 1);
        fs::write(file_name, output).unwrap_or_else(|reason| {
          eprintln!("Failed to write file '{}' with reason: {}", file_name, reason);
          exit(1);
        });
      }
      Err(reason) => {
        eprintln!("Failed to read file '{}' with reason: {}", file_name, reason);
        exit(1);
      }
    };
  } else {
    eprintln!("Invalid argument, expected the input file name.");
    exit(1);
  }
}
