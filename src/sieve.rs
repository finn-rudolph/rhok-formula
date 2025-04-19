use std::ops::AddAssign;

pub fn gcd(n: usize, m: usize) -> usize {
    if m == 0 { n } else { gcd(m, n % m) }
}

pub fn lcm(n: usize, m: usize) -> usize {
    return (n / gcd(n, m)) * m;
}

// removes all factors of p from n
fn remove_prime(mut n: usize, p: usize) -> usize {
    while n % p == 0 {
        n /= p
    }
    n
}

pub struct Sieve {
    largest_prime_divisor: Vec<usize>,
    phi: Vec<usize>,
}

impl Sieve {
    pub fn new(n_max: usize) -> Sieve {
        let mut largest_prime_divisor = vec![1; n_max + 1];
        let mut phi = vec![1usize; n_max + 1];

        for p in 2..=n_max {
            if largest_prime_divisor[p] == 1 {
                let mut n = p;
                while n <= n_max {
                    largest_prime_divisor[n] = p;
                    if n % (p * p) == 0 {
                        phi[n] = phi[n / p] * p;
                    } else {
                        phi[n] = phi[n / p] * (p - 1);
                    }
                    n += p;
                }
            }
        }

        return Sieve {
            largest_prime_divisor,
            phi,
        };
    }

    pub fn sum_over_divisors<T: Default + AddAssign>(&self, n: usize, f: impl Fn(usize) -> T) -> T {
        return self._sum_over_divisors(n, &f, 1);
    }

    fn _sum_over_divisors<T: Default + AddAssign>(
        &self,
        mut n: usize,
        f: &impl Fn(usize) -> T,
        divisor: usize,
    ) -> T {
        if n == 1 {
            return f(divisor);
        }

        let p = self.largest_prime_divisor[n];
        let n_ = remove_prime(n, p);
        let mut result = T::default();
        while n >= n_ {
            result += self._sum_over_divisors(n_, f, divisor * n / n_);
            n /= p;
        }
        result
    }

    pub fn phi(&self, n: usize) -> usize {
        return self.phi[n];
    }

    pub fn psi(&self, n: usize, g: usize) -> usize {
        assert!(n % g == 0);

        let p = self.largest_prime_divisor[n];
        if n - n / p == self.phi[n] {
            // n is a power of p
            return if g % p == 0 {
                self.phi[n / g]
            } else {
                self.phi[n] - n / p
            };
        }

        let n_ = remove_prime(n, p);
        let g_ = remove_prime(g, p);
        return self.psi(n_, g_) * self.psi(n / n_, g / g_);
    }
}

#[cfg(test)]
mod test {
    use super::Sieve;

    #[test]
    fn test_psi() {
        let s = Sieve::new(100);
        assert_eq!(s.psi(3, 3), 1);
        assert_eq!(s.psi(4, 1), 0);
        assert_eq!(s.psi(12, 3), 0);
        assert_eq!(s.psi(6, 2), 1);
    }

    #[test]
    fn test_sum_over_divisors() {
        let s = Sieve::new(100);
        assert_eq!(s.sum_over_divisors(6, |d| d), 12);
        assert_eq!(s.sum_over_divisors(8, |d| d * d + 1), 89);
        assert_eq!(s.sum_over_divisors(61, |_d| 1), 2);
    }
}
