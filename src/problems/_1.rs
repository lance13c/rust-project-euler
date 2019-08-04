// Multiples of 3 and 5

// Problem 1
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

// Find the sum of all the multiples of 3 or 5 below 1000.


  pub fn solve() -> u32 {

    return mul_sum();
  }

  fn mul_sum() -> u32 {
    let mut sum = 0;

    const RANGE_TOP:u32 = 1000;

    for num in (3..RANGE_TOP).rev() {
      if num % 5 == 0 { sum += num; }
      else if num % 3 == 0 { sum += num; }
    }

    return sum
  }





