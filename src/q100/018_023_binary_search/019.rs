use proconio::input;
use std::cmp::min;

fn main() {
  input! {
    d: usize,
    n: usize,
    m: usize,
    mut s: [usize; n-1],
    mut k: [usize; m],
  }
  let ans = solve(d, m, &s, &k);
  println!("{}", ans);
}

fn solve(d: usize, m: usize, s: &[usize], k: &[usize]) -> usize {
  let mut s = s.to_vec();
  s.sort();
  s.insert(0, 0);
  s.push(d);
  let mut ret = 0;
  for j in 0..m {
    let i = s.binary_search(&k[j]).unwrap_or_else(|x| x);
    if i == 0 {
      continue;
    }
    ret += min(s[i] - k[j], k[j] - s[i-1]);
  }
  ret
}

#[test]
fn test_solve_1() {
  let s = vec![1, 3];
  let k = vec![4, 6];
  let result = solve(8, 2, &s, &k);
  assert_eq!(result, 3);
}

#[test]
fn test_solve_2() {
  let s = vec![8, 12, 16];
  let k = vec![7, 7, 11, 8];
  let result = solve(20, 4, &s, &k);
  assert_eq!(result, 3);
}