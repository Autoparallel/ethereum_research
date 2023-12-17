# Permutation
We've seen that Poseidon uses a sponge construction to hash data, but the sponge alone is not enough to ensure security.
This is a common theme in cryptography: we often need to add additional layers of complexity to ensure that our constructions are secure.
Where one layer has a weakness, another layer can make up for it.
The permutation is one such layer that Poseidon uses to ensure security.

## Permutation is like a blender

Imagine the Poseidon permutation as a process similar to using a kitchen blender to mix ingredients together.
We can think of this in the context of the sponge construction, where the permutation is applied to the sponge's state to mix the input data with the existing state.

1. **Initial Setup (State Initialization):**
   - Think of foods and liquids inside of the blender's jar as a represention of the internal state of the hash function. 
   When it's not blended, the ingredients are still separate and distinct, but once blended, they become a single mixture and are no longer distinguishable.
   The unblended mixture corresponds to the initial state in the Poseidon hash function.
    - **Blender Jar (Sponge Capacity):**
        - This blender jar  effectively represents the internal state of the sponge, which is called its "capacity." This capacity includes both the part of the state that interacts with the input (like absorbing ingredients) and the part that does not.
        - In cryptographic terms, the capacity is crucial for security. It's like having a larger jar that ensures even after adding and mixing ingredients, there's still space left that never directly interacts with the ingredients you add later. This 'unused space' in the jar is what guarantees the security of the hash function.

2. **Adding Ingredients (Input Absorption):**
   - Each piece of data to be hashed is akin to adding another ingredient to the blender's jar. 
   In Poseidon, this involves integrating the input data into the internal state.
   - **Amount of Food (Sponge Rate):**
        - The amount of food you can add at once to the blender is like the "rate" of the sponge. It's the portion of the sponge's capacity that directly interacts with the input data (or ingredients).
        - In the sponge construction, you can only absorb (or add) a certain amount of data in each round, just like you can only add a certain amount of food to the blender at once without overfilling it.

3. **Blending Process (Permutation Rounds):**
   - The heart of the POSEIDON permutation is akin to the blending process. Here, the blender goes through several rounds of blending, each round mixing the ingredients thoroughly.
   - In POSEIDON, each round consists of a series of mathematical operations (like stirring, chopping, or grinding in a blender). These operations include:
     - **Linear Mixing:** Similar to a gentle stir or a slow blend, ensuring that all parts of the mixture are evenly combined.
     - **Non-linear Transformation (S-Boxes):** This is like a high-speed blend or pulse, creating complex interactions among the ingredients. It's where the non-linear S-boxes come into play, introducing complexity and security.
     - **Fixed Permutation Pattern:** Just like following a specific sequence in blending (like pulse, blend, stir), POSEIDON follows a fixed pattern of mixing and transforming in each round.
   - **Blending/Pulsing (Permutation):**
        - Once you've added food to the blender (absorbed data into the rate part of the sponge), you start the blending or pulsing. This is analogous to applying the permutation in the sponge construction. This step thoroughly mixes the contents (data) with the existing state.
        - The permutation (blending) ensures that the data is uniformly and complexly mixed, contributing to the hash function's security.

4. **Repeating the Rounds:**
   - Just as some recipes call for you to "blend until smooth", POSEIDON repeats its blending (permutation) rounds a set number of times to ensure thorough mixing and security.

5. **Final Product (Hash Output):**
   - After the final round of blending, the mixture in the blender represents the transformed state. In the context of the full sponge construction, part of this state is then 'squeezed out' to produce the hash output.
   - **Satisfaction and Emptying (Squeezing):**
        - When you're satisfied with the blending (after sufficient permutations), you pour out the mix from the blender. In the sponge construction, this is akin to the squeezing phase, where part of the state (the equivalent of the mix you pour out) is read out as the hash result.
        - If your desired hash output is larger than what you can pour out in one go, you would blend again (apply another permutation) and pour out more, just like making multiple servings from a blender by blending and pouring in stages.
