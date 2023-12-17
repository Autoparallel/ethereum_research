# S-Boxes
Poseidon uses S-boxes (Substitution boxes) to apply a non-linear transformation to the input data.
This non-linearity is crucial for security, as it makes the relationship between the input and output complex and difficult to reverse-engineer or predict.
In particular, it strengthens the hash against linear and differential cryptanalysis.

Poseidon uses a specific type of S-boxes, which are based on integer powers of the input.
This is in contrast to other S-boxes, say used in AES, which may use other types of functions like lookups, multiplicative inverses, or XOR.
Poseidon typically uses integer powers of the input, such as $x^3$ or $x^5$.

### Why Non-linearity Matters

- **Resisting Linear Attacks:** Cryptographic algorithms need to withstand linear attacks, where an attacker tries to find linear relations (like invariant subspaces which are like generalizations of eigenvectors) in the algorithm to break it. 
Non-linear transformations disrupt such linear relationships.
- **Diffusion:** Non-linear S-boxes help in spreading the influence of a single input bit over many output bits, a property known as diffusion.

### Why choose $x^3$ and $x^5$?

1. **Non-linearity:**
   - Taking integer powers of numbers like $x^2$, $x^3$, $x^4$, and $x^5$ are inherently non-linear.
   - However, even powers (like $x^2$ or $x^4$) do exhibit some linear-like properties that we can see by doing:  
    $$
    (a + b)^2 = a^2 + 2ab + b^2 
    $$
    whwere see see that the $2ab$ is itself linear, leading to less non-linear behavior.
   - Odd powers ensure a higher degree of non-linearity, which is more effective in thwarting linear and differential cryptanalysis.

2. **Efficiency in Finite Fields:**
   - Computations like $x^3$ and $x^5$ can be very efficiently implemented in finite fields, especially those used in cryptographic applications like ZKPs.
   - These operations strike a balance between providing sufficient non-linearity and being computationally feasible.
   One could use higher powers like $x^{11}$ or $x^{13}$, but these would be more computationally expensive.
   - ZKPs are typically built on top of polynomial schemes, so using monomials like $x^3$ and $x^5$ is a natural choice as they don't require writing complex circuits.

### Invertibility
If we use a function like $x^3$ or $x^5$ in a cryptographic algorithm, we should be wary of whether it is invertible as this could lead to a vulnerability.
In the field of real numbers, both $x^3$ and $x^5$ are invertible. 
However, when it comes to finite fields, this isn't always the case.
In finite fields, the invertibility of functions like $x^3$ or $x^5$ depends on the specific properties of the field, especially its size and characteristic.

1. **General Case:**
   - In general, for a finite field $\mathbb{F}_q$ (where $q$ is a prime or a power of a prime), a function $f(x) = x^n$ is invertible if and only if $n$ is coprime to $q - 1$ (see this [link](https://math.stackexchange.com/questions/2239567/necessary-and-sufficient-condition-for-a-map-between-a-finite-field-being-invert)). This is because of the properties of the multiplicative group of the finite field.

2. **Specific Cases of $x^3$ and $x^5$:**
   - Note that $3$ and $5$ are prime numbers, so they are always coprime to $q - 1$.


## Other kinds of S-boxes
Substitution boxes (S-boxes) are crucial components in many cryptographic systems, used to perform non-linear transformations on input data. 
While S-boxes like those in the Poseidon hash function often use mathematical functions such as exponentiation in finite fields, there are various other designs and types of S-boxes used in different cryptographic algorithms. 
Some of these include the use of XOR operations, lookup tables, and more complex algebraic constructions.

- **AES (Advanced Encryption Standard):**
  - AES uses a fixed S-box for its non-linear transformation step. The S-box is essentially a predefined 256-element lookup table, where each byte of input is replaced by the byte at the corresponding position in the table.
  - The construction of the AES S-box is based on the multiplicative inverse in the finite field $\mathbb{F}_{2^8}$, followed by an affine transformation (which includes XOR with a constant).