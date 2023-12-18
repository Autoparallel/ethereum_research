# Papers

>[Beierle, C., Canteaut, A., Leander, G., & Rotella, Y. (2017, July). Proving resistance against invariant attacks: How to choose the round constants. In Annual International Cryptology Conference (pp. 647-678). Cham: Springer International Publishing.](https://eprint.iacr.org/2017/463.pdf)
"Proving Resistance against Invariant Attacks: How to Choose the Round Constants," focuses on the resistance of lightweight block ciphers against invariant attacks, specifically addressing the selection of round constants in cipher design. 
It provides a detailed analysis of how the linear layer and round constants influence resistance to such attacks, using examples like Prince, Skinny-64, and Mantis7.
The authors propose a framework to analyze and prove resistance, emphasizing the mathematical properties critical for ensuring security against invariant attacks. 
They also offer practical guidelines for choosing round constants and analyze various ciphers' resistance levels.

>[Costello, C. (2012). Pairings for beginners. Online: https://www. craigc ostello. com. au/s/PairingsForBeginners. pdf.](https://static1.squarespace.com/static/5fdbb09f31d71c1227082339/t/5ff394720493bd28278889c6/1609798774687/PairingsForBeginners.pdf)
This book reads more like a novel than a textbook.
The author is empathetic and provides a lot of context for the reader while maintaining a high level of rigor.
For beginers or experts, this book is a rigorous resource learning about pairings.

>[Gabizon, A., Williamson, Z. J., & Ciobotaru, O. (2019). Plonk: Permutations over lagrange-bases for oecumenical noninteractive arguments of knowledge. Cryptology ePrint Archive.](https://eprint.iacr.org/2019/953.pdf)
"PlonK: Permutations over Lagrange-bases for Oecumenical Noninteractive arguments of Knowledge" by Ariel Gabizon, Zachary J. Williamson, and Oana Ciobotaru, focuses on enhancing zk-SNARK constructions, specifically addressing the efficiency of proof construction and verification. 
It presents a universal SNARK with fully succinct verification and significantly reduced prover running time. 
The paper emphasizes improvements in arithmetization, utilizing permutations over univariate evaluations on a multiplicative subgroup. 
This approach offers a more direct and efficient verification process compared to previous models like Sonic, making it practical for a wider range of real-world applications in cryptography.

>[Grassi, L., Khovratovich, D., Rechberger, C., Roy, A., & Schofnegger, M. POSEIDON: A New Hash Function for Zero-Knowledge Proof Systems (Updated Version).](https://eprint.iacr.org/2019/458.pdf)
"POSEIDON: A New Hash Function for Zero-Knowledge Proof Systems" by Grassi, Khovratovich, Rechberger, Roy, and Schofnegger, presents a modular framework for cryptographic hash functions optimized for zero-knowledge proof systems like SNARKs and STARKs. 
It introduces the POSEIDON hash function, designed to work efficiently with large prime fields and aims to be more constraint-efficient than existing hash functions like Pedersen Hash. 
The paper details the security analysis of POSEIDON, its applications in various cryptographic systems, and comparisons with other hash functions. 

>[Guido, B., Joan, D., Michaël, P., & Gilles, V. A. (2011). Cryptographic sponge functions.](https://keccak.team/files/CSF-0.1.pdf)
"Cryptographic Sponge Functions" offers an extensive exploration of sponge functions in cryptography. 
It details their fundamentals, applications, and security aspects. 
The document is notable for its thorough coverage, practical application discussions, security analysis, and accessibility to readers with basic cryptography knowledge. 
It's a valuable resource for understanding the role of sponge functions in cryptographic systems, catering to both theoreticians and practitioners in the field.

>[Petkus, M. (2019). Why and how zk-snark works. arXiv preprint arXiv:1906.07221.](https://arxiv.org/abs/1906.07221)
Despite the existence of multiple great resources on zk-SNARK construction, from original papers to explainers, due to the sheer number of moving parts the subject remains a black box for many. 
While some pieces of the puzzle are given one can not see the full picture without the missing ones. 
Hence the focus of this work is to shed light onto the topic with a straightforward and clean approach based on examples and answering many whys along the way so that more individuals can appreciate the state of the art technology, its innovators and ultimately the beauty of math. 
Paper's contribution is a simplistic exposition with a sufficient and gradually increasing level of complexity, necessary to understand zk-SNARK without any prerequisite knowledge of the subject, cryptography or advanced math. The primary goal is not only to explain how it works but why it works and how it came to be this way.

>[Sauer, J. F., & Szepieniec, A. (2021). Sok: Gröbner basis algorithms for arithmetization oriented ciphers. Cryptology ePrint Archive.](https://eprint.iacr.org/2021/870.pdf)
"SoK: Gröbner Basis Algorithms for Arithmetization Oriented Ciphers" by Jan Ferdinand Sauer and Alan Szepieniec provides an in-depth study of Gröbner basis algorithms in the context of arithmetization-oriented ciphers. 
It surveys key algorithms, emphasizing their intuitive understanding and complexity analysis, aiding in assessing the security of these ciphers. In my opinion, this is the best resource on Gröbner bases attacks.

>[Zhang, H., & Wang, X. (2009). Cryptanalysis of stream cipher grain family. Cryptology ePrint Archive.](https://eprint.iacr.org/2009/109.pdf)
"Cryptanalysis of Stream Cipher Grain Family," examines vulnerabilities in the Grain family of stream ciphers, including Grain v0, Grain v1, and Grain-128. 
The authors present a distinguishing attack against weak Key-IV (Initialisation Vector) combinations in these ciphers, using the second Walsh spectra of nonlinear functions to identify weak Key-IV pairs. 
They demonstrate that these weak Key-IVs can be distinguished with relatively few keystream bits and moderate computational effort. 
Additionally, they apply algebraic attacks to recover the secret key for these weak Key-IVs efficiently. 
The paper highlights significant security concerns in the Grain family, particularly in the context of weak Key-IV pairs, and suggests improvements for their cryptographic robustness.

