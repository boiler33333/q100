use proconio::input;

const MOD: usize = 1_000_000_007;

fn pow(x: usize, n: usize) -> usize {
  if n == 0 {
    1
  } else if n % 2 == 0 {
    pow(x * x % MOD, n/2)
  } else {
    x * pow(x, n-1) % MOD 
  }
}

fn inv(x: usize) -> usize {
  pow(x, MOD - 2)
}

fn rec(
  n: usize,
  u: usize,
  p: usize, //parent
  graph: &Vec<Vec<usize>>,
  deg: &mut [Vec<usize>],
) -> usize {
  let mut s = 1;
  for &v in &graph[u] {
    if v != p {
      let t = rec(n, v, u, graph, deg);
      deg[u].push(t);
      s += t;
    }
  }
  deg[u].push(n - s);
  s
}

fn main() {
  input! {
    n: usize,
    ab: [(usize, usize); n-1],
  }
  let mut graph = vec![vec![]; n];
  for (a, b) in ab {
    graph[a-1].push(b-1);
    graph[b-1].push(a-1);
  }
  let mut deg = vec![vec![]; n];
  for &v in &graph[0] {
    let t = rec(n, v, 0, &graph, &mut deg);
    deg[0].push(t);
  }
  let mut s = 0;
  for i in 0..n {
    s = (s + pow(2, n-1) + MOD - 1) % MOD;
    for &t in &deg[i] {
      s = (s + MOD - pow(2, t) + 1) % MOD;
    }
  }
  let ans = s * inv(pow(2, n)) % MOD;
  println!("{}", ans);
}