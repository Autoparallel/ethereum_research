# Algebraic Cryptanalysis

## Gröbner Basis Attack
A Gröbner basis is a particular kind of generating set of a polynomial ideal in multivariate polynomial rings. It has properties that make the solution of polynomial systems simpler and more efficient. A Gröbner basis for an ideal is a set of polynomials from which one can derive the same solutions to the polynomial system as from the original set of polynomials, but with more straightforward computation, particularly for solving systems of equations and testing ideal membership.

The paper outlines three steps regarding the attack:
#### Polynomial Modeling
The first step to performing a Gröbner basis attack is to model the hash function as a system of polynomial equations. This involves expressing each step of the hash function (like the S-boxes and linear layers) in polynomial form. Then, using a chosen monomial order (such as degrevlex), a Gröbner basis algorithm like Buchberger's algorithm or F4/F5 algorithms is applied to these polynomial equations. The process iteratively refines the initial set of equations into a Gröbner basis, which simplifies the system and makes it easier to analyze algebraically.

#### Gröbner Basis Computation
The next step is to compute a Gröbner basis for the ideal spanned by the list of polynomials, typically using a degree-refining term order such as degrevlex.

##### Degrevlex Order
The "degrevlex" order, short for "degree reverse lexicographic order," is a specific type of monomial ordering used in computational algebra, particularly in the computation of Gröbner bases. In this ordering, monomials are first compared by their total degree, with lower degree monomials being considered smaller. If two monomials have the same total degree, a reverse lexicographic order is applied, which means the monomials are compared based on the powers of their variables, starting from the last variable and moving backwards. This order is commonly used due to its computational efficiency in Gröbner basis calculations.

#### Term Order Change
The process of converting from a degrevlex to a lexicographical order in a Gröbner basis involves a sequence of computational steps. These steps reorient the basis from a degree-based prioritization (degrevlex) to one where variables are considered in a strict order (lexicographical). This reordering aligns the basis with the natural order of variables, which can be essential for solving systems of equations or performing further algebraic manipulations. 

#### Computing the solution
The last step in a Gröbner basis attack typically involves solving the system of equations represented by the Gröbner basis to find the hash function's pre-image or collision. The Euclidean algorithm and polynomial factorization are used to simplify and solve these equations.

I think this is the best resource on Gröbner bases attacks

> [Sauer, J. F., & Szepieniec, A. SoK: Gröbner Basis Algorithms for Arithmetization Oriented Ciphers](https://eprint.iacr.org/2021/870.pdf)