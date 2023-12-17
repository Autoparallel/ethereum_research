# Attacking Poseidon
# Attacks
There of course has been some existing formal analysis of the security of Poseidon. Some of th


# Algebraic Hash Cryptanalysis
Algebraic attacks on cryptographic hash functions involve exploiting the algebraic properties and structures within the hash function. These attacks may attempt to solve equations that describe the hash function's behavior or find vulnerabilities in its algebraic representation. The Poseidon hash function's design and analysis likely include considerations to mitigate such attacks, but the exact methodologies and conclusions drawn in the paper are not accessible in this context.

## Interpolation Attack
This type of attack attempts to construct an interpolation polynomial that accurately describes the hash function. If the number of unknown monomials in the polynomial is sufficiently large, constructing it becomes as hard as a brute-force attack. For Poseidon, the security against this type of attack is related to the number of different monomials in the interpolation polynomial, which in turn depends on the degree of the function.

The mathematical intuition behind this attack is that a higher number of rounds generally increases the complexity of the interpolation polynomial, making it more difficult for an attacker to accurately construct or exploit it. The parameters of the Poseidon hash function, including the size of the finite field (denoted by p), the number of S-boxes applied per round, and the algebraic properties of these S-boxes, directly influence the number of different monomials in the interpolation polynomial.

The number of rounds required to prevent this attack denoted by the sum of partial and full rounds $R = RP +RF$ is bound by 

<!-- need to understand this more -->
$$
RP + RF \geq 1 + \frac{\log_{\alpha}(2) \cdot \min \{ M, \log_2(p) \} }{ \log_{\alpha}(t) }
$$

