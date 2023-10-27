# Grant Scope
Zero-Knowledge (ZK) cryptography uses a variety of cryptographic primitives to achieve its goals. These primitives are often based on new and novel mathematics. As a result, the security of these primitives is not as well understood as the security of more traditional primitives like RSA and AES.

For example, we rely on the following:
- Security of hash functions.
- Security of polynomial commitments.
These two concepts will be our main focus for this grant.
    
Operating on the premise that ZK will bring massive changes to the world, not just for blockchains but also for the broader technical infrastructure of the world. For example, with applications like privacy preserving machine learning, and MPC, there are boundless benifits of contributing to the security of these cryptographic primitives. However, unless these chemes are proven to be secure, they will not be useful let alone adopted.

Polynomial commitments are a very important primitive for ZKPs. One of the most important schemes is [FRI](https://drops.dagstuhl.de/opus/volltexte/2018/9018/pdf/LIPIcs-ICALP-2018-14.pdf) but we make a lot of security assumptions for FRI.
We aim to create some evidence for or against the security of algebraic hash functions and FRI.

## Stages
We propose two stages for the grant. An initial learning stage where the expectations are to put together a practical guide to understanding the security assumptions in FRI and reed soloman encodings as well as a practical guide to algebraic hash functions and grobner basis. We plan to allocate 1 day per week, and anticipate the timeline of this stage to be between 1 and 3 months.

Some examples of technical material covered would be 
- Small practical write up of Reed-Solomon codes
- Small practical write up on a Grobner bases

Furthermore we will aim to cover and more deeply understand and document
- What are the security assumptions in FRI
- What are the security assumption in Tiny-Sponge / Poseidon?

### First stage is understanding period (1-3month):
- Become experts in Reed-Solomon encodings / Grobner bases 
- Understand security assumptions in FRI and Algebraic Hash Functions
Second Stage(1-3month)

## Deliverables for first stage
- Practical write up of Reed-Soloman codes
- Practical Write up on Grobner bases
Would be great for these give gentle overviews on topics like polynomials over finite fields and ideals.
I think it would also be a good goal to provide some code examples to bring the theory into application.

