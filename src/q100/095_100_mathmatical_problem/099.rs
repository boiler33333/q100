use proconio::input;

fn main() {
  input! {
    m: usize,
    dc: [(usize, usize); m],
  }
  let mut sum = 0;
  let mut cnt = 0;
  for (d, c) in dc {
    sum += d * c;
    cnt += c;
  }
  let ans = (cnt - 1) + (sum - 1) / 9;
  println!("{}", ans);
}