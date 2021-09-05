use proconio::input;
use std::usize::MAX;

fn main() {
  input! {
    n: usize,
    p: [(usize, usize); n],
  }
  let mut left = 0;
  let mut right = MAX;
  while left < right {
    let mid = (left + right) / 2;
    let mut ok = true;
    let mut t = vec![0; n];
    for (i, &(h, s)) in p.iter().enumerate() {
      if mid < h {
        ok = false;
      } else {
        t[i] = (mid - h) / s;
      }
    }
    t.sort();
    for i in 0..n {
      if t[i] < i {
        ok = false;
      }
    }
    if ok {
      right = mid;
    } else {
      left = mid + 1;
    }
  }
  println!("{}", right);
}