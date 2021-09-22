use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    abx: [(usize, usize, usize); m],
  }
  let ans = solve(n, m, &abx);
  println!("{}", ans);
}

fn solve(n: usize, _: usize, abx: &[(usize, usize, usize)]) -> usize {
  let mut acc = vec![vec![0; n+3]; n+3];
  for &(a, b, x) in abx {
    acc[a][b] += 1;
    acc[a][b+1] -= 1;
    acc[a+x+1][b] -= 1;
    acc[a+x+1][b+x+2] += 1;
    acc[a+x+2][b+1] += 1;
    acc[a+x+2][b+x+2] -= 1;
  }
  for a in 1..=n+2 {
    for b in 1..=n+2 {
      acc[a][b] += acc[a][b-1];
    }
  }
  for a in 1..=n+2 {
    for b in 1..=n+2 {
      acc[a][b] += acc[a-1][b];
    }
  }
  for a in 1..=n+2 {
    for b in 1..=n+2 {
      acc[a][b] += acc[a-1][b-1];
    }
  }
  let mut ret = 0;
  for a in 1..=n {
    for b in 1..=a {
      if acc[a][b] > 0 {
        ret += 1;
      }
    }
  }
  ret
}

#[test]
fn test_solve() {
  let n = 5;
  let m = 2;
  let abx = vec![(2,2,1),(2,1,3)];
  let got = solve(n, m, &abx);
  assert_eq!(got, 12);
}