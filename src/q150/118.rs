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

const MOD: usize = 1_000_000_007;

struct RollingHash {
  hash: Vec<usize>,
  power: Vec<usize>,
}

impl RollingHash {
  fn new(s: String) -> Self {
    let s: Vec<usize> = s.bytes().map(|x| (x-48) as usize).collect();
    let n = s.len();
    let b = 100_007;
    let mut power = vec![0; n+1];
    power[0] = 1;
    for i in 0..n {
      power[i+1] = power[i] * b % MOD;
    }
    let mut hash = vec![0; n+1];
    for i in 0..n {
      hash[i+1] = (hash[i] * b + s[i]) % MOD;
    }
    Self { hash, power }
  }
  fn get(&self, l: usize, r: usize) -> usize {
    let h1 = self.hash[r];
    let h2 = self.hash[l] * self.power[r-l] % MOD;
    if h1 > h2 { h1 - h2 } else { (h1 + MOD) - h2 }
  }
}

fn main() {
  let t = read::<String>();
  let p = read::<String>();
  let n = t.len();
  let m = p.len();
  if m > n {
    return;
  }
  let t = RollingHash::new(t);
  let p = RollingHash::new(p);
  let p = p.get(0, m);
  for l in 0..n-m+1 {
    let r = l + m;
    let t = t.get(l, r);
    if p == t {
      println!("{}", l);
    }
  }
}