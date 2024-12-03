use regex::Regex;

/// Mull It Over
/// ```plaintext
/// xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
/// ```
///
/// Rules:
/// - There are some instructions in the text, but its all jumbled up
/// - The instructions are:
/// - `mul(a,b)`, a and b are all 1-3 digit numbers
/// - `mul(4,*` is invalid
/// - `mul(6,9!` is invalid
/// - `?(12,34)` is invalid
/// - `mul ( 2, 4 )` is invalid
///
/// You need to:
/// - Find all the valid instructions
/// - Multiply the numbers in the valid instructions
/// - Sum all the results
///
/// Example above:
/// - `mul(2,4)` is valid, 2 * 4 = 8
/// - `mul(5,5)` is valid, 5 * 5 = 25
/// - `mul(11,8)` is valid, 11 * 8 = 88
/// - `mul(8,5)` is valid, 8 * 5 = 40
///
/// Answer: 8 + 25 + 88 + 40 = 161
#[allow(dead_code)]
fn day03_part1() -> i32 {
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  let str = from_str();
  let mut sum = 0;

  let mut res = 0;
  for (_, [num1, num2]) in re.captures_iter(str).map(|c| c.extract()) {
    let num1 = num1.parse::<i32>().unwrap();
    let num2 = num2.parse::<i32>().unwrap();
    res += num1 * num2;
  }
  sum += res;

  sum
}

/// Part two
/// All part one rules are still applicable, additionally:
/// - `do()` instruction, enables future `mul` instructions.
/// - `don't()` instruction, disables future `mul` instructions.
/// - Only the most recent do() or don't() instruction applies.
/// - At the beginning of the program, mul instructions are `enabled`.
///
/// Example:
/// ```plaintext
/// xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
/// ```
///
/// Example above:
/// - `mul(5,5)` and `mul(11,8)` are disabled by `don't()`
///
/// Answer: mul(2,4) + mul(8,5) = 2 * 4 + 8 * 5 = 48
#[allow(dead_code)]
fn day03_part2() -> i32 {
  let re = Regex::new(r"mul\((\d+,\d+)\)|(don't\(\))|(do\(\))").unwrap();
  let str = from_str();

  let mut sum = 0;

  let mut enabled = true;
  for (_, [captured]) in re.captures_iter(str).map(|c| c.extract()) {
    match captured {
      "do()" => enabled = true,
      "don't()" => enabled = false,
      _ => {
        if enabled {
          let num = captured.split(",").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
          sum += num[0] * num[1];
        }
      }
    }
  }

  sum
}

fn from_str<'a>() -> &'a str {
  include_str!("./input.txt")
}

#[cfg(test)]
mod test {
  use super::{day03_part1, day03_part2};

  #[test]
  fn test_day03_part1() {
    let res = day03_part1();
    assert_eq!(res, 166905464);
  }
  #[test]
  fn test_day03_part2() {
    let res = day03_part2();
    assert_eq!(res, 72948684);
  }
}
