# Differential Cryptanalysis
Differential cryptanalysis is a method of cryptanalysis that exploits the characteristics of differential probabilities to break a cipher. 
It is a general form of cryptanalysis that can be applied to many ciphers, including block ciphers, stream ciphers, and hash functions. 
The attack involves finding a difference between two plaintexts, resulting in a specific difference in the ciphertexts. 
The attacker then uses this information to recover the secret key. 
This attack is particularly effective against block ciphers that use a substitution-permutation network (SPN) structure.

The probability of successful differential attacks on Poseidon is determined by analyzing the number of active S-boxes in its structure. 
Poseidon is designed with a certain number of rounds, each involving substitution (S-boxes) and permutation steps. 
The security against differential attacks is higher when more active S-boxes are involved, as this increases the complexity and reduces the probability of finding exploitable differential patterns. 
The mathematical intuition hinges on the observation that each active S-box significantly reduces the likelihood of a successful differential attack by increasing the nonlinearity and complexity of the transformation.