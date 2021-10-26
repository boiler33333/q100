use proconio::input;
use std::cmp::max;
use std::collections::HashMap;

fn check(s: &str, d: usize) -> bool {
  let n = s.len();
  let mut hm = HashMap::new();
  for l in 0..n-d+1 {
    let r = l + d;
    let k = &s[l..r];
    if let Some(&r2) = hm.get(k) {
      if r2 <= l {
        return true
      }
    } else {
      hm.insert(k, d);
    }
  }
  false
}

fn main() {
  input! {
    n: usize,
    s: String
  }
  let mut l = 0;
  let mut r = n;
  let mut ans = 0;
  while r - l > 1 {
    let m = (l + r) / 2;
    if check(&s, m) {
      ans = max(ans, m);
      l = m;
    } else {
      r = m;
    }
  }
  println!("{}", ans);
}