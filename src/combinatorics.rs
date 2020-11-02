/// Returns the factorial of a given number n
/// # Arguments
/// `n` - A non-negative number
///
/// # Examples
///```
/// use ralgeb::combinatorics;
/// let result = combinatorics::fact(3);
///```
pub fn fact(n: usize) -> usize {
  if n == 1 {
    return 1;
  }
  return n * fact(n - 1);
}

/// Returns the permutation
///
/// # Arguments
/// `n` - The number of items which are to be arranged.
///
/// `r` - The slots available.
///
/// # Examples
/// ```
/// use ralgeb::combinatorics;
///
/// // Number of ways 3 people can sit in a 2 seated vehicle
/// // 3!/(3-2)! => 3! => 6 ways in which 4 people can be arranged
/// // let a,b,c be 3 people then the number of ways are
/// // ab,bc,ca,ba,cd,ac
///
/// let ways = combinatorics::permutation(4,3);
/// ```
///
pub fn permutation(n: usize, r: usize) -> usize {
  if n > r {
    fact(n) / fact(n - r)
  } else {
    1
  }
}

/// Returns the combinations
/// # Arguments
/// `n` - The number of items
///
/// `r` - The slots available
///
/// # Examples
/// ```
/// use ralgeb::combinatorics;
///
/// // Combinations of ways 4 people can sit in a 3 seated vehicle
/// // 4!/((4-3)!*3!) => 4 combinations
/// // let a,b,c,d be 4 people then result is
/// // abc, bcd, cda, dab
/// let combinations = combinatorics::combinations(4,3);
///
/// ```
///
pub fn combinations(n: usize, r: usize) -> usize {
  if n > r {
    permutation(n, r) / fact(r)
  } else {
    1
  }
}

#[cfg(test)]
mod tests {
  use crate::combinatorics;
  #[test]
  fn fact() {
    let f = combinatorics::fact(3);
    assert_eq!(6, f);
  }
  #[test]
  fn permutation() {
    let ways = combinatorics::permutation(3, 2);
    assert_eq!(ways, 6);
  }
  #[test]
  fn combinations() {
    let combi = combinatorics::combinations(4, 3);
    assert_eq!(combi, 4);
  }
}
