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
  let n: usize = read();
  let mut a: Vec<usize> = vec![0; n];
  for i in 0..n {
    a[i] = read();
  }
  let m: usize = read();
  let mut q: Vec<usize> = vec![0; m];
  for j in 0..m {
    q[j] = read();
  }
  let mut ans = vec![false; m];
  for bits in 0..(1<<n) {
    let mut sum = 0;
    for i in 0..n {
      if (bits >> i) & 1 == 0 {
        sum += a[i];
      }
    }
    for j in 0..m {
      if q[j] == sum {
        ans[j] = true;
      }
    }
  }
  for ok in ans {
    println!("{}", if ok { "yes" } else { "no" });
  }
}