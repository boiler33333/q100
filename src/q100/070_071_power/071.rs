use proconio::input;

const MOD: i64 = 1_000_000_007;

fn pow(x: i64, n: i64) -> i64 {
  if n == 0 {
    1
  } else if n % 2 == 0 {
    pow(x * x % MOD, n/2)
  } else {
    x * pow(x, n-1) % MOD
  }
}

fn main() {
  input! {
    n: usize, mut q: usize,
    a: [i64; n],
    mut c: [usize; q],
  }
  c.insert(0, 1);
  c.push(1);
  q = c.len();
  let mut acc = vec![0; n];
  for i in 1..n {
    acc[i] = pow(a[i-1], a[i]);
  }
  for i in 1..n {
    acc[i] += acc[i-1];
  }
  let mut ans = 0;
  for i in 1..q {
    let x2 = acc[c[i]-1];
    let x1 = acc[c[i-1]-1];
    ans += (x2 - x1).abs();
    ans %= MOD;
  }
  println!("{}", ans);
}