use std::collections::HashMap;

/// https://adventofcode.com/2024/day/1
/// ```
/// 3 4
/// 4 3
/// 2 5
/// 1 3
/// 3 9
/// 3 3
/// ```
///
/// Rules:
/// - Pair up the smallest number in the left list with the smallest number in the right list
/// - then the second-smallest left number with the second-smallest right number, and so on
/// - Within each pair, figure out how far apart the two numbers are; you'll need to add up all of those distances.
///
/// Example above:
/// - 1, 3 -> 2
/// - 2, 3 -> 1
/// - 3, 3 -> 0
/// - 3, 4 -> 1
/// - 3, 5 -> 2
/// - 4, 9 -> 5
///
/// Answer: 2 + 1 + 0 + 1 + 2 + 5 = 11
#[allow(dead_code)]
fn day01_part1() -> Result<i32, String> {
  // Time complexity: O(n log n)
  // TODO: Optimize to O(n)
  let (mut left, mut right) = from_str()?;

  left.sort();
  right.sort();

  Ok(left.iter().zip(right.iter()).map(|(l, r)| (l - r).abs()).sum())
}

/// Part two
/// ```
/// 3 4
/// 4 3
/// 2 5
/// 1 3
/// 3 9
/// 3 3
/// ```
///
/// Rules:
/// - Calculate similarity score
/// - by adding up each number in the left list after multiplying it
/// - by the number of times that number appears in the right list.
/// - Answer is the sum of all the scores
///
/// Example above:
/// - 3: 3 * 3(appears 3 times) = 9
/// - 4: 4 * 1(appears 1 time) = 4
/// - 2: 2 * 0(does't appears) = 0
/// - 1: 1 * 0(does't appears) = 0
/// - 3: 3 * 3(appears 3 times) = 9
/// - 3: 3 * 3(appears 3 times) = 9
///
/// Answer: 9 + 4 + 0 + 0 + 9 + 9 = 31
#[allow(dead_code)]
fn day01_part2() -> Result<i32, String> {
  let (left, right) = from_str()?;

  let mut count_map: HashMap<i32, i32> = HashMap::new();
  for num in right {
    *count_map.entry(num).or_insert(0) += 1;
  }

  Ok(left.iter().filter_map(|&num| count_map.get(&num).map(|&count| count * num)).sum())
}

fn from_str() -> Result<(Vec<i32>, Vec<i32>), String> {
  let text = include_str!("./input.txt");

  let mut left = Vec::new();
  let mut right = Vec::new();

  for line in text.lines() {
    let mut parts = line.split_ascii_whitespace();
    let l = parts.next().ok_or_else(|| format!("Missing left part in line: {}", line))?;
    let r = parts.next().ok_or_else(|| format!("Missing right part in line: {}", line))?;
    left.push(l.parse::<i32>().map_err(|e| format!("Invalid number '{}': {}", l, e))?);
    right.push(r.parse::<i32>().map_err(|e| format!("Invalid number '{}': {}", r, e))?);
  }

  Ok((left, right))
}

#[cfg(test)]
mod test {
  use super::{day01_part1, day01_part2};

  #[test]
  fn test_day01_part_1() {
    let res = day01_part1().unwrap_or(-1);
    assert_eq!(res, 2176849);
  }

  #[test]
  fn test_day02_part_2() {
    let res = day01_part2().unwrap_or(-1);
    assert_eq!(res, 23384288);
  }
}
