use proconio::input;

fn main() {
  input! {
    n: usize,      //宿場町の個数
    m: usize,      //旅の日数
    s: [i64; n-1], //宿場町間の距離
    a: [i64; m],   //移動
  }
  let ans = solve(n, m, &s, &a);
  println!("{}", ans);
}

fn solve(n: usize, m: usize, s: &[i64], a: &[i64]) -> i64 {
  let mut acc = s.to_vec();
  acc.insert(0, 0);
  for i in 0..n-1 {
    acc[i+1] += acc[i];
  }
  let mut ret = 0;
  let mut u = 0;
  for i in 0..m {
    let v = u + a[i];
    {
      let u = u as usize;
      let v = v as usize;
      ret += (acc[v] - acc[u]).abs();
      ret %= 100000;
    }
    u = v;
  }
  ret
}

#[test]
fn test_solve_1() {
  let n = 7;
  let m = 5;
  let s = vec![2,1,1,3,2,1];
  let a = vec![2,-1,3,2,-3];
  let got = solve(n, m, &s, &a);
  assert_eq!(got, 18);
}