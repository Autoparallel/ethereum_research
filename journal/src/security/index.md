# Security of Poseidon
As you have seen from our book here, the purpose of this repository is to document a thorough understanding of the Poseidon hash function and its security.
Let's make sure we understand why this matters.

## Security of Hash Functions
The security of hash functions is a cornerstone in the field of cryptography. 
Secure hash functions are essential for a variety of applications, from securing passwords to ensuring data integrity and enabling complex cryptographic protocols. 
Here, we discuss what it means for hash functions to be secure and why these properties are crucial.

## What Makes a Hash Function Secure?

A secure hash function is designed to satisfy several important properties:

### Preimage Resistance

- **Definition:** Given a hash output $y$, it should be computationally infeasible to find any input $x$ such that $\texttt{hash}(x) = y$.
We refer to $x$ as a *preimage* of $y$ and we refer to this property as *preimage resistance*.
- Preimage resistance ensures that even if a hash value is known, the original data cannot be derived from it, protecting against reverse-engineering.
This is part of the one-way nature of hash functions.

### Second Preimage Resistance

- **Definition:** Given an input $x_1$, it should be computationally infeasible to find a different input $x_2$ such that $\texttt{hash}(x_1) = \texttt{hash}(x_2)$.
- Second preimage resistance property is vital for data integrity, ensuring that the input data cannot be tampered with without changing the hash.

### Collision Resistance

- **Definition:** It should be computationally infeasible to find any two distinct inputs $x_1$ and $x_2$ such that $\texttt{hash}(x_1) = \texttt{hash}(x_2)$.
- Collision resistance is crucial for cryptographic applications like digital signatures, where unique hash values are necessary to prevent fraud.

### Quick note
When comparing second preimage resistance and collision resistance, I (Colin) found it helpful to think of second preimage resistance as a stronger version of collision resistance as otherwise the two seem very similar.
Second preimage resistance is the property that given a specific input, it is hard to find another input that hashes to the same value whereas collision resistance is the property that it is hard to find any two inputs that hash to the same value.
Your search space is confined in second preimage resistance since you have one fixed input, but in collision resistance, you can search over all possible inputs and keep track of which ones you have already tried and their hashes.

### Why Do These Security Properties Matter?

The security properties of hash functions have direct implications for their practical use:

- Data Integrity
    - Secure hashes are used to verify data integrity in transmission and storage. Any tampering with the data would result in a different hash value, alerting the system to potential corruption.

- Authentication
    - Hash functions, in combination with digital signatures, authenticate the origin and verify the integrity of messages, preventing impersonation and forgery.

- Password Security
    - Hashing passwords before storing them ensures that even if the hash is compromised, the original password remains protected due to pre-image resistance.

- Blockchain and Cryptocurrencies
    - In blockchain technology, secure hash functions underpin the integrity of the ledger, with each block being linked to the previous one through hashes, creating a tamper-evident chain.

## Current state of Poseidon security
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

