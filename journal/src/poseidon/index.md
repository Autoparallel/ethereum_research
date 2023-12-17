# Poseidon
Poseidon is an example of an *algebraic* hash function.
Succinctly, this means that the operations used in the hash involve addition and multiplication (i.e., algebraic manipulation) of elements in a finite fields as opposed to bitwise operations which are present in other hashing algorithms.
Poseidon itself was introduced in the paper: [Poseidon: A New Hash Function for Zero-Knowledge Proof Systems](https://eprint.iacr.org/2019/458.pdf) by Grassi et al.. 
It presents a new cryptographic hash function, Poseidon, optimized for use in practical computational integrity proof systems like SNARKs, STARKs, and Bulletproofs. 
The paper describes the design, implementation, and security analysis of Poseidon, highlighting its efficiency in zero-knowledge (ZK) proof systems, particularly in scenarios requiring the proving of preimage knowledge under a hash function. 
The authors focus on the hash function's modular framework, efficiency in large prime fields, and its comparative advantage over existing functions like SHA-256 and Pedersen Hash in terms of computational cost. 
Additionally, the paper details the cryptanalysis of Poseidon, emphasizing its resilience to various types of attacks, and demonstrates its practical applications in systems like Merkle trees and zero-knowledge proofs.

## Reasoning
Hash functions are ubiquitous in modern computing and cryptography, see the [curriculum section](../overview/curriculum.md) for more details.
At a high level, hashing is an extremely important operation in computing since it provides a means of having a "random oracle" which is a function that maps input data to output data in a completely "random" way.
These random oracles are not only needed in computation (e.g., for mappings and other collections, data validation, compression, etc.) but are also used implicitly to take interactive arguments and convert them into non-interactive arguments (this is Shamir's trick).
The reality is that we need hash functions for a lot of things, and we need them to be fast and secure.
By secure, we mean that they are hard to invert (i.e., find preimages) and hard to find collisions (i.e., find two inputs that hash to the same output).
This comes down the quality of the hash function's output distribution or randomness, which is a function of the hash function's design.

The main reason why algebraic hash functions, in particular over their non-algebraic counterparts, are important today is their use in ZK proving systems.
These systems rely on *arithmetization*. 
In essence, arithmetization takes a computer algorithm and rewrites it in terms of a sequence polynomial operations which we can call *arithmetic circuits*.
Operations such as $\mathtt{XOR}$ are fundamentally more expensive to arithmetize than their algebraic counterparts like addition and multiplication.
This is because arithmetic circuits are built from algebraic operations over finite fields that we previously mentioned.

## Reading the paper
If you decide to read the Poseidon paper [Poseidon: A New Hash Function for Zero-Knowledge Proof Systems](https://eprint.iacr.org/2019/458.pdf), then here are some notes to help you along the way.

### Design of POSEIDON
- **Architecture**: Describes the unique construction of Poseidon, focusing on its efficiency and security.
Security is outlined in terms of the parameters available to the designer, including the number of rounds, the size of the finite field, the algebraic properties of the S-boxes, and the sponge capacity and rate.
We cover these details in the sections of this chapter.
- **Comparison**: Compares Poseidon with existing hash functions, highlighting its advantages.

### Security Analysis
- **Resilience**: Discusses the cryptographic strength of Poseidon against various attack vectors.
- **Cryptanalysis**: Provides a detailed analysis of the hash function's resistance to common cryptographic attacks.

These are the most important sections of the paper, for us. 
We are very interested in the security of the hash function and its resistance to attacks since this is a fundamental tool used in present day ZK proof systems.
The advantages of Poseidon are meaningless if the has function is not secure!

### Implementation and Performance
- **Practical Application**: Details how Poseidon is implemented in systems like SNARKs and Merkle trees.
- **Efficiency**: Emphasizes the computational efficiency of Poseidon, especially in proving preimage knowledge.

These results highlight the practicality of Poseidon in real-world applications.
If it weren't for the efficiency of Poseidon, then it would not be a good candidate for use in ZK proof systems and we could continue to use other hash functions like SHA-256.