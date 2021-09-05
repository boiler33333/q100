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

fn main() {
  let mut tetrahedron = Vec::new();
  let mut i = 1;
  let mut n = 0;
  while n < 1_000_000 {
    n = i * (i+1) * (i+2) / 6;
    tetrahedron.push(n);
    i += 1;
  }
  let mut dp1 = vec![MAX; 1_000_000];
  let mut dp2 = vec![MAX; 1_000_000];
  dp1[0] = 0;
  dp2[0] = 0;
  for i in 1..1_000_000 {
    for &n in &tetrahedron {
      if i < n {
        break;
      }
      dp1[i] = min(dp1[i], dp1[i-n]+1);
      if n % 2 == 1 {
        dp2[i] = min(dp2[i], dp2[i-n]+1);
      }
    }
  }
  loop {
    let n: usize = read();
    if n == 0 {
      break;
    }
    println!("{} {}", dp1[n], dp2[n])
  }
}