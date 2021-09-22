use proconio::input;
use std::cmp::min;

fn main() {
  input! {
    n: usize,
    m: usize,
    p: [usize; m],
    abc: [(i64, i64, i64); n-1],
  }
  let mut acc = vec![0i64; n];
  for i in 1..m {
    if p[i] > p[i-1] {
      acc[p[i-1]-1] += 1;
      acc[p[i]-1] -= 1;
    } else {
      acc[p[i]-1] += 1;
      acc[p[i-1]-1] -= 1;
    }
  }
  for i in 1..n {
    acc[i] += acc[i-1];
  }
  let mut ans = 0;
  for i in 0..n-1 {
    let (a, b, c) = abc[i];
    let paper = acc[i] * a;
    let card = acc[i] * b + c;
    ans += min(paper, card);
  }
  println!("{}", ans);
}