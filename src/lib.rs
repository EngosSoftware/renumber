use std::fmt::Write;

/// Returns tokenized input string.
pub fn tokenize(input: String) -> Vec<String> {
  /// Tokenizer states.
  enum State {
    Start,
    Token,
    Whitespace,
  }
  let mut tokens = vec![];
  let mut buffer = String::new();
  let mut state = State::Start;
  for ch in input.chars() {
    match state {
      State::Start => {
        if ch.is_whitespace() {
          state = State::Whitespace;
        } else {
          state = State::Token;
        }
        buffer.push(ch);
      }
      State::Token => {
        if ch.is_whitespace() {
          state = State::Whitespace;
          tokens.push(buffer.clone());
          buffer = ch.to_string();
        } else {
          buffer.push(ch);
        }
      }
      State::Whitespace => {
        if ch.is_whitespace() {
          buffer.push(ch);
        } else {
          state = State::Token;
          tokens.push(buffer.clone());
          buffer = ch.to_string();
        }
      }
    }
  }
  tokens.push(buffer);
  tokens
}

/// Renumbers tests or benchmarks in specified input string.
pub fn renumber(input: String, mut index: usize) -> String {
  /// Renumbering states.
  #[derive(Clone, Copy, PartialEq, Eq)]
  enum State {
    Start,
    Annotation,
    BeforeFn,
    AfterFn,
    FnName,
  }
  let mut output = String::new();
  let tokens = tokenize(input);
  let mut state = State::Start;
  for token in &tokens {
    if state != State::FnName {
      let _ = write!(&mut output, "{token}");
    }
    match (state, token.as_str()) {
      (State::Start, "#[test]") => state = State::Annotation,
      (State::Start, "#[bench]") => state = State::Annotation,
      (State::Start, _) => state = State::Start,
      (State::Annotation, "\n") => state = State::BeforeFn,
      (State::Annotation, _) => state = State::Start,
      (State::BeforeFn, "fn") => state = State::AfterFn,
      (State::BeforeFn, _) => state = State::Start,
      (State::AfterFn, " ") => state = State::FnName,
      (State::AfterFn, _) => state = State::Start,
      (State::FnName, name) => {
        if let Some(suffix) = split_fn_token(name) {
          let _ = write!(output, "_{index:04}({suffix}");
          index += 1;
        } else {
          let _ = write!(output, "{name}");
        }
        state = State::Start;
      }
    }
  }
  output
}

/// Returns the suffix of the function token after the first `(`,
/// if the name begins with `_`.
fn split_fn_token(name: &str) -> Option<String> {
  if name.starts_with("_") {
    name.split_once("(").map(|(_, suffix)| suffix.to_string())
  } else {
    None
  }
}
