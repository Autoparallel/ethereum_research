# Attacks
There of course has been some existing formal analysis of the security of Poseidon. Some of th


## Invarient subspace attack

An invariant subspace attack on a cryptographic hash function, like Poseidon, involves finding a subset of the input space that, under the hash function's transformations, maps to itself or another invariant set. Mathematically, this means if $x$ is in this subset, then $f(x)$, where $f$ is the hash function, is also in this subset. The attacker exploits this invariance to predict outputs or find collisions. The effectiveness of such attacks depends on the specific design and mathematical properties of the hash function, including its non-linear and linear components.

## Differential Cryptanalysis
Differential cryptanalysis is a method of cryptanalysis that exploits the characteristics of differential probabilities to break a cipher. It is a general form of cryptanalysis that can be applied to many types of ciphers, including block ciphers, stream ciphers, and hash functions. The attack involves finding a difference between two plaintexts that results in a specific difference in the ciphertexts. The attacker then uses this information to recover the secret key. This attack is particularly effective against block ciphers that use a substitution-permutation network (SPN) structure.

The probability of successful differential attacks on Poseidon is determined by analyzing the number of active S-boxes in its structure. Poseidon is designed with a certain number of rounds, each involving substitution (S-boxes) and permutation steps. The security against differential attacks is higher when more active S-boxes are involved, as this increases the complexity and reduces the probability of finding exploitable differential patterns. The mathematical intuition hinges on the observation that each active S-box significantly reduces the likelihood of a successful differential attack by increasing the non-linearity and complexity of the transformation.


## Linear Cryptanalysis
A linear attack on a cryptographic hash function involves finding a linear relationship between the input and output bits. Attackers exploit these linear approximations to predict the hash output, which could compromise the hash function's security. In the context of the Poseidon hash function, the paper analyzes its resilience to linear attacks by examining its structure and transformations. The mathematical intuition behind this analysis focuses on the degree of linearity in the hash function's operations. The more non-linear the function, the less susceptible it is to linear attacks, enhancing its security

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

## Gröbner Basis Attack
A Gröbner basis is a particular kind of generating set of a polynomial ideal in multivariate polynomial rings. It has properties that make the solution of polynomial systems simpler and more efficient. A Gröbner basis for an ideal is a set of polynomials from which one can derive the same solutions to the polynomial system as from the original set of polynomials, but with more straightforward computation, particularly for solving systems of equations and testing ideal membership.

The paper outlines three steps regarding the attack:

(1) computing the Gröbner basis in degrevlex order,


#### Degrevlex Order
The "degrevlex" order, short for "degree reverse lexicographic order," is a specific type of monomial ordering used in computational algebra, particularly in the computation of Gröbner bases. In this ordering, monomials are first compared by their total degree, with lower degree monomials being considered smaller. If two monomials have the same total degree, a reverse lexicographic order is applied, which means the monomials are compared based on the powers of their variables, starting from the last variable and moving backwards. This order is commonly used due to its computational efficiency in Gröbner basis calculations.

(2) converting the result to the lexicographic order, and
(3) factorizing the univariate polynomial, and backsubstituting its roots.


To find a Gröbner basis for the Poseidon hash function, one would start by formulating the function as a system of polynomial equations. This involves expressing each step of the hash function (like the S-boxes and linear layers) in polynomial form. Then, using a chosen monomial order (such as degrevlex), a Gröbner basis algorithm like Buchberger's algorithm or F4/F5 algorithms is applied to these polynomial equations. The process iteratively refines the initial set of equations into a Gröbner basis, which simplifies the system and makes it easier to analyze algebraically.