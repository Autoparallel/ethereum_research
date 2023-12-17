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


### Maximum Distance Separable Matrix

Maximum Distance Seprarable Matrices(MDS) are commonly used in the linear layer of hash functions.
It's used in the linear mixing step of cipher algorithms to ensure a high level of *diffusion*, meaning that a change in a single input bit will affect many output bits. 
This contributes to the cipher's security by making it resistant to certain types of cryptanalytic attacks. 
MDS matrices are designed so that any subset of rows (or columns) forms a linearly independent set, maximizing the spread of input differences across the cipher's state.

The mathematical notion of linear independence refers to a set of vectors in which no vector can be represented as a linear combination of the others. 
This means that each vector adds a new dimension to the space spanned by the set. 
For example, in a three-dimensional space, three vectors are linearly independent if none of them lies in the plane formed by the other two. 
This concept is crucial in many areas of mathematics and is fundamental in understanding the behavior of systems of linear equations and transformations.

#### Cauchy Matrix
A good example of a maximum separable matrix is a Cauchy matrix, which is recommended to be used in the linear layer of Poseidon in the paper.
A Cauchy matrix is a special type of matrix with elements derived from the Cauchy formula, 
defined by $C_{ij} = \frac{1}{x_i - y_j}$, where $ x_i $ and $ y_j $ are elements 
from two distinct sets, and $ x_i \neq y_j $ for all $ i, j $. 
Cauchy matrices are known for their high determinant values and good error-correcting capabilities.
