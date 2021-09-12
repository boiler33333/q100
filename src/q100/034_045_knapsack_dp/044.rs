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

fn tetrahedron() -> Vec<usize> {
  let mut a = vec![];
  let mut v = 1;
  let mut n = 1;
  while v < 1_000_000 {
    v = n * (n+1) * (n+2) / 6;
    a.push(v);
    n += 1;
  }
  a
}

fn gen_dp(tetrahedron: &Vec<usize>) -> (Vec<usize>, Vec<usize>) {
  let mut dp1 = vec![MAX; 1_000_000];
  let mut dp2 = vec![MAX; 1_000_000];
  dp1[0] = 0;
  dp2[0] = 0;
  for &v in tetrahedron {
    for n in 1..1_000_000 {
      if n < v {
        continue;
      }
      dp1[n] = min(dp1[n], dp1[n-v]+1);
      if v % 2 == 1 {
        dp2[n] = min(dp2[n], dp2[n-v]+1);
      }
    }
  }
  (dp1, dp2)
}

fn main() {
  let a = tetrahedron();
  let (dp1, dp2) = gen_dp(&a);
  loop {
    let n: usize = read();
    if n == 0 {
      break;
    }
    println!("{} {}", dp1[n], dp2[n]);
  }
}

#[test]
fn test_solve() {
  let arg = vec![40,14,5,165,120,103,106,139];
  let a = tetrahedron();
  let (dp1, dp2) = gen_dp(&a);
  let want = vec![(2,6),(2,14),(2,5),(1,1),(1,18),(5,35),(4,4),(3,37)];
  for (i, &n) in arg.iter().enumerate() {
    assert_eq!(dp1[n], want[i].0);
    assert_eq!(dp2[n], want[i].1);
  }
}