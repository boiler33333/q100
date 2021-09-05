use proconio::input;
use std::cmp::min;
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
    d: usize,            //環状線の全長
    n: usize,            //店舗の個数
    m: usize,            //注文の個数
    mut s: [usize; n-1], //本店以外の店舗の位 置を
    k: [usize; m],       //宅配先の場所
  }
  s.sort();
  s.insert(0, 0);
  s.push(d);
  let ans = solve(s, k);
  println!("{}", ans);
}

fn solve(s: Vec<usize>, k: Vec<usize>) -> usize {
  let mut res = 0;
  for v in k {
    let i = s.lower_bound(&v);
    if i == 0 {
      continue;
    }
    let u1 = v - s[i-1];
    let u2 = s[i] - v;
    res += min(u1, u2);
  }
  res
}

#[test]
fn test_solve_1() {
  let s = vec![0, 1, 3, 8];
  let k = vec![4, 6];
  let result = solve(s, k);
  assert_eq!(result, 3);
}

#[test]
fn test_solve_2() {
  let s = vec![0, 8, 12, 16, 20];
  let k = vec![7, 7, 11, 8];
  let result = solve(s, k);
  assert_eq!(result, 3);
}