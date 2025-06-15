# Running time of the parallel rho method

This program computes the expected running time of the parallel Pollard rho method. Used in

## Usage

```bash
cargo run --release -- --M=[NUMBER OF MACHINES] --kmax=[MAXIMUM K]
```

## Example

```bash
> cargo run --release -- --M=2 --kmax=3

1    1      |  0.7071067811865475
1    2      |  0.8251780685091852
1    3      |  0.8444731808090262
2    2      |  1.1153550716504106
2    3      |  1.1134863162263935
3    3      |  1.3226407151066075
```
