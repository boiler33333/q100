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

fn dfs(
  graph: &Vec<Vec<usize>>,
  seen: &mut Vec<bool>,
  from: usize,
  st: &mut Vec<usize>,
  et: &mut Vec<usize>,
  t: &mut usize,
) {
  *t += 1;
  st[from] = *t;
  seen[from] = true;
  for &to in &graph[from] {
    if !seen[to] {
      dfs(graph, seen, to, st, et, t);
    }
  }
  *t += 1;
  et[from] = *t;
}

fn main() {
  let n: usize = read();
  let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
  for _ in 0..n {
    let u: usize = read();
    let k: usize = read();
    for _ in 0..k {
      let v: usize = read();
      graph[u-1].push(v-1);
    }
  }
  let mut seen = vec![false; n];
  let mut t = 0;
  let mut st = vec![0; n];
  let mut et = vec![0; n];
  for i in 0..n {
    if !seen[i] {
      dfs(&graph, &mut seen, i, &mut st, &mut et, &mut t);
    }
  }
  for i in 0..n {
    println!("{} {} {}", 1+i, st[i], et[i]);
  }
}