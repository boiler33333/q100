use proconio::input;

fn main() {
  input! {
    n: usize,
    cp: [(usize, usize); n],
    q: usize,
    lr: [(usize, usize); q],
  }
  let mut acc1 = vec![0; n+1];
  let mut acc2 = vec![0; n+1];
  for i in 0..n {
    let (c, p) = cp[i];
    if c == 1 {
      acc1[i+1] = acc1[i] + p;
      acc2[i+1] = acc2[i];
    } else {
      acc1[i+1] = acc1[i];
      acc2[i+1] = acc2[i] + p;
    }
  }
  for (l, r) in lr {
    let ans1 = acc1[r] - acc1[l-1];
    let ans2 = acc2[r] - acc2[l-1];
    println!("{} {}", ans1, ans2);
  }
}