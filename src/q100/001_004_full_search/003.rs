use proconio::input;
use std::cmp::max;

fn main() {
  input! { s: String }
  let mut ans = 0;
  let mut cnt = 0;
  for c in s.chars() {
    match c {
      'A' | 'C' | 'G' | 'T' => cnt += 1,
      _ => cnt = 0,
    };
    ans = max(ans, cnt);
  }
  println!("{}", ans);
}