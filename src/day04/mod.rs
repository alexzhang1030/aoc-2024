/// Ceres Search
/// ```plaintext
/// MMMSXXMASM
/// MSAMXMSMSA
/// AMXSXMAAMM
/// MSAMASMSMX
/// XMASAMXAMM
/// XXAMMXXAMA
/// SMSMSASXSS
/// SAXAMASAAA
/// MAMMMXMMMM
/// MXMXAXMASX
/// ```
///
/// Rules:
/// - You need to find `XMAS` word
/// - This word search allows words to be horizontal, vertical, diagonal, written backwards,
/// - or even overlapping other words.
///
/// Example above:
/// In this word search, XMAS occurs a total of 18 times
///
/// ```plaintext
/// ....XXMAS.
/// .SAMXMS...
/// ...S..A...
/// ..A.A.MS.X
/// XMASAMX.MM
/// X.....XA.A
/// S.S.S.S.SS
/// .A.A.A.A.A
/// ..M.M.M.MM
/// .X.X.XMASX
/// ```
///
/// Answer: 18
#[allow(dead_code)]
fn day04_part1() -> i32 {
  let data = from_str();
  let mut count = 0;
  let directions = [
    (0, 1),   // right
    (1, 0),   // down
    (1, 1),   // down-right
    (0, -1),  // left
    (-1, 0),  // top
    (1, -1),  // top-left
    (-1, -1), // down-left
    (-1, 1),  // top-right
  ];
  let target = ['X', 'M', 'A', 'S'];

  let rows = data.len();
  let cols = data[0].len();

  for r in 0..rows {
    for c in 0..cols {
      for (dx, dy) in &directions {
        let mut found = true;
        for (i, &char) in target.iter().enumerate() {
          let x = r as isize + i as isize * dx;
          let y = c as isize + i as isize * dy;
          if x < 0
            || x >= rows as isize
            || y < 0
            || y >= cols as isize
            || data[x as usize][y as usize] != char
          {
            found = false;
            break;
          }
        }
        if found {
          count += 1;
        }
      }
    }
  }

  count
}

/// Part two
/// You need to find `X-MAS` word, like this
/// ```plaintext
/// M.S
/// .A.
/// M.S
/// ```
#[allow(dead_code)]
fn day04_part2() -> i32 {
  let grid = from_str();
  let rows = grid.len();
  let cols = grid[0].len();
  let mut count = 0;

  let is_x_mas = |r: usize, c: usize| -> bool {
    if r == 0 || c == 0 || r + 1 >= rows || c + 1 >= cols {
      return false;
    }

    let top_left = grid[r - 1][c - 1];
    let top_right = grid[r - 1][c + 1];
    let center = grid[r][c];
    let bottom_left = grid[r + 1][c - 1];
    let bottom_right = grid[r + 1][c + 1];

    if center != 'A' {
      return false;
    }

    is_pattern(&top_left, &bottom_right) && is_pattern(&top_right, &bottom_left)
  };

  for r in 0..rows {
    for c in 0..cols {
      if is_x_mas(r, c) {
        println!("Found X-MAS at row: {}, col: {}", r, c);
        count += 1;
      }
    }
  }

  count
}

fn is_pattern(x1: &char, x2: &char) -> bool {
  *x1 == 'S' && *x2 == 'M' || *x1 == 'M' && *x2 == 'S'
}

fn from_str() -> Vec<Vec<char>> {
  let str = include_str!("./input.txt");

  str.lines().map(|l| l.chars().collect()).collect()
}

#[cfg(test)]
mod test {
  use super::{day04_part1, day04_part2};

  #[test]
  fn test_day04_part1() {
    let res = day04_part1();
    assert_eq!(res, 2554);
  }

  #[test]
  fn test_day04_part2() {
    let res = day04_part2();
    assert_eq!(res, 1916);
  }
}
