# Linear Layer
The linear layer is a crucial component that complements the non-linear transformations (like those performed by S-boxes). 

### Purpose of linear layers in hashing

1. **Diffusion:**
   - The primary purpose of the linear layer in POSEIDON is to provide diffusion. 
   Diffusion, as defined by Claude Shannon, is the property of spreading the influence of a single input bit over many output bits.
   In practical terms, this means that changing one bit of the input should change many bits of the output, making the relationship between input and output complex and hard to predict.

2. **Efficiency:**
   - Linear transformations are generally more computationally efficient than non-linear ones. 
   In a hash function like POSEIDON, which is optimized for use in Zero-Knowledge Proofs (ZKPs), maintaining computational efficiency is crucial.

3. **Balancing Non-linearity and Linearity:**
   - While non-linear transformations provide security against certain types of cryptanalysis, a good cryptographic hash function also needs linear components for complete diffusion. 
   The linear layer ensures that the changes made by the non-linear layer (S-boxes) are spread throughout the entire state. 
   This is why it is good to use a linear layer in tandem with non-linear transformations like S-boxes.
