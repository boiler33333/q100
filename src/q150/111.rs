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

fn dfs(
  parent: &mut Vec<Vec<usize>>,
  depth: &mut Vec<usize>,
  graph: &Vec<Vec<usize>>,
  u: usize,
  p: usize,
  d: usize,
) {
  for &v in &graph[u] {
    if v != p {
      parent[0][v] = u;
      depth[v] = depth[u] + 1;
      dfs(parent, depth, graph, v, u, d+1);
    }
  }
}

fn lca(
  log_n: usize,
  parent: &mut Vec<Vec<usize>>,
  depth: &Vec<usize>,
  u: usize,
  v: usize,
) -> usize {
  let mut u = u;
  let mut v = v;
  if depth[u] > depth[v] {
    let tmp = u;
    u = v;
    v = tmp;
  }
  let d = depth[v] - depth[u];
  for i in 0..log_n {
    if d & 1<<i > 0 {
      v = parent[i][v];
    }
  }
  if u == v {
    return u;
  }
  for i in (0..log_n).rev() {
    if parent[i][u] != parent[i][v] {
      u = parent[i][u];
      v = parent[i][v];
    }
  }
  parent[0][u]
}

fn main() {
  let n: usize = read();
  let mut graph = vec![vec![]; n];
  for i in 0..n {
    let k: usize = read();
    for _ in 0..k {
      let c: usize = read();
      graph[i].push(c);
      graph[c].push(i);
    }
  }

  let mut log_n = 1;
  while 1 << log_n < n {
    log_n += 1;
  }

  let mut depth = vec![0; n];
  let mut parent = vec![vec![0; n]; log_n];
  dfs(&mut parent , &mut depth, &graph, 0, MAX, 0);

  for i in 0..log_n-1 {
    for j in 0..n {
      parent[i+1][j] = parent[i][parent[i][j]];
    }
  }
  
  let q: usize = read();
  for _ in 0..q {
    let u: usize = read();
    let v: usize = read();
    let ans = lca(log_n, &mut parent, &depth, u, v);
    println!("{}", ans);
  }
}