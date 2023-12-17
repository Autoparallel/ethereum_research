# Curriculum and Content

This Mdbook will present the knowledge required to understand the security assumptions of algebraic hash functions, particularly Poseidon.

Poseidon is commonely used in Zero-Knowledge (ZK) cryptography both of which require background knowledge in some mathematics.
This document aims to provide a curriculum for learning the necessary mathematics to understand cryptography and its primitives.
We will also provide a list of resources for learning the material.

## Background
We list here some of the essential background for hash functions and cryptography.

### Cryptography
Cryptography is the formal study of techniques for secure communication across insecure channels.
Cryptography aims to allow two parties to communicate securely, even if an adversary can listen to all of their communications.
Secure communication can be explained intuitively by the following analogy:
Two parties want to send a message to one another but do not want any third party to see the message. 
Each party can write a message on a piece of paper, put the paper in a box, and lock the box with a padlock.
The message is secure if the other party is the only one with the key to the padlock.
However, sharing the key poses a problem; there are many ways to pick locks!

### Proofs
The idea of mathematical proofs is the first stepping stone to understanding the implications of ZK cryptography.
A proof is a string of facts (or axioms) that lead to a conclusion. 
For instance, we may want to prove that we own a particular online account.
One way to do this would be to use this account to make a signature on a message.
A signature proves that we have access to the account and are the owner.
Of course, anyone with access to the account can forge a signature.
Another example of a proof would be showing that an entity is a member of a particular group (not necessarily a mathematical group).
For example, say an individual wants to prove they are over 21 without revealing their age or birthday or in other words that they are member of the group of people over 21.

### Hash Functions
Hash functions are a fundamental primitive of cryptography. Hash functions are used for data integrity and often in combination with digital signatures. With a good hash function, even a 1-bit change in a message will produce a different hash (on average, half of the bits change). A message is hashed with digital signatures, and then the hash itself is signed. Hash functions must have the following properties if they are to be considered secure:

- **Deterministic**: The same input always produces the same output.
- **Pre-image resistant**: Given a hash, finding a message that hashes to that value is computationally infeasible. In other words, the hash is invertible.
- **Collision resistant**: Finding two messages that hash to the same value is computationally infeasible.

The following properties are not required for security but are desirable in specific applications:

- **Efficient**: The hash function should be efficient to compute.
- **Fixed output size**: The hash function should produce a fixed-size output.

#### Applications
Hash functions, serving as a one way function, play a crucial role in various applications, including:

- **Data Integrity**: Hash functions ensure data integrity by generating unique hash values for different data inputs. Any alteration in the data, however minor, results in a different hash value, indicating a breach in data integrity.

- **Password Storage**: Hash functions store passwords securely. Instead of storing the actual password, the hash of the password is stored in what is called the *shadow file*. The entered password is hashed and compared with the stored hash value when authentication is required.

- **Digital Signatures**: Hash functions are integral to creating digital signatures. The data is hashed, and the hash value is encrypted using a private key to create a digital signature. This ensures the authenticity and integrity of the data.

Hash functions also play a critical role in the software engineers' toolbox. For example, a hashmap is a data structure that uses a hash function to map keys to values. This allows for efficient lookup of values given a key, which can improve the performance of algorithms and increase user experience. Another example would be a merkle tree which is often used to represents the integrity of relationships between data. 

### Algebraic hash functions
Algebraic hash functions are a particular class of hash functions used in ZK cryptography. Historically, most hash functions have output values that are binary strings. On the other hand, Algebraic hash functions produce values that are elements of a finite field. This allows for more efficient computation of the hash function and more efficient proving times for ZK proofs.

### Mathematics
Underlying cryptography is much mathematics, and as a result, some mathematical background is essential.
For example, we discuss prime numbers, factorization, and modular arithmetic.
We can also build more involved structures on these, such as polynomials.
We will lean towards discussing and evaluating Posiedon in the context of algebra and geometry as these views shed light on the additional structure that algebraic hash functions have and are less explored at large. 