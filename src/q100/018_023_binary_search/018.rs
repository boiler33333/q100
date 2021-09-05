use std::io::*;
use std::str::FromStr;

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
  let n = read::<usize>();
  let mut s = vec![0; n];
  for i in 0..n {
    s[i] = read();
  }
  let mut ans = 0;
  let q = read::<usize>();
  for _ in 0..q {
    let t: usize = read();
    match s.binary_search(&t) {
      Ok(_) => ans += 1,
      Err(_) => {},
    }
  }
  println!("{}", ans);
}