# Curriculum and Content

Zero-Knowledge (ZK) cryptography requires background knowledge in some mathematics.
The goal of this document is to provide a curriculum for learning the necessary mathematics to understand ZK cryptography and its primitives.
We will also provide a list of resources for learning the material.

## Background
We list here some of the basic background for ZK cryptography.


### Cryptography
Cryptography is the formal study of techniques for secure communication across insecure channels.
The goal of cryptography is to allow two parties to communicate securely even if an adversary can listen to all of their communications.
This can be explained intuitively by the following analogy:
Two parties want to send a message to one another, but they do not want any third party to see the message. 
Each party can write a message on a piece of paper, put the paper in a box, and lock the box with a padlock.
If the other party is the only one with the key to the padlock, then the message is secure.
However, sharing the key poses a problem and there are many ways to pick locks!

### Proofs
The idea of mathematical proofs is the first stepping stone to understanding implications of ZK cryptography.
A proof is a string of facts (or axioms) that lead to a conclusion. 
For instance, we may want to prove that we are the owner of a certain online account.
One way to do this, formally, would be to use this account to make a signature on a message.
This proves that we have access to the account, and therefore we are the owner.
Of course, the above can be easily forged by anyone who has access to the account.
Another example of a proof would be showing that an entity is a member of a certain group (not necessarily a mathematical group).

## Mathematics
Underlying cryptography is a lot of mathematics but it is mathematics that many of us are familiar with.
For example, we talk about prime numbers, factorization, and modular arithmetic.
We can also build more involved structures on these such as polynomials.