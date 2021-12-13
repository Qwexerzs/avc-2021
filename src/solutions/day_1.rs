pub fn puzzle_1(input: String) -> i32 {
  let mut last_number = std::i32::MIN;
  let mut counter = -1;

  for number_string in input.lines() {
    let current_number = number_string.parse::<i32>().unwrap();
    if current_number > last_number {
      counter += 1;
    }
    last_number = current_number
  }
  counter
}

pub fn puzzle_2(input: String) -> i32 {
  let mut previous_window_sum = std::i32::MIN;
  let mut counter = 0;
  let mut window = [0, 0, 0];
  let mut pre_load_counter = 0;
  let mut window_position_tracker = 0;

  for number_string in input.lines() {
    if pre_load_counter < window.len() {
      window[pre_load_counter] = number_string.parse::<i32>().unwrap();
      pre_load_counter += 1;
      continue;
    }

    window[window_position_tracker] = number_string.parse::<i32>().unwrap();

    let window_sum = window[(window_position_tracker + 1) % window.len()]
      + window[(window_position_tracker + 2) % window.len()]
      + window[window_position_tracker];
    if window_sum > previous_window_sum {
      counter += 1;
    }
    previous_window_sum = window_sum;

    window_position_tracker = (window_position_tracker + 1) % window.len();
  }
  counter
}
