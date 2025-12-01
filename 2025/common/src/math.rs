//! Mathematical utilities for Advent of Code

use num::{Integer, Signed};

/// Greatest Common Divisor using Euclidean algorithm
pub fn gcd<T: Integer + Copy>(a: T, b: T) -> T {
    if b.is_zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Least Common Multiple
pub fn lcm<T: Integer + Copy>(a: T, b: T) -> T {
    if a.is_zero() || b.is_zero() {
        T::zero()
    } else {
        (a / gcd(a, b)) * b
    }
}

/// LCM of a slice of numbers
pub fn lcm_all<T: Integer + Copy>(numbers: &[T]) -> T {
    numbers.iter().copied().reduce(lcm).unwrap_or(T::one())
}

/// GCD of a slice of numbers
pub fn gcd_all<T: Integer + Copy>(numbers: &[T]) -> T {
    numbers.iter().copied().reduce(gcd).unwrap_or(T::zero())
}

/// Extended Euclidean Algorithm
/// Returns (gcd, x, y) where ax + by = gcd(a, b)
pub fn extended_gcd<T: Integer + Signed + Copy>(a: T, b: T) -> (T, T, T) {
    if b.is_zero() {
        (a, T::one(), T::zero())
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

/// Modular inverse of a mod m (if it exists)
pub fn mod_inverse<T: Integer + Signed + Copy>(a: T, m: T) -> Option<T> {
    let (g, x, _) = extended_gcd(a, m);
    if g == T::one() {
        Some(((x % m) + m) % m)
    } else {
        None
    }
}

/// Chinese Remainder Theorem solver
/// Given pairs of (remainder, modulus), find x where x ≡ r_i (mod m_i) for all i
pub fn chinese_remainder_theorem(pairs: &[(i64, i64)]) -> Option<i64> {
    let (result, product) = pairs.iter().try_fold((0i64, 1i64), |(acc, prod), &(r, m)| {
        let g = gcd(prod, m);
        if (r - acc) % g != 0 {
            return None;
        }
        let lcm_val = prod / g * m;
        let inv = mod_inverse(prod / g, m / g)?;
        let x = (acc + prod * ((r - acc) / g % (m / g) * inv % (m / g))).rem_euclid(lcm_val);
        Some((x, lcm_val))
    })?;
    Some(result.rem_euclid(product))
}

/// Shoelace formula for polygon area
pub fn shoelace_area(vertices: &[(i64, i64)]) -> i64 {
    let n = vertices.len();
    let mut sum = 0i64;
    for i in 0..n {
        let j = (i + 1) % n;
        sum += vertices[i].0 * vertices[j].1;
        sum -= vertices[j].0 * vertices[i].1;
    }
    sum.abs() / 2
}

/// Pick's theorem: interior points = area - boundary_points/2 + 1
pub fn interior_points(area: i64, boundary_points: i64) -> i64 {
    area - boundary_points / 2 + 1
}

/// Modular exponentiation: base^exp mod m
pub fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp /= 2;
        base = base * base % modulus;
    }
    result
}

/// Count digits in a number
pub fn digit_count(mut n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
}

/// Split number in half by digits
pub fn split_number(n: u64) -> Option<(u64, u64)> {
    let count = digit_count(n);
    if count % 2 != 0 {
        return None;
    }
    let divisor = 10u64.pow(count / 2);
    Some((n / divisor, n % divisor))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 13), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(3, 5), 15);
    }

    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(3, 7), Some(5)); // 3 * 5 = 15 ≡ 1 (mod 7)
    }

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 10, 1000), 24); // 2^10 = 1024 ≡ 24 (mod 1000)
    }

    #[test]
    fn test_digit_count() {
        assert_eq!(digit_count(12345), 5);
        assert_eq!(digit_count(0), 1);
    }

    #[test]
    fn test_split_number() {
        assert_eq!(split_number(1234), Some((12, 34)));
        assert_eq!(split_number(123), None);
    }
}

