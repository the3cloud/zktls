# Benchmark

## RISC0 Backend

> Generate the groth 16 proof.

| Input Length | Output Length | Cycle (user / total)    | CPU Model          | Core Number | Max Memory | GPU Model   | Max GPU Memory | Proof Time     |
| ------------ | ------------- | ----------------------- | ------------------ | ----------- | ---------- | ----------- | -------------- | -------------- |
| 56B          | 426B          | 37,758,703 / 41,943,040 | 8369B <sup>1</sup> | 8           | 4460MB     | Nvidia A10  | 9246MiB        | 282.295366971s |
| 56B          | 426B          | 37,758,703 / 41,943,040 | 8163 <sup>2</sup>  | 8           | 4908MB     | Nvidia T4   | 8605MiB        | 426.617830981s |
| 56B          | 426B          | 37,758,703 / 41,943,040 | 8255C <sup>3</sup> | 8           | 4491MB     | Nvidia V100 | 9983MiB        | 324.583163623s |


## SP1 Backend

> Generate the groth 16 proof.

| Input Length | Output Length | Cycle      | CPU Model          | Core Number | Max Memory | GPU Model   | Max GPU Memory | Proof Time     |
| ------------ | ------------- | ---------- | ------------------ | ----------- | ---------- | ----------- | -------------- | -------------- |
| 56B          | 426B          | 36,018,041 | 8369B <sup>1</sup> | 8           | 19.2GB     | Nvidia A10  | 14682MiB       | 174.762700752s |
| 56B          | 426B          | 36,018,041 | 8163 <sup>2</sup>  | 8           | 14.1GB     | Nvidia T4   | 13725MiB       | 464.79359922s  |
| 56B          | 426B          | 36,018,041 | 8255C <sup>3</sup> | 12          | 19.6GB     | Nvidia V100 | 14747MiB       | 182.572047771s |

## Comments

> 1: Intel(R) Xeon(R) Platinum 8369B CPU @ 2.90GHz
>
> 2: Intel(R) Xeon(R) Platinum 8163 CPU @ 2.50GHz
>
> 3: Intel(R) Xeon(R) Platinum 8255C CPU @ 2.50GHz
