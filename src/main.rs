use regex::Regex;
use std::{env, fs};

fn tokenize(input: &str) -> Vec<String> {
  let re = Regex::new(r"\S+|\s+").unwrap();
  re.find_iter(input).map(|m| m.as_str().to_string()).collect()
}

fn renumber_tests(file_name: &str) {
  let content = fs::read_to_string(file_name).expect("Failed to read input file");
  let tokens = tokenize(&content);
  let mut state = 0;
  let mut index = 1;
  for token in &tokens {
    if state != 4 {
      print!("{}", token);
    }
    match (state, token.as_str()) {
      (0, "#[test]") => state = 1,
      (0, _) => state = 0,
      (1, "\n") => state = 2,
      (1, _) => state = 0,
      (2, "fn") => state = 3,
      (2, _) => state = 0,
      (3, " ") => state = 4,
      (3, _) => state = 0,
      (4, _) => {
        print!("_{:04}()", index);
        index += 1;
        state = 0;
      }
      _ => {}
    }
  }
}

fn main() {
  let args = env::args().collect::<Vec<String>>();
  if args.len() != 2 {
    println!("Invalid arguments");
  } else {
    renumber_tests(&args[1]);
  }
}
