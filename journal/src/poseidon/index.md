# Poseidon

## Intro
Poseidon is an example of an *algebraic* hash function.
Succinctly, this means that the operations used in the hash involve addition and multiplication of elements in a finite field of order $p$, $\mathbb{F}_P$.
This differs from non-algebraic hash functions that usually operate over bits (which, indeed, can be seen as $\mathbb{F}_2$) and use non-algebraic operations such as $\mathtt{XOR}$.


## Reasoning
A big reason why algebraic hash functions are important today is their use in ZK/*ARK proving systems.
These systems rely on *arithmetization*. 
In essence, arithmetization takes a computer algorithm and rewrites it in terms of a sequence polynomial operations which we can call *arithmetic circuits*.
Operations such as $\mathtt{XOR}$ are fundamentally more difficult to arithmetize than their algebraic counterparts. 
This is because arithmetic circuits are built from algebraic operations over finite fields.

Hashing is an extremely important operation in computing since it provides a means of having a "random oracle". 
That is, a perfect hash function is one that maps given input data into completely random output data (hence the random oracle). 
These random oracles are not only needed in computation (e.g., for mappings and other collections, data validation, compression, etc.) but are also used implicitly to take interactive arguments and convert them into non-interactive arguments (this is Shamir's trick).

