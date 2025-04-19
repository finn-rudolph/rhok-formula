pub fn gcd(n: u64, m: u64) -> u64 {
    if m == 0 { n } else { gcd(m, n % m) }
}

pub fn lcm(n: u64, m: u64) -> u64 {
    return (n / gcd(n, m)) * m;
}

pub struct Numbers {
    prime_divisors: Vec<Vec<(u64, u32)>>,
    phi: Vec<u64>,
}

impl Numbers {
    pub fn new(n_max: usize) -> Numbers {
        let mut prime_divisors = vec![Vec::<(u64, u32)>::new(); n_max + 1];
        let mut phi = vec![1u64; n_max + 1];

        for p in 2..=n_max {
            if prime_divisors[p].is_empty() {
                let mut n = p;
                while n <= n_max {
                    let e = if n % (p * p) == 0 {
                        prime_divisors[n / p].last().unwrap().1 + 1
                    } else {
                        1
                    };
                    prime_divisors[n].push((p as u64, e));
                    phi[n] *= (p.pow(e - 1) * (p - 1)) as u64;

                    n += p;
                }
            }
        }

        return Numbers {
            prime_divisors,
            phi,
        };
    }

    pub fn sum_over_divisors(&self, n: u64, f: impl Fn(u64) -> f64) -> f64 {
        return self._sum_over_divisors(n, &f, 1);
    }

    fn _sum_over_divisors(&self, n: u64, f: &impl Fn(u64) -> f64, multiplier: u64) -> f64 {
        if n == 1 {
            return f(multiplier);
        }

        let p = self.prime_divisors[n as usize][0].0;
        let n_ = n / p.pow(self.prime_divisors[n as usize][0].1);
        return (0..=self.prime_divisors[n as usize][0].1)
            .map(|e| self._sum_over_divisors(n_, f, p.pow(e)))
            .sum();
    }

    pub fn phi(&self, n: u64) -> u64 {
        return self.phi[n as usize];
    }

    pub fn psi(&self, n: u64, g: u64) -> u64 {
        assert!(n % g == 0);

        if self.prime_divisors[n as usize].len() == 1 {
            return if g % self.prime_divisors[n as usize][0].0 == 0 {
                self.phi[(n / g) as usize]
            } else {
                self.phi[n as usize] - n / self.prime_divisors[n as usize][0].0
            };
        }

        return self.prime_divisors[n as usize]
            .iter()
            .map(|(p, e)| {
                let prime_power = p.pow(*e);
                self.psi(prime_power, gcd(prime_power, g))
            })
            .product();
    }
}

#[cfg(test)]
mod test {
    use super::Numbers;

    #[test]
    fn test_psi() {
        let num = Numbers::new(1000);
        assert_eq!(num.psi(3, 3), 1);
        assert_eq!(num.psi(4, 1), 0);
        assert_eq!(num.psi(12, 3), 0);
        assert_eq!(num.psi(6, 2), 1);
    }
}
