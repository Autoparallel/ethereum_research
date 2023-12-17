# Blogs and Code

## Blogs
>[Khovratovich, D., Aumasson, J. P., Quine, P., & Mennink, B. (n.d.). SAFE (Sponge API for Field Elements) – A Toolbox for ZK Hash Applications. HackMD.](https://hackmd.io/bHgsH6mMStCVibM_wYvb2w)
The authors define a unified Sponge API for Field Elements (SAFE), which provides ZK proof systems designers with a secure and efficient framework for hashing, encryption, and applications thereof (commitment schemes, Fiat-Shamir transforms, AEAD, and so on). 
The authors do not restrict the permutation algorithm nor the field type, thus SAFE can be instantiated with established constructions.
The API supports various permutations and applications like hashing, Fiat-Shamir transforms, and authenticated encryption, but excludes variable-length hashing due to complexity and performance issues. 

>[Fichtner, J. (2021, June 28). Gröbner Basis-Attacking a Tiny Sponge. ](https://jfs.sh/blog/)
I personally recommend this blog post by Jannik Fichtner, which provides a detailed explanation of Gröbner basis attacks on the Poseidon hash function. 
The blog at large covers advanced topics in cryptography and mathematics, focusing on areas like Gröbner bases, sponge functions, and Hilbert regularity. 
The posts delve into technical aspects and provide insightful analyses, often exploring the intersection between theoretical concepts and practical cryptographic applications.

## Code

>[lurk-lab. (2023). Neptune: Rust Poseidon implementation. GitHub.](https://github.com/lurk-lab/neptune)
This is a well done production implementation of Poseidon in Rust. We initially really wanted to try to attack this implementation however neptune doesn't support field sizes smaller than 32bits and we wanted to start with a smaller field size.

>[ingonyama-zk. (2023). Poseidon-hash: Reference implementation in Python of Poseidon and optimized Poseidon (Neptune) hash functions. GitHub.](https://github.com/ingonyama-zk/poseidon-hash?tab=readme-ov-file)
This refrerence implementation of Poseidon in Python is a great resource for understanding the algorithms involved in Poseidon. The repository is well documented and easy to follow. It has a good test sweet and proves to be a gentle introduction to understanding the Poseidon hash function.

>[arnaucube. (2023). Poseidon-rs: Poseidon hash function. GitHub.](https://github.com/arnaucube/poseidon-rs)
This is another implementation of Poseidon in Rust. While it is less documented it is incredibly simple making it a really good place to start with rust. The whole implementation is contained in 2 small files. 