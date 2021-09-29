use proconio::input;
use std::usize::MAX;

fn main() {
  input! {
    n: usize,
    hs: [(usize, usize); n],
  }
  let mut low = 0;
  let mut high = MAX;
  while high - low > 1 {
    let mid = (low + high)/2;
    let mut ok = true;
    let mut t = vec![0; n];
    for (i, &(h, s)) in hs.iter().enumerate() {
      if mid >= h {
        t[i] = (mid - h) / s;
      } else {
        ok = false;
      }
    }
    t.sort();
    for i in 0..n {
      if t[i] < i {
        ok = false;
      }
    }
    if ok {
      high = mid;
    } else {
      low = mid;
    }
  }
  println!("{}", high);
}