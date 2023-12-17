# Scalar Field
Poseidon uses a scalar field from an elliptic curve which of size $q=p^k$ for some prime $p$ and some integer $k$.
These are often called *finite fields* or, in particular when $k=1$, *prime fields*.
When $k>1$, these are called *extension fields* or *Galois fields*.
Elliptic curves provide a natural way to construct scalar fields, and many modern cryptographic systems use elliptic curves.

Many ZKP systems, especially zk-SNARKs, are built around elliptic curve cryptography (ECC). Using a scalar field derived from the same elliptic curve ensures that all operations are in the same field, which simplifies and optimizes computations.
These come in the form of operations such as the [pairing computations](https://medium.com/@VitalikButerin/exploring-elliptic-curve-pairings-c73c1864e627), which are more efficiently conducted in the scalar field of an elliptic curve. 
Scalar fields from elliptic curves often provide stronger security properties for a given field size compared to standard finite fields. 
This is due to the more complex structure of elliptic curves, which can increase the difficulty of certain cryptographic attacks.

For a given level of security, elliptic curves can often use smaller key sizes compared to algorithms based on standard finite fields. 
This results in better performance and smaller memory usage, which is particularly advantageous in constrained environments.
Since many modern cryptographic systems use elliptic curves, designing a hash function that operates in the same domain can facilitate better interoperability between different cryptographic primitives and protocols.

## Mathematical Details

   - A scalar field associated with an elliptic curve is the field over which the elliptic curve is defined. In elliptic curve cryptography (ECC), these fields are typically either prime fields denoted as $\mathbb{F}_p$ (where $p$ is prime) or binary fields denoted as $\mathbb{F}_{2^k}$.

- **Prime Fields ($\mathbb{F}_p$):**
   - In a prime field, the operations of addition, subtraction, multiplication, and division (excluding division by zero) are defined modulo a prime number $p$. 
   - Elliptic curves over prime fields are defined by equations with coefficients in $\mathbb{F}_p$ which we call the *scalar field*.
   - An example of such a curve is the [secp256r1 curve](https://neuromancer.sk/std/secg/secp256r1).

- **Binary Fields ($\mathbb{F}_{2^k}$):**
   - Binary fields are a special case of finite fields of prime power order, where the prime is 2. Here, the field has $2^k$ elements, and the operations are performed modulo an irreducible polynomial of degree $k$ (which comes from Galois extensions).
   - Elliptic curves over binary fields are particularly useful in certain cryptographic applications due to their efficient implementation in hardware and software. 
   For instance, you can use Fast Fourier Transform (FFT) algorithms to perform multiplication in binary fields, which is more efficient than the multiplication algorithms used in prime fields.
   - An example is the [NIST B-283 curve](https://neuromancer.sk/std/nist/B-283).


The size of the scalar field of an elliptic curve, $|p^k|$, indicates the number of elements in the field. 
The choice between prime fields and binary fields can impact the efficiency, security, and complexity of the cryptographic algorithms that use these elliptic curves.

### Cryptographic Significance
The properties of the scalar field, including its size, play a crucial role in the security of ECC. 
They determine the complexity of the [discrete logarithm problem](https://en.wikipedia.org/wiki/Discrete_logarithm), which is the foundation of the security in ECC-based cryptographic systems.

### Poseidon Implementation
Poseidon reference implementations use various different scalar fields from different elliptic curves in order to provide flexibility and interoperability with other cryptographic primitives as well as more freedom in choosing the security parameters.
Specifically, the curves are from the families:
- [BN curves](https://neuromancer.sk/std/bn/) (Barreto-Naehrig curves)
- [BLS curves](https://neuromancer.sk/std/bls/) (Boneh-Lynn-Shacham curves)
- [ED curves (one example)](https://neuromancer.sk/std/other/Ed25519) (Edwards-curve Digital Signature Algorithm)
The specific choice of curve is up to the user.
Recall, the size of the scalar field is a security parameter for the hash function, so the choice of curve itself does not impact security so long as the size is the same!

## Implementation
TODO: Add a snippet from our Poseidon crate here that shows what we are doing to get these fields.