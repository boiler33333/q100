use proconio::input;
use std::cmp::max;

fn main() {
  input! {
    n: usize, m: usize,
    p: [usize; n],
  }
  let ans = solve(m, &p);
  println!("{}", ans);
}

fn solve(m: usize, p: &[usize]) -> usize {
  let mut p = p.to_vec();
  p.push(0);
  let mut q = vec![];
  for &p1 in &p {
    for &p2 in &p {
      q.push(p1 + p2);
    }
  }
  q.sort();
  let n = q.len();
  let mut ret = 0;
  for i in 0..n {
    if q[i] > m {
      break;
    }
    let v = m - q[i];
    let j = q.binary_search(&v).unwrap_or_else(|x| x);
    ret = max(ret, q[i] + q[j-1]);
  }
  ret
}

#[test]
fn test_solve_1() {
  let m = 50;
  let p = vec![3, 14, 15, 9];
  let result = solve(m, &p);
  assert_eq!(result, 48);
}

#[test]
fn test_solve_2() {
  let m = 21;
  let p = vec![16, 11, 2];
  let result = solve(m, &p);
  assert_eq!(result, 20);
}