use proconio::input;

fn lcm(a: usize, b: usize) -> usize {
  a / gcd(a, b) * b
}

fn gcd(a: usize, b: usize) -> usize {
  if a % b == 0 {
    return b;
  }
  gcd(b, a % b)
}

fn main() {
  input! {
    n: usize,
    m: usize,
    mut a: [usize; n],
  }
  for i in 0..n {
    a[i] /= 2;
  }
  let mut x = a[0];
  for i in 1..n {
    x = lcm(x, a[i]);
  }
  for i in 0..n {
    if (x/a[i]) % 2 == 0 {
      println!("0");
      return;
    }
  }
  let ans = m / x;
  let ans = (ans + 1) / 2;
  println!("{}", ans);
}