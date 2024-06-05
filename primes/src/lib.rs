/// Generate an increment table
///
/// The seivew should be indexed by `test_number` mod `seive_length`. This maps `seive_length` to 0.
/// # Panic
/// Panics if the product of the seed_primes overflows.
pub fn generate_prime_seive(seed_primes: &[u32]) -> Vec<u32> {
    let length: u32 = seed_primes.iter().product(); // NOTE: May panic
    let next_prime = first_new_prime(seed_primes);

    let mut seive: Vec<u32> = Vec::<u32>::with_capacity(length.try_into().unwrap());
    let mut distance_to_next_prime: u32 = 1;
    for idx in (distance_to_next_prime..length).chain(0..distance_to_next_prime) {}

    todo!()
}

pub(crate) fn first_new_prime(known_primes: &[u32]) -> u32 {
    let upper_bound = known_primes.iter().product();
    for n in 2..upper_bound {
        if naive_is_prime(n, known_primes) {
            return n;
        }
    }
    0 // NOTE: document this!
}

/// Test if a number is prime, given a list of known primes.
///
/// This operation scans through the entire list of known primes.]
pub(crate) fn naive_is_prime(n: u32, known_primes: &[u32]) -> bool {
    if n <= 1 {
        return false;
    }
    for &prime in known_primes {
        if n == prime {
            return true;
        } else if n.rem_euclid(prime) == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive_is_prime_simple_test() {
        let known_primes = &[2, 3, 5, 7];

        assert_eq!(naive_is_prime(8, known_primes), false);
        assert_eq!(naive_is_prime(9, known_primes), false);
        assert_eq!(naive_is_prime(10, known_primes), false);
        assert_eq!(naive_is_prime(11, known_primes), true);
        assert_eq!(naive_is_prime(12, known_primes), false);
        assert_eq!(naive_is_prime(13, known_primes), true);
        assert_eq!(naive_is_prime(14, known_primes), false);
    }

    #[test]
    fn naive_is_prime_test_lower_bound() {
        let known_primes = &[2];

        assert_eq!(naive_is_prime(0, known_primes), false);
        // 1 is not prime, neither is it composite.
        assert_eq!(naive_is_prime(1, known_primes), false);
    }

    #[test]
    fn naive_is_prime_test_on_known_primes() {
        let known_primes = &[2, 3, 5, 7];

        assert_eq!(naive_is_prime(2, known_primes), true);
        assert_eq!(naive_is_prime(3, known_primes), true);
        assert_eq!(naive_is_prime(5, known_primes), true);
        assert_eq!(naive_is_prime(7, known_primes), true);
    }

    #[test]
    fn naive_is_prime_test_empty_known_primes() {
        assert_eq!(naive_is_prime(0, &[]), false);
        assert_eq!(naive_is_prime(1, &[]), false);
        assert_eq!(naive_is_prime(2, &[]), true);
        assert_eq!(naive_is_prime(3, &[]), true);
    }

    #[test]
    fn naive_is_prime_test_above_seive_range() {
        // NOTE: I am expecting the naive test to be wrong.
        assert_eq!(naive_is_prime(9, &[2]), true);
        assert_eq!(naive_is_prime(25, &[2, 3]), true);
    }
}
