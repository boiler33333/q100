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

fn solve(n: usize, x: usize) {
  let mut ans = 0;
  for i in 1..=n {
    for j in i+1..=n {
      for k in j+1..=n {
        if i + j + k == x {
          ans += 1;
        }
      }
    }
  }
  println!("{}", ans);
}

fn main() {
  loop {
    let n: usize = read();
    let x: usize = read();
    if n == 0 && x == 0 {
      break;
    }
    solve(n, x);
  }
}