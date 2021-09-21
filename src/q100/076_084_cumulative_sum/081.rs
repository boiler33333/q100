use proconio::input;

fn main() {
  input! {
    n: usize,
    ab: [(usize, usize); n],
  }
  let mut acc = vec![0; 1_000_002];
  for (a, b) in ab {
    acc[a] += 1;
    acc[b+1] -= 1;
  }
  for i in 1..1_000_002 {
    acc[i] += acc[i-1];
  }
  let ans = acc.iter().max().unwrap();
  println!("{}", ans);
}