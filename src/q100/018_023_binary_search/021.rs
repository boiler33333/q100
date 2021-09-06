use proconio::input;
use std::usize::MAX;

fn main() {
  input! {
    n: usize,
    hs: [(usize, usize); n],
  }
  let mut left = 0;
  let mut right = MAX;
  while right - left > 1 {
    let mid = (left + right) / 2;
    let mut ok = true;
    let mut t = vec![0; n];
    for (i, &(h, s)) in hs.iter().enumerate() {
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
      left = mid;
    }
  }
  println!("{}", right);
}