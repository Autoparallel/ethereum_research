# Sponge
Sponges are used in cryptography to absorb data of any size and produce a fixed-size output. 
They are used in hash functions, pseudo-random number generators, stream ciphers, and authenticated encryption schemes.
More can be found at this [link](https://en.wikipedia.org/wiki/Sponge_function).

## Why we call it a sponge

**The Sponge (State) Idea:**
Think of the sponge as having two action: it first absorbs input data like a sponge absorbs liquid, and subsequently this data gets squeezed to produce output data like a sponge releases liquid. 
In cryptographic terms, this sponge is the internal **state** of the hash function.

1. **Absorbing Phase:**
   - Data such as a message, a file, or a number is divided into blocks, for instance, of a given number of bits or the size of a finite field.
   - Each block of your input data is "absorbed" into the sponge where it is then mixed with the residual data (like the residual "liquid") in the sponge. 
   The absorption and mixing process are defined concretely by the rate and capacity of the sponge respectively.
   This mixing process can be thought of like absorbing some new blue colored water into one chunk of the sponge and some new red in another while there is already some green soaking in the rest of the sponge. 
   Fitting to the analogy, to truly mix these, the sponge would be allowed to sit where diffusion of the colors would occur.
   In the cryptographic case, we use a permutation to mix the data like diffusion can mix the colors.

2. **Squeezing Phase:**
   - Once the sponge has absorbed all your data (or all the colored water in the analogy), it's time to get the hash output, similar to squeezing water out of the sponge.
   - In the squeezing phase, part of the sponge's state is read out as the hash result. 
   If the desired hash output is longer, the sponge might be "squeezed" multiple times, with additional transformations in between to ensure security.
   If it is shorter, the sponge is squeezed only once, and the output is truncated to the desired length.
   By squeezing the sponge, we are essentially wringing out the water, leaving us with a fixed amount of water (the hash) that is a mix of all the colors (the input data).
   The blue, red, and green water in our analogy now becomes a single murky brownish purplish color, which is the hash output.
   Could you ever get the original colors back from this let alone the order they were added in?

### Why are sponges convenient?

1. **Flexibility with Input Sizes:**
   - The sponge construction can handle input data of any size. 
   It absorbs data block by block until all data is processed.

2. **Fixed Output Size:**
   - The hash function produces outputs of a fixed size, regardless of the input size.

3. **Security:**
   - The mixing and transformation processes in the sponge ensure that even a small change in the input (like adding a drop of differently colored water) leads to a completely different and unpredictable output, which is a crucial aspect of secure hash functions.