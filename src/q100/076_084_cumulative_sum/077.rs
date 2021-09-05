use proconio::input;

const MOD: i64 = 1e5 as i64;

fn main() {
  input! {
    n: usize,
    m: usize,
    mut d: [i64; n-1],
    a: [i64; m],
  }
  d.insert(0, 0);
  for i in 1..n {
    d[i] += d[i-1];
  }
  let mut ans = 0;
  let mut u = 0;
  for i in 0..m {
    let v = u + a[i];
    {
      let u = u as usize;
      let v = v as usize;
      ans += (d[v] - d[u]).abs();
    }
    ans %= MOD;
    u = v;
  }
  println!("{}", ans);
}