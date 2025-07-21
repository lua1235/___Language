# ___ Compiler
A compiler for the ___ language, written in Rust to learn Rust. The language is designed to share the elegant syntax of C, with a few "nice" modifications here or there to implement features I enjoy.

# Language Features
Refer to the wiki for a full language spec, as well as a programmer's guide. Here is a highlight of a few of the features that ___ implements.

## Expression-Based Syntax
In ___, almost everything is an expression. This means that you can very naturally write pseudo-functional code by composing expressions using different operators. An expression followed by a semicolon is considered a statement, and semantically it means to evaluate the expression to the left of the semicolon first, before evaluating the remainder of the program. This distinction is mostly for clarity, since a definition similar to the comma operator in C and C++ would be equivalent.
<pre>
  // Fast exponentiation function
  <b>int</b> pow(<b>int</b> base, int exponent) = 
    <b>if</b>(exponent == 0) 1 
    <b>else</b> (<b>if</b>(exponent % 2 == 1) base <b>else</b> 1) * pow(base * base, exponent >>= 1)
  ; // From this point on, the declaration-definition of pow is guaranteed to have been evaluated, and calls to pow are well-defined
  ...
</pre>
## Nicer Assignment
Has it ever irked you that the statement `y=3*x=5+2` isn't the same as `y=3*7` (In fact, it won't even compile)? This is because the assignment operator has low precedence in C, which means that the compiler parses it as `y=((3*x)=(5+2))`, and `3*x` is not an l-value. Bumping up the associativity of the = won't help either, since that would break regular assignment. 

The solution is to move away from precedence, and instead use left and right binding powers (lbp and rbp respectively), a concept taken from the Pratt parsing algorithm [1](https://dl.acm.org/doi/pdf/10.1145/512927.512931). In this way, the desired behavior can be very naturally represented by giving the assignment operator very high lbp, and very low rbp. As an example, suppose we have the following binding powers for our operators:  
`+,- : (2,3)`  
`*,/ : (4,5)`  
`= : (6,0)`  
Now, evaluating `y=3\*x=5+2` gives us `y=(3*(x=(5+2)))`. Here, everything to the right of an assignment operator is evaluated first, since the assignment operator has low rbp. On the other hand, the token immediately before the assignment operator is used as the left operand, thanks to the high lbp. 

## Declaration as an Expression
In ___, declaration specifiers (int, char, float, const, long, etc) are treated as prefix operators that either declare new variables in the current scope or modify the attributes of the existing variable. Type specifiers declare new variables of a specific type and can only be called on identifiers that have not been declared in this scope. Modifiers can only be applied to existing variables and change how the variable behaves. For example, the `const` modifier prevents the contents of that variable from being changed after that point. Note that this behavior allows the seperate specification of modifiers as needed, for example:
```
const int y = 8; // Declare y
int x = 10; // Declare x
...
x = 5; // Change the value of x. OK!
// y = 4; // ERROR, y is const
const x; // Mark x as const
x = 3; // ERROR, x is const
```
Declaration specifiers return a reference to the variable, so composing them works as valid
