use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
  let s = stdin();
  let s = s.lock();
  let s: String = s.bytes()
    .map(|c| c.expect("failed reading char") as char)
    .skip_while(|c| c.is_whitespace())
    .take_while(|c| !c.is_whitespace())
    .collect();
  s.parse().ok().expect("failed parsing")
}

fn main() {
  let n: usize = read();
  let mut rc: Vec<(usize, usize)> = vec![];
  for _ in 0..n {
    let r: usize = read();
    let c: usize = read();
    rc.push((r, c));
  }
  let table = solve(&rc);
  for row in table {
    println!("{}", row);
  }
}

fn solve(rc: &[(usize, usize)]) -> Vec<String> {
  let mut p: Vec<usize> = (0..8).collect();
  let mut table: Vec<String> = vec![];
  loop {
    if judge(&rc, &p) {
      for &i in &p {
        let mut s: Vec<char> = vec!['.'; 8];
        s[i] = 'Q';
        let s: String = s.into_iter().collect();
        table.push(s)
      }
    }
    if !p.next_permutation() {
      break;
    }
  }
  table
}

fn judge(rc: &[(usize, usize)], p: &[usize]) -> bool {
  let mut table = vec![vec![false; 8]; 8];
  for (r, &c) in p.iter().enumerate() {
    table[r][c] = true;
  }
  for &(r, c) in rc {
    if !table[r][c] {
      return false;
    }
  }
  if !diag(&table) {
    return false;
  }
  table.reverse();
  if !diag(&table) {
    return false;
  }
  true
}

// y = ax + b (a = -1, 0 <= b < 16
fn diag(table: &Vec<Vec<bool>>) -> bool {
  for b in 0..16 {
    let mut cnt = 0;
    for x in 0..=b {
      let y = b - x;
      if y < 8 && x < 8 && table[y][x] {
        cnt += 1;
      }
    }
    if cnt > 1 {
      return false;
    }
  }
  true
}

trait LexicalPermutation {
  fn next_permutation(&mut self) -> bool;
  fn prev_permutation(&mut self) -> bool;
}
impl<T> LexicalPermutation for [T] where T: Ord {
  fn next_permutation(&mut self) -> bool {
    if self.len() < 2 {
      return false;
    }
    let mut i = self.len() - 1;
    while i > 0 && self[i-1] >= self[i] {
      i -= 1;
    }
    if i == 0 {
      return false;
    }
    let mut j = self.len() - 1;
    while j >= i && self[j] <= self[i-1] {
      j -= 1;
    }
    self.swap(j, i-1);
    self[i..].reverse();
    true
  }

  fn prev_permutation(&mut self) -> bool {
    if self.len() < 2 {
      return false;
    }
    let mut i = self.len() - 1;
    while i > 0 && self[i-1] <= self[i] {
      i -= 1;
    }
    if i == 0 {
      return false;
    }
    self[i..].reverse();
    let mut j = self.len() - 1;
    while j >= i && self[j-1] < self[i-1] {
      j -= 1;
    }
    self.swap(i-1, j);
    true
  }
}

#[test]
fn test_solve_1() {
  let rc = vec![(2,2),(5,3)];
  let got = solve(&rc);
  let want = vec![
    "......Q.",
    "Q.......",
    "..Q.....",
    ".......Q",
    ".....Q..",
    "...Q....",
    ".Q......",
    "....Q...",
  ];
  for i in 0..8 {
    assert_eq!(got[i], want[i]);
  }
}
