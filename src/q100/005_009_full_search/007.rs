use proconio::input;
use std::cmp::max;
use std::collections::HashSet;

fn main() {
  input! {
    n: usize,
    xy: [(i64, i64); n],
  }
  let ans = solve(n, &xy);
  println!("{}", ans);
}

fn solve(_: usize, xy: &[(i64, i64)]) -> i64 {
  let hs: HashSet<(i64, i64)> = xy.iter().cloned().collect();
  let mut ret = 0;
  for &(x1, y1) in xy {
    for &(x2, y2) in xy {
      if x1 == x2 && y1 == y2 {
        continue;
      }
      let (ax, ay) = (x2 - x1, y2 - y1);
      let (x3, y3) = (x2 + ay, y2 - ax);
      let (x4, y4) = (x1 + ay, y1 - ax);
      if hs.contains(&(x3, y3)) && hs.contains(&(x4, y4)) {
        let s = ax * ax + ay * ay;
        ret = max(ret, s);
      }
    }
  }
  ret
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