# CompilR
A compiler for a C-based language, written in Rust to learn Rust. The language to be implemented is based heavily on the C standard, with a few "nice" modifications here or there to implement features I enjoy.

# Language Features
## Nicer Assignment
Has it ever irked you that the statement `y=3*x=5+2` isn't the same as `y=3*7` (In fact, it won't even compile)? This is because the assignment operator has low precedence in C, which means that the compiler parses it as `y=((3*x)=(5+2))`, and `3*x` is not an l-value. Bumping up the associativity of the = won't help either, since that would break regular assignment. 

The solution is to move away from precedence, and instead use left and right binding powers (lbp and rbp respectively), a concept taken from the Pratt parsing algorithm [1](https://dl.acm.org/doi/pdf/10.1145/512927.512931). In this way, the desired behavior can be very naturally represented by giving the assignment operator very high lbp, and very low rbp. As an example, suppose we have the following binding powers for our operators:  
`+,- : (2,3)`  
`*,/ : (4,5)`  
`= : (6,0)`  
Now, evaluating `y=3\*x=5+2` gives us `y=(3*(x=(5+2)))`. Here, everything to the right of an assignment operator is evaluated first, since the assignment operator has low rbp. On the other hand, the token immediately before the assignment operator is used as the left operand, thanks to the high lbp. 
