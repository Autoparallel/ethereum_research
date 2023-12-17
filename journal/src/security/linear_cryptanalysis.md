# Linear Cryptanalysis
## Invarient subspace attack

An invariant subspace attack on a cryptographic hash function, like Poseidon, involves finding a subset of the input space that, under the hash function's transformations, maps to itself or another invariant set. Mathematically, this means if $x$ is in this subset, then $f(x)$, where $f$ is the hash function, is also in this subset. The attacker exploits this invariance to predict outputs or find collisions. The effectiveness of such attacks depends on the specific design and mathematical properties of the hash function, including its non-linear and linear components.



## Linear Cryptanalysis
A linear attack on a cryptographic hash function involves finding a linear relationship between the input and output bits. Attackers exploit these linear approximations to predict the hash output, which could compromise the hash function's security. In the context of the Poseidon hash function, the paper analyzes its resilience to linear attacks by examining its structure and transformations. The mathematical intuition behind this analysis focuses on the degree of linearity in the hash function's operations. The more non-linear the function, the less susceptible it is to linear attacks, enhancing its security