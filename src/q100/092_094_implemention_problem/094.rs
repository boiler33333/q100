use std::cmp::max;
use std::cmp::min;
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

type Cake = (usize, usize);

fn cut(cake: Cake, s: usize) -> (Cake, Cake) {
  let (w, d) = cake;
  let s = s % (w + d);
  if s < w {
    let w1 = min(s, w - s);
    let w2 = max(s, w - s);
    ((w1, d), (w2, d))
  } else {
    let d1 = min(s - w, w + d - s);
    let d2 = max(s - w, w + d - s);
    ((w, d1), (w, d2))
  }
}

fn solve(w: usize, d: usize, ps: &Vec<(usize, usize)>) {
  let mut cake = (w, d);
  let mut cakes = vec![cake];
  for &(p, s) in ps {
    cake = cakes[p];
    cakes.remove(p);
    let (cake1, cake2) = cut(cake, s);
    cakes.push(cake1);
    cakes.push(cake2);
  } 
  let mut area: Vec<usize> = cakes.iter().map(|&(w, d)| w * d).collect();
  area.sort();
  let area: Vec<String> = area.iter().map(|x| x.to_string()).collect();
  let ans = area.join(" ");
  println!("{}", ans);
}

fn main() {
  loop {
    let n: usize = read();
    let w: usize = read();
    let d: usize = read();
    if n == 0 && w == 0 && d == 0 {
      break;
    }
    let mut ps: Vec<(usize, usize)> = Vec::new();
    for _ in 0..n {
      let p: usize = read();
      let s: usize = read();
      ps.push((p-1, s));
    }
    solve(w, d, &ps);
  }
}