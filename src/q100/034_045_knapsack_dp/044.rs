use std::cmp::min;
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

fn setup() -> (Vec<usize>, Vec<usize>) {
  let mut n = 1;
  let mut i = 1;
  let mut xs = vec![];
  while n < 1_000_000 {
    n = i*(i+1)*(i+2)/6;
    xs.push(n);
    i += 1;
  }
  let mut dp1 = vec![MAX; 1_000_000];
  let mut dp2 = vec![MAX; 1_000_000];
  dp1[0] = 0;
  dp2[0] = 0;
  for x in xs {
    for n in 1..1_000_000 {
      if n < x {
        continue;
      }
      dp1[n] = min(dp1[n], dp1[n-x] + 1);
      if x % 2 == 1 {
        dp2[n] = min(dp2[n], dp2[n-x] + 1);
      }
    }
  }
  (dp1, dp2)
}

fn main() {
  let (dp1, dp2) = setup();
  loop {
    let n: usize = read();
    if n == 0 {
      break;
    }
    println!("{} {}", dp1[n], dp2[n]);
  }
}