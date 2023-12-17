# Construction
Poseidon has a few core building blocks that are used to construct the hash function.
Each of the building blocks have their own parameters, and the security of the hash function depends on the choice of these parameters.
Poseidon implementations are built up by stringing these building blocks in a a flexible way, but the suggested implementations in the paper make specific choices due to their security and efficiency.

The *Hades design strategy*, used in the Poseidon hash function, is a cryptographic approach that optimizes the trade-off between security and efficiency. 
It incorporates a mix of full and partial rounds within the hash function. 
Full rounds apply a non-linear operation (like an S-box) to all elements of the state, offering high security. 
Partial rounds, on the other hand, apply this operation to only a subset of the state elements, enhancing efficiency. 
This combination allows for maintaining strong cryptographic security while reducing the computational overhead typically associated with full rounds in every step. 
This design is particularly beneficial in contexts like zero-knowledge proofs, where efficiency is crucial without compromising security. 
The paper highlights how the choice of the MDS matrix in HADES significantly affects security against differential and linear attacks.


## Scalar Field

A scalar field in the context of is a set of elements, along with operations of addition and multiplication, that follows certain rules (like commutativity, associativity, distributivity, and the existence of additive and multiplicative identities). 
First, we choose a size $q$ of the finite field $\mathbb{F}_q$ to work with where $q=p^k$ for some prime $p$ and some integer $k$.
Note that Poseidon actually uses a finite field that arises as the scalar field of an elliptic curve, but for simplicity we will just think of it as a finite field. 
- This size $q$ will be an important security parameter for the hash function.

Unlike other hash functions, Poseidon does not require the input to be a string of bits and rather the input will be a string of elements in $\mathbb{F}_q$ which we will call *chunks*.
More on that can be found in the [scalar field section](./scalar_field.md).

## Sponge
Like other hash algorithms, Poseidon uses the *sponge* construction.
Sponges are a general way to read in a data stream at some rate, allow for mixing of the data within the capacity of the sponge, and then output a fixed-size hash of the data.
There are two important parameters that are used to construct the sponge that have impact to the security of the hash function:
- The *rate* $r$ which decides how many chunks of size $|\mathbb{F}_q|$ are "absorbed" into the sponge in each step;
- The *capacity* which decides how many chunks of size $|\mathbb{F}_q|$ are stored in the sponge in each step.

The higher the rate, the faster and cheaper the hash becomes, but this makes the hash less secure, too.
Intuitively, the larger the capacity, the more random state you allow yourself to store in the sponge in each step. 
We go deeper into the sponge and its use in general in the [sponge section](./sponge.md).

## Permutation
The sponge alone will not produce a secure hash function.
We also need to have the data be mixed inside the sponge in some way that is difficult to invert and reduces the chance of collisions.
Poseidon uses integer powers (on selected chunks in the sponge) to permute the state. 
For instance, we may go through each chunk of the sponge and apply the map $x \mapsto x^5$ for $x\in \mathbb{F}_p$ (which is an example of an S-Box).
We could also apply a linear transformation to the whole state of the sponge to do wider-scale mixing (which is an example of a linear layer).
The particular permutation used in Poseidon is a combination of two different types of operations: *S-Boxes* and *linear layers*. 
We will explain these in their own sections, but for more intuition on the Poseidon permutation techniques, see the [permutation section](./permutation.md).

### S-Boxes
*S-Boxes* are a common tool in cryptography that are used to introduce non-linearity into a system.
They come in many different forms, but the S-Boxes used in Poseidon are essentially a sequence of operations that are applied to each chunk in the sponge.
The S-Boxes can use different functions (e.g., $x^3$ or $x^5$) or be full or partial rounds. These choices all impact the security for the hash function.
That is, security of the hash function from the S-Box alone depends on the choice of:
- The function used in the S-Box (i.e., the exponent of the power function);
- Number of full rounds;
- Number of partial rounds.

For details on the s-boxes, see the [S-Boxes section](./s-boxes.md).

### Linear Layer
In order to prevent certain algebraic attacks, POSEIDON also implements another stage of mixing in the *linear layer*. 
These are essentially big matrices that are applied to all of the chunks that make up the state of the sponge (as if they are a vector).
In the paper, the suggested linear layer uses Cauchy matrices. 
Security of the hash function from the linear layer depends on the choice of:
- The number of rounds of the linear layer;
- The choice of matrix used in the linear layer.

For more details on the linear layer, see the [linear layer section](./linear_layer.md).

### Substitution-Permutation Networks
A Substitution-Permutation Network (SPN) is a method used in block cipher design. 
It involves a series of linked operations, including substitution (replacing bits with other bits) and permutation (rearranging bits). 
This structure is known for its effectiveness in creating confusion and diffusion, important principles in cryptography.

A Partial SPN (PSPN) differs from a full SPN in that the substitution step is not applied to all bits in each round. 
Instead, it substitutes only a subset of the bits, which can enhance efficiency while still maintaining a good level of security. 
This selective application of substitution is the key difference between PSPN and full SPN designs.