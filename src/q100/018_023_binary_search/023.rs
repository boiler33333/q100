use proconio::input;
use std::cmp::max;
use std::cmp::Ordering::{ Less, Equal, Greater };

pub trait BinarySearch<T> {
  fn lower_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
  fn lower_bound(&self, x: &T) -> usize {
    let mut left = 0;
    let mut right = self.len();
    while left < right {
      let mid = (left + right) / 2;
      let ord = self[mid].cmp(x);
      match ord {
        Less => left = mid + 1,
        Equal | Greater => right = mid,
      }
    }
    left
  }
}

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
  let mut q = Vec::new();
  for &p1 in &p {
    for &p2 in &p {
      q.push(p1 + p2);
    }
  }
  q.sort();
  let mut point = 0;
  for &a in &q {
    if a > m {
      break;
    }
    let b = m - a;
    let i = q.lower_bound(&b);
    point = max(point, a + q[i-1]);
  }
  point
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