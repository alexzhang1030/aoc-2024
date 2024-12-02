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
fn day01() -> Result<i32, String> {
  // Time complexity: O(n log n)
  // TODO: Optimize to O(n)
  let text = include_str!("./input.txt");

  let mut left = Vec::new();
  let mut right = Vec::new();

  for line in text.lines() {
    let mut parts = line.split_whitespace();
    let l = parts.next().ok_or_else(|| format!("Missing left part in line: {}", line))?;
    let r = parts.next().ok_or_else(|| format!("Missing right part in line: {}", line))?;
    left.push(l.parse::<i32>().map_err(|e| format!("Invalid number '{}': {}", l, e))?);
    right.push(r.parse::<i32>().map_err(|e| format!("Invalid number '{}': {}", r, e))?);
  }

  left.sort();
  right.sort();

  Ok(left.iter().zip(right.iter()).map(|(l, r)| (l - r).abs()).sum())
}

#[cfg(test)]
mod test {
  use super::day01;

  #[test]
  fn test_day01() {
    let res = day01().unwrap_or(-1);
    assert_eq!(res, 2176849);
  }
}
