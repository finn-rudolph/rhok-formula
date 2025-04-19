mod numbers;
mod tuple_iter;

use numbers::Numbers;
use std::{collections::HashMap, env};
use tuple_iter::TupleIter;

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

    let num = Numbers::new(2 * k_max.pow(m as u32));

    for k in TupleIter::new(m, 1, k_max as u64) {
        let l = k.iter().fold(1, |acc, k_i| numbers::lcm(acc, 2 * *k_i));

        let log_2ki_squared: Vec<f64> = k
            .iter()
            .map(|k_i| (1.0 + (*k_i as f64).log2()) * (1.0 + (*k_i as f64).log2()))
            .collect();

        for k_i in &k {
            print!("{:<5}", k_i)
        }

        print!(
            "  |  {}\n",
            num.sum_over_divisors(l / 2, |g| num.psi(l, 2 * g) as f64
                / (0..m)
                    .map(|i| (2 * numbers::gcd(g, k[i]) - 1) as f64 / log_2ki_squared[i])
                    .sum::<f64>()
                    .sqrt())
                / num.phi(l) as f64
        )
    }
}
