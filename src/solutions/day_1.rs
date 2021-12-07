pub fn puzzle_1(input: String) -> i32 {
  let numbers = convert_to_numbers(input);
  let mut last_number = std::i32::MIN;
  let mut counter = -1;
  for current_number in numbers {
    if current_number > last_number {
      counter += 1;
    }
    last_number = current_number
  }
  counter
}

fn convert_to_numbers(input: String) -> Vec<i32> {
  let splits = input.split("\n");
  let mut vec: Vec<i32> = Vec::new();
  for split in splits {
    let number_string = String::from(split);
    vec.push(number_string.parse::<i32>().unwrap());
  }
  vec
}
