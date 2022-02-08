use proconio::input;

fn main() {
  input! {
    m: usize,       //居住予定地の南北の長さ (km)
    n: usize,       //居住予定地の東西の長さ (km)
    k: usize,       //調査対象となる領域の個数
    s: [String; m], //居住予定地の情報
    abcd: [(usize, usize, usize, usize); k], //調査領域
  }
  let ans = solve(m, n, k, &s, &abcd);
  for (s1, s2, s3) in ans {
    println!("{} {} {}", s1, s2, s3);
  }
}

fn cumulative_sum(h: usize, w: usize, table: &mut Vec<Vec<usize>>) {
  for y in 0..h {
    for x in 1..w {
      table[y][x] += table[y][x-1];
    }
  }
  for y in 1..h {
    for x in 0..w {
      table[y][x] += table[y-1][x];
    }
  }
}
fn calc((a, b, c, d): (usize, usize, usize, usize), table: &Vec<Vec<usize>>) -> usize {
  table[c][d] + table[a-1][b-1] - table[c][b-1] - table[a-1][d]
}

fn solve(
  m: usize,
  n: usize,
  k: usize,
  s: &[String],
  abcd: &[(usize, usize, usize, usize)],
) -> Vec<(usize, usize, usize)> {
  let h = m+1;
  let w = n+1;
  let mut jungle = vec![vec![0; w]; h];
  let mut ocean = vec![vec![0; w]; h];
  let mut ice = vec![vec![0; w]; h];
  for y in 0..h-1 {
    for (x, c) in s[y].chars().enumerate() {
      match c {
        'J' => jungle[y+1][x+1] = 1,
        'O' => ocean[y+1][x+1] = 1,
        'I' => ice[y+1][x+1] = 1,
        _ => unreachable!(),
      }
    }
  }
  cumulative_sum(h, w, &mut jungle);
  cumulative_sum(h, w, &mut ocean);
  cumulative_sum(h, w, &mut ice);
  let mut ret = vec![];
  for i in 0..k {
    let s1 = calc(abcd[i], &jungle);
    let s2 = calc(abcd[i], &ocean);
    let s3 = calc(abcd[i], &ice);
    ret.push((s1, s2, s3));
  }
  ret
}

#[test]
fn test_solve() {
  let m = 4;
  let n = 7;
  let k = 4;
  let s = vec![
    String::from("JIOJOIJ"),
    String::from("IOJOIJO"),
    String::from("JOIJOOI"),
    String::from("OOJJIJO"),
  ];
  let abcd = vec![(3,5,4,7),(2,2,3,6),(2,2,2,2),(1,1,4,7)];
  let got = solve(m, n, k, &s, &abcd);
  let want = vec![(1,3,2),(3,5,2),(0,1,0),(10,11,7)];
  for i in 0..k {
    assert_eq!(got[i], want[i]);
  }
}