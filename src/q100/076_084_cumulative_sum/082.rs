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

fn second(s: &str) -> usize {
  let b: Vec<usize> = s.bytes().map(|b| (b - 48) as usize).collect();
  let hh = 10 * b[0] + b[1];
  let mm = 10 * b[3] + b[4];
  let ss = 10 * b[6] + b[7];
  3600 * hh + 60 * mm + ss
}

fn solve(n: usize, st: &[String], et: &[String]) -> i64 {
  let m = 24 * 3600;
  let mut acc = vec![0; m];
  for i in 0..n {
    let a = second(&st[i]);
    let b = second(&et[i]);
    acc[a] += 1;
    acc[b] -= 1;
  }
  for i in 1..m {
    acc[i] += acc[i-1];
  }
  *acc.iter().max().unwrap()
}

fn main() {
  loop {
    let n: usize = read();
    if n == 0 {
      break;
    }
    let mut st = vec![];
    let mut et = vec![];
    for _ in 0..n {
      st.push(read::<String>());
      et.push(read::<String>());
    }
    let ans = solve(n, &st, &et);
    println!("{}", ans);
  }
}