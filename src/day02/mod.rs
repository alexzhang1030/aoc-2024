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
fn day02() -> usize {
  let text = include_str!("./input.txt");
  let list: Vec<Vec<i32>> = text
    .lines()
    .map(|line| {
      line
        .split_whitespace()
        .map(|n| n.parse::<i32>().expect("Faced invalid number"))
        .collect::<Vec<i32>>() // 收集每行的数字
    })
    .collect();

  list.iter().filter(|numbers| is_valid(numbers)).count()
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
    if i == 1 {
      if num == previous_num {
        return false;
      }
      increasing = num > previous_num;
    }
    let differ = if increasing { num - previous_num } else { previous_num - num };
    if !(1..=3).contains(&differ) {
      return false;
    }
  }

  true
}

#[cfg(test)]
mod test {
  use super::day02;

  #[test]
  fn test_day02() {
    let res = day02();
    assert_eq!(res, 472);
  }
}
