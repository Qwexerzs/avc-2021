pub fn puzzle_1(input: String) -> i32 {
  let splits = input.split("\n");
  let mut last_number = std::i32::MIN;
  let mut counter = -1;

  for split in splits {
    let number_string = String::from(split);
    let current_number = number_string.parse::<i32>().unwrap();
    if current_number > last_number {
      counter += 1;
    }
    last_number = current_number
  }
  counter
}
