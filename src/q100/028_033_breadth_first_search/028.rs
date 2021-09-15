use std::collections::VecDeque;
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

fn bfs(
  n: usize,
  graph: &Vec<Vec<usize>>,
  s: usize,
) -> Vec<Option<usize>> {
  let mut dist = vec![None; n];
  let mut que: VecDeque<usize> = VecDeque::new();
  dist[s] = Some(0);
  que.push_back(s);
  while let Some(u) = que.pop_front() {
    for &v in &graph[u] {
      if dist[v] == None {
        dist[v] = dist[u].and_then(|x| Some(x+1));
        que.push_back(v);
      }
    }
  }
  dist
}

fn main() {
  let n: usize = read();
  let mut graph = vec![vec![]; n];
  for _ in 0..n {
    let u: usize = read();
    let d: usize = read();
    for _ in 0..d {
      let v: usize = read();
      graph[u-1].push(v-1);
    }
  }
  let dist = bfs(n, &graph, 0);
  for i in 0..n {
    if let Some(d) = dist[i] {
      println!("{} {}", i+1, d);
    } else {
      println!("{} -1", i+1);
    }
  }
}