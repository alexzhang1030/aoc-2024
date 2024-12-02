/// Red-Nosed Reports
/// ```
/// 7 6 4 2 1
/// 1 2 7 8 9
/// 9 7 6 2 1
/// 1 3 2 4 5
/// 8 6 4 4 1
/// 1 3 6 7 9
/// ```
///
/// Rules:
/// - Each line is a list of numbers, separated by spaces
/// - Ensure numbers are all increasing or all decreasing
/// - Ensure any two adjacent levels differ by at least one and at most three.
/// - Calculate the number of valid lines
///
/// Example above:
/// - `7 6 4 2 1` Valid, all decreasing and differ by at most 3
/// - `1 2 7 8 9` Invalid, `2 -> 7` differ by 5
/// - `9 7 6 2 1` Invalid, `6 -> 2` differ by 4
/// - `1 3 2 4 5` Invalid, `1 -> 3 -> 2` is increasing then decreasing
/// - `8 6 4 4 1` Invalid, `4 -> 4` is either nor increasing nor decreasing
/// - `1 3 6 7 9` Valid, all increasing and differ by at most 3
///
/// Answer: 2, only two lines are valid
#[allow(dead_code)]
fn day02_part1() -> usize {
  let list = from_str();
  list.iter().filter(|numbers| is_valid(numbers)).count()
}

/// Part two
/// All part one rules are still applicable
/// Additionally:
/// - If removes one number from the list, the list will be valid, then that this list is valid
///
/// Example above:
/// - `7 6 4 2 1` Valid, all decreasing and differ by at most 3
/// - `1 2 7 8 9` Invalid, `2 -> 7` differ by 5
/// - `9 7 6 2 1` Invalid, `6 -> 2` differ by 4
/// - `1 3 2 4 5` Valid, removes `2` then it's valid
/// - `8 6 4 4 1` Valid, removes `4` then it's valid
/// - `1 3 6 7 9` Valid, all increasing and differ by at most 3
///
/// Answer: 2, only two lines are valid
#[allow(dead_code)]
fn day02_part2() -> usize {
  let list = from_str();
  list.iter().filter(|numbers| is_valid_part2(numbers)).count()
}

fn from_str() -> Vec<Vec<i32>> {
  let text = include_str!("./input.txt");
  text
    .lines()
    .map(|line| {
      line
        .split_ascii_whitespace()
        .map(|n| n.parse::<i32>().expect("Faced invalid number"))
        .collect::<Vec<i32>>()
    })
    .collect()
}

fn is_valid(numbers: &[i32]) -> bool {
  // TODO: Smarter way
  let mut increasing = true;

  for i in 1..numbers.len() {
    if i == 0 {
      continue;
    }
    let num = numbers[i];
    let previous_num = numbers[i - 1];
    // 1. ensure numbers are all increasing or all decreasing
    if num == previous_num {
      return false;
    }
    if i == 1 {
      // 2.0, let's determine it's direction
      increasing = num > previous_num;
    } else {
      // 2.1 ensure direction is consistent
      if (num > previous_num) != increasing {
        return false;
      }
    }
    // 3. Ensure any two adjacent levels differ by at least one and at most three.
    let differ = if increasing { num - previous_num } else { previous_num - num };
    if !(1..=3).contains(&differ) {
      return false;
    }
  }

  true
}

fn is_valid_part2(numbers: &[i32]) -> bool {
  let result = is_valid(numbers);
  if result {
    return true;
  }
  // Remove one number from the list
  // and check if the list is valid
  for i in 0..numbers.len() {
    let mut new_numbers = numbers.to_vec();
    new_numbers.remove(i);
    if is_valid(&new_numbers) {
      return true;
    }
  }
  false
}

#[cfg(test)]
mod test {
  use super::{day02_part1, day02_part2};

  #[test]
  fn test_day02_part1() {
    let res = day02_part1();
    assert_eq!(res, 472);
  }

  #[test]
  fn test_day02_part2() {
    let res = day02_part2();
    assert_eq!(res, 520);
  }
}
