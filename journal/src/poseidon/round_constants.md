# Round Constants
Round constants in hash functions, including those that use the sponge construction, serve several important purposes:

1. **Symmetry Breaking:** Round constants are used to break the symmetry of the state during the permutation process. 
Without these constants, each round would be identical, and certain patterns or structures in the input could propagate through the rounds unchanged. 
The constants ensure that each round is unique, preventing these patterns from surviving the hashing process.

2. **Avoiding Fixed Points:** A fixed point is when a certain input to the hash function results in the same output (e.g., $\texttt{hash}(x) = x$). 
Round constants help to avoid such scenarios, which could weaken the hash function's security.
<!-- A good term that shows up in dynamical systems for periodic attractors! -->

3. **Preventing Slide Attacks:** In cryptography, a slide attack is a type of attack that takes advantage of repeating patterns within the rounds of a cryptographic algorithm.
Round constants mitigate this risk by ensuring that each round of the permutation is distinct.

4. **Diffusion:** They contribute to the diffusion property of the hash function by ensuring that the input bits are spread across the resulting hash value, making the output look random.

5. **Resisting Cryptanalysis:** By carefully choosing the round constants, designers can protect against specific cryptanalytic attacks that exploit the mathematical structure of the permutation function.

TODO: Apparently these are for resistance against invariant attacks? One of the citations of the paper: Christof Beierle, Anne Canteaut, Gregor Leander, and Yann Rotella. Proving Resistance Against Invariant Attacks: How to Choose the Round Constants. In CRYPTO 2017, volume 10402 of LNCS, pages 647–678, 2017.

TODO: Talk about slide attacks? https://en.wikipedia.org/wiki/Slide_attack