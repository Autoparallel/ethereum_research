# Construction

POSEIDON relies on a few different input parameters, each of which plays a role in the overall security of the hash as well as its speed and efficiency.
- First, we choose a size $p$ of the finite field $\mathbb{F}_p$ to work with.
Note that POSEIDON actually uses a prime field that arises as the scalar field of an elliptic curve, but for simplicity we will just think of it as a finite field. More on that can be found in the [scalar field section](poseidon/scalar_field.md).

## Sponge
POSEIDON builds off of the *sponge* construction which has two parameters to choose from at first.
- The *rate* $r$ which decides how many chunks of size $|\mathbb{F}_p|$ are "absorbed" into the sponge in each step;
- The *capacity* which decides how many chunks of size $|\mathbb{F}_p|$ are stored in the sponge in each step.
The higher the rate, the faster and cheaper the hash becomes. 
However, this makes the hash less secure, too.
Intuitively, the larger the capacity, the more random state you allow yourself to store in the sponge in each step. 
We go deeper into the sponge and its use in general in the [sponge section](poseidon/sponge.md).

## Permutation
The hash also requires that the data is permuted in some way that is extremely challenging (if possible) to invert.
POSEIDON uses integer powers (on selected chunks in the sponge) to permute the state. 
For instance, we may go through each chunk of the sponge and apply the map $x \mapsto x^5$ for $x\in \mathbb{F}_p$.
QUESTION: This surely has something to do with these powers being hard to invert in a finite field?4
For a more intuitive take on the POSEIDON permutation techniques, see the [permutation section](poseidon/permutation.md).

### S-Boxes
For details on the S-Bixes, see the [S-Boxes section](poseidon/s-boxes.md).

### Linear Layer
In order to prevent certain algebraic attacks, POSEIDON also implements another stage of mixing in the *linear layer*. 
These are essentially big matrices that are applied to the chunks in the sponge (as if they are a vector).
In the paper, the suggested linear layer uses Cauchy matrices.
For more details on the linear layer, see the [linear layer section](poseidon/linear_layer.md).
