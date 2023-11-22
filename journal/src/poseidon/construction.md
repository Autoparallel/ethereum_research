# Construction

POSEIDON relies on a few different input parameters, each of which plays a role in the overall security of the hash as well as its speed and efficiency.
- First, we choose a size $p$ of the finite field $\mathbb{F}_p$ to work with.

## Sponge
POSEIDON builds off of the *sponge* construction which has two parameters to choose from at first.
- The *rate* $r$ which decides how many chunks of size $|\mathbb{F}_p|$ are "absorbed" into the sponge in each step;
- The *capacity* which decides how many chunks of size $|\mathbb{F}_p|$ are stored in the sponge in each step.
The higher the rate, the faster and cheaper the hash becomes. 
However, this makes the hash less secure, too.
Intuitively, the larger the capacity, the more random state you allow yourself to store in the sponge in each step. 

## Permutation
The hash also requires that the data is permuted in some way that is extremely challenging (if possible) to invert.
POSEIDON uses integer powers (on selected chunks in the sponge) to permute the state. 
For instance, we may go through each chunk of the sponge and apply the map $x \mapsto x^5$ for $x\in \mathbb{F}_p$.
QUESTION: This surely has something to do with these powers being hard to invert in a finite field?

## Linear Layer
In order to prevent certain algebraic attacks, POSEIDON also implements another stage of mixing in the *linear layer*. 
These are essentially big matrices that are applied to the chunks in the sponge (as if they are a vector).
In the paper, the suggested linear layer uses Cauchy matrices.