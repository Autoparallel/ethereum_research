# Algebraic Cryptanalysis

## Gröbner Basis Attack
A Gröbner basis is a particular kind of generating set of a polynomial ideal in multivariate polynomial rings. It has properties that make the solution of polynomial systems simpler and more efficient. A Gröbner basis for an ideal is a set of polynomials from which one can derive the same solutions to the polynomial system as from the original set of polynomials, but with more straightforward computation, particularly for solving systems of equations and testing ideal membership.

The paper outlines three steps regarding the attack:

(1) computing the Gröbner basis in degrevlex order,


#### Degrevlex Order
The "degrevlex" order, short for "degree reverse lexicographic order," is a specific type of monomial ordering used in computational algebra, particularly in the computation of Gröbner bases. In this ordering, monomials are first compared by their total degree, with lower degree monomials being considered smaller. If two monomials have the same total degree, a reverse lexicographic order is applied, which means the monomials are compared based on the powers of their variables, starting from the last variable and moving backwards. This order is commonly used due to its computational efficiency in Gröbner basis calculations.

(2) converting the result to the lexicographic order, and
(3) factorizing the univariate polynomial, and backsubstituting its roots.


To find a Gröbner basis for the Poseidon hash function, one would start by formulating the function as a system of polynomial equations. This involves expressing each step of the hash function (like the S-boxes and linear layers) in polynomial form. Then, using a chosen monomial order (such as degrevlex), a Gröbner basis algorithm like Buchberger's algorithm or F4/F5 algorithms is applied to these polynomial equations. The process iteratively refines the initial set of equations into a Gröbner basis, which simplifies the system and makes it easier to analyze algebraically.