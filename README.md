# Code for my paper _Choosing iteration maps for the parallel Pollard rho method_

arxiv: [[2506.12844]](https://arxiv.org/abs/2506.12844)

This program computes the expected running time of the parallel Pollard rho method using the formula

$$
\frac 1 {\varphi(\ell)}
\sum_{d \in (\mathbb{Z}/\ell\mathbb{Z})^\times}
\left( \sum_{i = 1}^M
    \frac {\gcd(d - 1, 2k_i) - 1} {\log_2^2 2k_i}
\right)^{-1/2}
$$

from the paper.

_Usage_

```bash
cargo run --release -- --M=[NUMBER OF MACHINES] --kmax=[MAXIMUM K]
```

_Example_

```bash
> cargo run --release -- --M=2 --kmax=3

1    1      |  0.7071067811865475
1    2      |  0.8251780685091852
1    3      |  0.8444731808090262
2    2      |  1.1153550716504106
2    3      |  1.1134863162263935
3    3      |  1.3226407151066075
```

The numbers must always be interpreted relative to each other; the absolute running time heavily depends on the hardware and implementation details.
