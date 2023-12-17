# Diving into Poseidon hash and its security
The purpose of this work is to document a thorough understanding of the Poseidon hash function and its security. 
This document is a work in progress and will be updated as we and the community learn more about the topic.

As a forewarning, this document is a continual work in progress.
Material in here is subject to change, there may be errors, and there may be things that are not fully explained.
We would love to hear your feedback and suggestions for improvement and are happy to answer any questions you may have.
Community engagement, open source development, and collaborative learning are core values for us, so please reach out!

## Contents

- **Overview**
    - **Curriculum**: Providing you with a list of relevant topics that will come up with Poseidon.
    - **Goals**: Providing you with a list of goals that we want to achieve with this repository.
- **Poseidon**
    - **Construction**: Explaining the construction of the Poseidon hash function at a high level.
    - **Scalar Field**: Explaining how scalar (finite) fields are used in Poseidon.
    - **Sponge**: Explaining the sponge construction used in Poseidon both intuitively and formally.
    - **Permutation**: Explaining the permutation used in Poseidon intuitively.
    - **S-Boxes**: Explaining the S-Boxes used in Poseidon formally and comparing to techniques used in other hashes.
    - **Linear Layer**: Explaining the linear layer used in Poseidon formally.
    - **Round Constants**: Explaining the round constants used in Poseidon formally.
- **Security**
    - **Linear Cryptanalysis**: Explaining linear cryptanalysis and how it can be used to attack Poseidon.
    - **Differential Cryptanalysis**: Explaining differential cryptanalysis and how it can be used to attack Poseidon.
    - **Algebraic Cryptanalysis**: Explaining algebraic cryptanalysis and how it can be used to attack Poseidon.
    - **New Thoughts**: Explaining new thoughts on cryptanalysis of Poseidon.

## Poseidon crate
We are also developing our own hand-baked implementation of Poseidon in Rust.
This is a work in progress, but the code is located in `./poseidon` and can be built via `cargo build`.

### Purpose
To understand something well, it is often helpful to implement it yourself.
Plus, we are strong believers that researchers should be able to implement their own ideas and test them out.
Researchers can be builders, and builders can do research (and love math!).

### Intention
The idea is to implement Poseidon in Rust and then use it to test out different ideas and attacks.
The production versions of Poseidon should also be tested in their own right, but we plan to test the methodology of Poseidon and, given that, it is nice to make Poseidon less secure to demonstrate how to find collisions, find preimages, and orchestrate attacks all in one place and without it taking an unbelievable amount of time. 
Keep in mind, searching through the space of all possible hashes is a very large space, so even with a noticeably less secure version of Poseidon, it will still take a long time to find collisions and preimages!