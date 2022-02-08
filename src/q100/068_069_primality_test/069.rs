use proconio::input;

fn main() {
  input! {
    q: usize,
    lr: [(usize, usize); q],
  }
  //エラトステネスの篩
  let n = 100_000;
  let mut is_prime = vec![true; n+1];
  is_prime[0] = false;
  is_prime[1] = false;
  for i in 2..=n {
    if !is_prime[i] {
      continue;
    }
    let mut j = i+i;
    while j <= n {
      is_prime[j] = false;
      j += i;
    }
  }
  //2017に似た数 となる奇数 x の場合は１にする
  let mut acc = vec![0; n+1];
  for i in 2..=n {
    if i % 2 == 0 {
      continue;
    }
    if is_prime[i] && is_prime[(i+1)/2] {
      acc[i] = 1;
    }
  }
  //累積和
  for i in 1..=n {
    acc[i] += acc[i-1];
  }
  for (l, r) in lr {
    let ans = acc[r] - acc[l-1];
    println!("{}", ans);
  }
}