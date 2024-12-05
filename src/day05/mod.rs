use std::collections::{HashMap, HashSet, VecDeque};

/// Print Queue
/// ```plaintext
/// 47|53
/// 97|13
/// 97|61
/// 97|47
/// 75|29
/// 61|13
/// 75|53
/// 29|13
/// 97|29
/// 53|29
/// 61|53
/// 97|53
/// 61|29
/// 47|13
/// 75|47
/// 97|75
/// 47|61
/// 75|61
/// 47|29
/// 75|13
/// 53|13
///
/// 75,47,61,53,29
/// 97,61,53,29,13
/// 75,29,13
/// 75,97,47,61,53
/// 61,13,29
/// 97,13,75,29,47
/// ```
///
/// Rules:
/// - `X|Y` rule represents that `X` should be appeared before `Y`
/// - All rules should be satisfied for each numbers
/// - You need to extract the middle number of each line
/// - And get the sum of all the middle numbers
///
/// Example above:
/// - `75,47,61,53,29` is valid, the middle number is `61`
/// - `97,61,53,29,13` is valid, the middle number is `53`
/// - `75,29,13` is valid, the middle number is `29`
///
/// Answer: 61 + 53 + 29 = 143
#[allow(dead_code)]
fn day05_part1() -> i32 {
  let (rules, updates) = from_str();

  let valid_updates: Vec<Vec<i32>> = updates
    .iter()
    .filter(|update| {
      let sorted = topological_sort(update, &rules);
      sorted == **update
    })
    .cloned()
    .collect();

  calculate_middle_sum(&valid_updates, &rules)
}

/// Part two
/// Add the following rules:
/// - Now we have to fix the incorrect order
/// - And only sum the middle number of the correct order which has been fixed
///
/// Example above:
/// - `75,97,47,61,53` becomes `97,75,47,61,53`
/// - `61,13,29` becomes `61,29,13`
/// - `97,13,75,29,47` becomes `97,75,47,29,13`
///
/// Answer: 47 + 29 + 47 = 123
#[allow(dead_code)]
fn day05_part2() -> i32 {
  let (rules, updates) = from_str();

  let valid_updates: Vec<Vec<i32>> = updates
    .iter()
    .filter(|update| {
      let sorted = topological_sort(update, &rules);
      sorted == **update
    })
    .cloned()
    .collect();

  let invalid_updates: Vec<Vec<i32>> =
    updates.iter().filter(|update| !valid_updates.contains(update)).cloned().collect();

  calculate_middle_sum(&invalid_updates, &rules)
}

fn calculate_middle_sum(updates: &[Vec<i32>], rules: &[(i32, i32)]) -> i32 {
  updates
    .iter()
    .filter_map(|update| {
      let sorted = topological_sort(update, rules);
      if !sorted.is_empty() {
        Some(sorted[sorted.len() / 2])
      } else {
        None
      }
    })
    .sum()
}

fn build_graph(
  update: &[i32],
  rules: &[(i32, i32)],
) -> (HashMap<i32, HashSet<i32>>, HashMap<i32, i32>) {
  let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
  let mut in_degree: HashMap<i32, i32> = HashMap::new();
  let update_set: HashSet<i32> = update.iter().copied().collect();

  for &page in update {
    graph.entry(page).or_default();
    in_degree.entry(page).or_insert(0);
  }

  for &(x, y) in rules {
    if update_set.contains(&x) && update_set.contains(&y) {
      graph.entry(x).or_default().insert(y);
      *in_degree.entry(y).or_insert(0) += 1;
    }
  }

  (graph, in_degree)
}

fn topological_sort(update: &[i32], rules: &[(i32, i32)]) -> Vec<i32> {
  let (graph, mut in_degree) = build_graph(update, rules);
  let mut queue: VecDeque<i32> =
    in_degree.iter().filter(|&(_, &degree)| degree == 0).map(|(&node, _)| node).collect();
  let mut sorted: Vec<i32> = Vec::new();

  while let Some(node) = queue.pop_front() {
    sorted.push(node);
    if let Some(neighbors) = graph.get(&node) {
      for &neighbor in neighbors {
        let entry = in_degree.get_mut(&neighbor).unwrap();
        *entry -= 1;
        if *entry == 0 {
          queue.push_back(neighbor);
        }
      }
    }
  }

  if sorted.len() == update.len() {
    sorted
  } else {
    Vec::new()
  }
}

fn from_str() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
  let text = include_str!("./input.txt");

  let mut rules: Vec<(i32, i32)> = vec![];
  let mut numbers: Vec<Vec<i32>> = vec![];
  let mut is_rule = true;

  for text in text.lines() {
    if text.is_empty() {
      is_rule = false;
      continue;
    }
    if is_rule {
      let mut parts = text.split("|");
      let x = parts.next().unwrap().parse::<i32>().unwrap();
      let y = parts.next().unwrap().parse::<i32>().unwrap();
      rules.push((x, y));
      continue;
    }
    let parts = text.split(",");
    let mut nums = vec![];
    for part in parts {
      nums.push(part.parse::<i32>().unwrap());
    }
    numbers.push(nums);
  }

  (rules, numbers)
}

#[cfg(test)]
mod test {
  use super::{day05_part1, day05_part2};

  #[test]
  fn test_day05_part1() {
    let res = day05_part1();
    assert_eq!(res, 4609);
  }

  #[test]
  fn test_day05_part2() {
    let res = day05_part2();
    assert_eq!(res, 5723);
  }
}
