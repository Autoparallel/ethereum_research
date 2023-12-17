# Resources
Throughout our research, we have found many resources that have helped us understand the concepts behind Poseidon and the field of symmetric cryptography. We have compiled a list of these resources here, and we hope that they will be useful to you as well.

# Papers and books

## Books
> [Silverman, J. H., Pipher, J., & Hoffstein, J. (2008). An introduction to mathematical cryptography (Vol. 1). Springer New York.](https://github.com/isislovecruft/library--/blob/master/cryptography%20%26%20mathematics/An%20Introduction%20to%20Mathematical%20Cryptography%20(2014)%20-%20Hoffstein%2C%20Pipher%2C%20Silverman.pdf)
"An Introduction to Mathematical Cryptography" by Hoffstein, Pipher, and Silverman is a highly valuable resource for those seeking a deep understanding of the mathematical principles in cryptography. 
Its thorough coverage of both classical and modern cryptographic techniques, combined with a clear explanation of underlying mathematical concepts, makes it great introductory matterial. 
The book strikes a balance between theory and practice, offering insights into the practical applications of the discussed mathematical methods. 
Its comprehensive approach ensures that readers gain a solid foundation in cryptographic principles.

>[Rosulek, M. (n.d.). The Joy of Cryptography.](https://joyofcryptography.com)
"The Joy of Cryptography" is an accessible, undergraduate-level textbook that provides a comprehensive introduction to the principles of modern cryptography. 
The author, Mike Rosulek, emphasizes a deep understanding of the subject, focusing on provable security aspects and offering insight into how cryptographic protocols are constructed to resist various attacks. 
The book is known for its clear explanations and practical examples, making complex concepts approachable. 
It's particularly useful for students and practitioners who wish to gain a solid grounding in cryptography without getting overwhelmed by overly technical details. 

## Papers

>[Gabizon, A., Williamson, Z. J., & Ciobotaru, O. (2019). Plonk: Permutations over lagrange-bases for oecumenical noninteractive arguments of knowledge. Cryptology ePrint Archive.](https://eprint.iacr.org/2019/953.pdf)
"PlonK: Permutations over Lagrange-bases for Oecumenical Noninteractive arguments of Knowledge" by Ariel Gabizon, Zachary J. Williamson, and Oana Ciobotaru, focuses on enhancing zk-SNARK constructions, specifically addressing the efficiency of proof construction and verification. 
It presents a universal SNARK with fully succinct verification and significantly reduced prover running time. 
The paper emphasizes improvements in arithmetization, utilizing permutations over univariate evaluations on a multiplicative subgroup. 
This approach offers a more direct and efficient verification process compared to previous models like Sonic, making it practical for a wider range of real-world applications in cryptography.

>[Grassi, L., Khovratovich, D., Rechberger, C., Roy, A., & Schofnegger, M. POSEIDON: A New Hash Function for Zero-Knowledge Proof Systems (Updated Version).](https://eprint.iacr.org/2019/458.pdf)
"POSEIDON: A New Hash Function for Zero-Knowledge Proof Systems" by Grassi, Khovratovich, Rechberger, Roy, and Schofnegger, presents a modular framework for cryptographic hash functions optimized for zero-knowledge proof systems like SNARKs and STARKs. 
It introduces the POSEIDON hash function, designed to work efficiently with large prime fields and aims to be more constraint-efficient than existing hash functions like Pedersen Hash. 
The paper details the security analysis of POSEIDON, its applications in various cryptographic systems, and comparisons with other hash functions. 

>[Sauer, J. F., & Szepieniec, A. (2021). Sok: Gröbner basis algorithms for arithmetization oriented ciphers. Cryptology ePrint Archive.](https://eprint.iacr.org/2021/870.pdf)
"SoK: Gröbner Basis Algorithms for Arithmetization Oriented Ciphers" by Jan Ferdinand Sauer and Alan Szepieniec provides an in-depth study of Gröbner basis algorithms in the context of arithmetization-oriented ciphers. 
It surveys key algorithms, emphasizing their intuitive understanding and complexity analysis, aiding in assessing the security of these ciphers. In my opinion, this is the best resource on Gröbner bases attacks.

>[Guido, B., Joan, D., Michaël, P., & Gilles, V. A. (2011). Cryptographic sponge functions.](https://keccak.team/files/CSF-0.1.pdf)
"Cryptographic Sponge Functions" offers an extensive exploration of sponge functions in cryptography. 
It details their fundamentals, applications, and security aspects. 
The document is notable for its thorough coverage, practical application discussions, security analysis, and accessibility to readers with basic cryptography knowledge. 
It's a valuable resource for understanding the role of sponge functions in cryptographic systems, catering to both theoreticians and practitioners in the field.

>[Beierle, C., Canteaut, A., Leander, G., & Rotella, Y. (2017, July). Proving resistance against invariant attacks: How to choose the round constants. In Annual International Cryptology Conference (pp. 647-678). Cham: Springer International Publishing.](https://eprint.iacr.org/2017/463.pdf)
"Proving Resistance against Invariant Attacks: How to Choose the Round Constants," focuses on the resistance of lightweight block ciphers against invariant attacks, specifically addressing the selection of round constants in cipher design. 
It provides a detailed analysis of how the linear layer and round constants influence resistance to such attacks, using examples like Prince, Skinny-64, and Mantis7.
The authors propose a framework to analyze and prove resistance, emphasizing the mathematical properties critical for ensuring security against invariant attacks. 
They also offer practical guidelines for choosing round constants and analyze various ciphers' resistance levels.

>[Zhang, H., & Wang, X. (2009). Cryptanalysis of stream cipher grain family. Cryptology ePrint Archive.](https://eprint.iacr.org/2009/109.pdf)
"Cryptanalysis of Stream Cipher Grain Family," examines vulnerabilities in the Grain family of stream ciphers, including Grain v0, Grain v1, and Grain-128. 
The authors present a distinguishing attack against weak Key-IV (Initialisation Vector) combinations in these ciphers, using the second Walsh spectra of nonlinear functions to identify weak Key-IV pairs. 
They demonstrate that these weak Key-IVs can be distinguished with relatively few keystream bits and moderate computational effort. 
Additionally, they apply algebraic attacks to recover the secret key for these weak Key-IVs efficiently. 
The paper highlights significant security concerns in the Grain family, particularly in the context of weak Key-IV pairs, and suggests improvements for their cryptographic robustness.
