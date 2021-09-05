// 最小公倍数 (least common multiple) 
//  ２つ以上の正の整数の共通な倍数
//  lcm(27, 36) -> 108
fn lcm(a: usize, b: usize) -> usize {
  a / gcd(a, b) * b
}

// 最大公約数 (greatest common divisor)
//  ２つ以上の正の整数に共通な約数
//  gcd(12, 18) -> 6
fn gcd(a: usize, b: usize) -> usize {
  if a % b == 0 {
    return b;
  }
  gcd(b, a % b)
}