use renumber::tokenize;

#[test]
fn _0001() {
  let input = r#"jumping jack flash"#.to_string();
  assert_eq!(vec!["jumping", " ", "jack", " ", "flash"], tokenize(input));
}

#[test]
fn _0002() {
  let input = r#" jumping jack flash"#.to_string();
  assert_eq!(vec![" ", "jumping", " ", "jack", " ", "flash"], tokenize(input));
}

#[test]
fn _0003() {
  let input = "jumping\n\n\rjack".to_string();
  assert_eq!(vec!["jumping", "\n\n\r", "jack"], tokenize(input));
}
