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

fn seconds(s: String) -> usize {
  let b: Vec<u8> = s.bytes().collect();
  let hh: usize = (10 * (b[0] - 48) + (b[1] - 48)).into();
  let mm: usize = (10 * (b[3] - 48) + (b[4] - 48)).into();
  let ss: usize = (10 * (b[6] - 48) + (b[7] - 48)).into();
  3600 * hh + 60 * mm + ss
}

fn main() {
  loop {
    let n = read::<usize>();
    if n == 0 {
      break;
    }
    let m = 3600 * 24;
    let mut acc = vec![0; m+1];
    for _ in 0..n {
      let st = seconds(read::<String>());
      let et = seconds(read::<String>());
      acc[st] += 1;
      acc[et] -= 1;
    }
    for i in 0..m {
      acc[i+1] += acc[i];
    }
    let ans = acc.iter().max().unwrap();
    println!("{}", ans);
  }
}