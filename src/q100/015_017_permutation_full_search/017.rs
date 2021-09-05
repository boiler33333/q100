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

trait LexicalPermutation {
  fn next_permutation(&mut self) -> bool;
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
}

fn judge(p: &Vec<(usize, usize)>, q: &Vec<usize>) -> bool {
  let mut board = vec![vec![false; 8]; 8];
  for (y, &x) in q.iter().enumerate() {
    board[y][x] = true;
  }
  for &(y, x) in p {
    if !board[y][x] {
      return false;
    }
  }
  if !diag(&board) {
    return false;
  }
  board.reverse();
  if !diag(&board) {
    return false;
  }
  true
}

// y = ax + b (a = -1, 0 <= b < 16)
fn diag(board: &Vec<Vec<bool>>) -> bool {
  for b in 0..16 {
    let mut cnt = 0;
    for x in 0..=b {
      let y = b - x;
      if y < 8 && x < 8 && board[y][x] {
        cnt += 1;
      }
    }
    if cnt > 1 {
      return false;
    }
  }
  true
}

fn main() {
  let n = read::<usize>();
  let mut p = vec![(0, 0); n];
  for i in 0..n {
    p[i] = (read(), read());
  }

  let mut q: Vec<usize> = (0..8).collect();
  loop {
    if judge(&p, &q) {
      for &x in &q {
        let mut s: Vec<char> = vec!['.'; 8];
        s[x] = 'Q';
        let s: String = s.into_iter().collect();
        println!("{}", s);
      }
    }
    if !q.next_permutation() {
      break;
    }
  }
}
