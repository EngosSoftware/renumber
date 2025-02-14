use renumber::renumber;

#[test]
fn _0001() {
  let input = r#"

#[test]
fn _a() {
  // some test
}

  "#;
  let expected = r#"

#[test]
fn _0001() {
  // some test
}

  "#;
  assert_eq!(expected, renumber(input.to_string(), 1));
}

#[test]
fn _0002() {
  let input = r#"

#[bench]
fn _a() {
  // some test
}

  "#;
  let expected = r#"

#[bench]
fn _0001() {
  // some test
}

  "#;
  assert_eq!(expected, renumber(input.to_string(), 1));
}

#[test]
fn _0003() {
  let input = r#"

#[test] a
fn _a() {
  // some test
}

  "#;
  assert_eq!(input, renumber(input.to_string(), 1));
}

#[test]
fn _0004() {
  let input = r#"

#[test]
gn _a() {
  // some test
}

  "#;
  assert_eq!(input, renumber(input.to_string(), 1));
}

#[test]
fn _0005() {
  let input = r#"

#[test]
fn    _a() {
  // some test
}

  "#;
  assert_eq!(input, renumber(input.to_string(), 1));
}

#[test]
fn _0006() {
  let input = r#"

#[test]
fn a() {
  // some test
}

  "#;
  assert_eq!(input, renumber(input.to_string(), 1));
}
