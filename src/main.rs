use std::{collections::HashMap, env};

fn gcd(n: u64, m: u64) -> u64 {
    if m == 0 { n } else { gcd(m, n % m) }
}

fn lcm(n: u64, m: u64) -> u64 {
    return (n / gcd(n, m)) * m;
}

struct Nat {
    prime_divisors: Vec<Vec<(u64, u64)>>,
    phi: Vec<u64>,
}

impl Nat {
    fn new(n_max: usize) -> Nat {
        let mut prime_divisors = vec![Vec::<(u64, u64)>::new(); n_max + 1];
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
                    phi[n] *= (p.pow(e as u32 - 1) * (p - 1)) as u64;

                    n += p;
                }
            }
        }

        return Nat {
            prime_divisors,
            phi,
        };
    }

    fn psi(&self, n: u64, g: u64) -> u64 {
        if self.prime_divisors[n as usize].len() == 1 {
            return if n % g == 0 {
                self.phi[(n / g) as usize]
            } else {
                self.phi[n as usize] - n / self.prime_divisors[n as usize][0].0
            };
        }

        return self.prime_divisors[n as usize]
            .iter()
            .map(|(p, e)| self.psi(p.pow(*e as u32), g))
            .product();
    }
}

fn main() {
    let args: HashMap<String, String> = env::args()
        .skip(1)
        .map(|arg| {
            let parts: Vec<&str> = arg.split('=').collect();
            assert_eq!(parts.len(), 2);
            (parts[0].to_owned(), parts[1].to_owned())
        })
        .collect();
    let k_max: usize = args["--kmax"].parse().unwrap();
    let m: usize = args["--M"].parse().unwrap();

    print!("{}", k_max);

    let nat = Nat::new(2 * k_max.pow(m as u32));
}
