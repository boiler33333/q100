use proconio::input;
use std::collections::HashSet;

fn main() {
  input! {
    n: usize,
    s: [String; n],
  }
  let mut hs = HashSet::new();
  for i in 0..n {
    if !hs.contains(&s[i]) {
      println!("{}", i+1);
      hs.insert(&s[i]);
    }
  }
}