#[test]
fn one_should_be_equal_to_one() {
  assert_eq!(1, 1);
}

#[test]
fn one_should_not_be_equal_to_two() {
  assert_ne!(1, 2);
}

#[test]
fn first_name_should_be_shorter_than_the_second() {
  let first_name = "John";
  let second_name = "Alexander";
  assert!(first_name.cmp(&second_name).is_gt());
}
