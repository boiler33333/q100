use proconio::input;
use std::collections::HashSet;
use std::cmp::max;

fn main() {
  input! {
    n: usize,
    xy: [(i64, i64); n],
  }
  let ans = solve(n, &xy);
  println!("{}", ans);
}

fn solve(n: usize, xy: &Vec<(i64, i64)>) -> i64 {
  let hs: HashSet::<(i64, i64)> = xy.iter().cloned().collect();
  let mut ans = 0;
  for i in 0..n {
    for j in 0..n {
      if i == j {
        continue;
      }
      let (x1, y1) = xy[i];
      let (x2, y2) = xy[j];
      let (ax, ay) = (x2 - x1, y2 - y1);
      let (x3, y3) = (x2 - ay, y2 + ax);
      let (x4, y4) = (x1 - ay, y1 + ax);
      if hs.contains(&(x3, y3)) && hs.contains(&(x4, y4)) {
        let s = ax * ax + ay * ay;
        ans = max(ans, s);
      }
    }
  }
  ans
}

#[test]
fn test_solve() {
  let n = 10;
  let xy = vec![
    (9, 4), (4, 3), (1, 1), (4, 2), (2, 4),
    (5, 8), (4, 0), (5, 3), (0, 5), (5, 2),
  ];
  assert_eq!(solve(n, &xy), 10);
}