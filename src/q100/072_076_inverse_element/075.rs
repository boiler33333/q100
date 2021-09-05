use proconio::input;

const MOD: i64 = 1_000_000_007;

fn inv(x: i64) -> i64 {
  pow(x, MOD-2) % MOD
}

fn pow(x: i64, n: i64) -> i64 {
  if n == 0 {
    1
  } else if n % 2 == 0 {
    pow(x * x % MOD, n/2)
  } else {
    x * pow(x, n-1) % MOD 
  }
}

fn rec(
  n: i64,
  g: &Vec<Vec<usize>>,
  p: Option<usize>,
  u: usize,
  ans: &mut i64,
) -> i64 {
  let mut res = 1;
  let mut ts = vec![];
  for &v in &g[u] {
      if Some(v) == p {
          continue;
      }
      let t = rec(n, g, Some(u), v, ans);
      res += t;
      res %= MOD;
      ts.push(t);
  }
  if p != None {
      ts.push(n - res);
  }
  let mut now = pow(2, n-1) - 1;
  now %= MOD;
  for &t in &ts {
      now -= pow(2, t) - 1;
  }
  now += MOD;
  now %= MOD;
  *ans += now;
  *ans %= MOD;
  res
}

fn main() {
  input! {
    n: usize,
    ab: [(usize, usize); n-1],
  }
  let mut graph = vec![vec![]; n+1];
  for (a, b) in ab {
    graph[a-1].push(b-1);
    graph[b-1].push(a-1);
  }
  let mut ans = 0;
  let n = n as i64;
  rec(n, &graph, None, 0, &mut ans);
  ans *= inv(pow(2, n));
  ans %= MOD;
  println!("{}", ans);
}