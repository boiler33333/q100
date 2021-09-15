use std::io::*;
use std::str::FromStr;
use std::usize::MAX;

fn read<T: FromStr>() -> T {
  let s = stdin();
  let s = s.lock();
  let s: String = s.bytes()
    .map(|c| c.expect("failed reading char") as char)
    .skip_while(|c| c.is_whitespace())
    .take_while(|c| !c.is_whitespace())
    .collect();
  s.parse().ok().expect("failed parsing")
}

fn main() {
  let n: usize = read();
  let mut dp = vec![MAX; n+1];
  for _ in 0..n {
    let a = read::<usize>();
    let i = match dp.binary_search(&a) { Ok(i) => i, Err(i) => i };
    dp[i] = a;
  }
  let ans = dp.iter().position(|&x| x == MAX).unwrap();
  println!("{}", ans);
}