# Scalar Field

POSEIDON uses a scalar field from an elliptic curve, as opposed to a standard finite field representation like $\Z/p\Z$  in order to optimize for cryptographic/ZK protocols

Many ZKP systems, especially zk-SNARKs, are built around elliptic curve cryptography (ECC). Using a scalar field derived from the same elliptic curve ensures that all operations are in the same field, which simplifies and optimizes computations.
These come in the form of operations such as the [pairing computations](https://medium.com/@VitalikButerin/exploring-elliptic-curve-pairings-c73c1864e627), which are more efficiently conducted in the scalar field of an elliptic curve. 
Scalar fields from elliptic curves often provide stronger security properties for a given field size compared to standard finite fields. 
This is due to the more complex structure of elliptic curves, which can increase the difficulty of certain cryptographic attacks.

For a given level of security, elliptic curves can often use smaller key sizes compared to algorithms based on standard finite fields. 
This results in better performance and smaller memory usage, which is particularly advantageous in constrained environments.
Since many modern cryptographic systems use elliptic curves, designing a hash function that operates in the same domain can facilitate better interoperability between different cryptographic primitives and protocols.
