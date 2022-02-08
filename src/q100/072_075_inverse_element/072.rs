use proconio::input;

const MOD: usize = 1_000_000_007;

fn combination(n: usize, k: usize) -> usize {
  let a = permutation(n, k);
  let b = factorial(k);
  a * pow(b, MOD-2) % MOD
}

fn factorial(k: usize) -> usize {
  (2..=k).fold(1, |sum, i| sum * i % MOD)
}

fn permutation(n: usize, k: usize) -> usize {
  (0..k).fold(1, |sum, i| sum * (n-i) % MOD)
}

fn pow(x: usize, n: usize) -> usize {
  if n == 0 {
    1
  } else if n % 2 == 0 {
    pow(x * x % MOD, n/2)
  } else {
    x * pow(x, n-1) % MOD 
  }
}

fn main() {
  input! { w: usize, h: usize }
  let ans = combination(w+h-2, w-1);
  println!("{}", ans);
}