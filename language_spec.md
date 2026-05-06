# Introduction
Specification of the ___ language. ___ is an expression-based programming language inspired by C. It is strongly typed.
# Notation
Syntactic components will be described in the form `component : definition`. Alternative definitions are provided on separate lines. Optional symbols will be surrounded by [] as follows `[optional]`. Literals and keywords will be **bolded**. For clarity, certain syntax components in a rule may be named as `name=Component`.
# Lexical Elements
TODO
# Expressions
Expressions are a valid sequence of operators and operands that represent some combination of computation, assignment, and declaration. Expressions always evaluate to a value. We classify expressions as sequence expressions and simple expressions. A ___ program is a single expression, or an empty file.

# The Sequence Expression
A ___ program is a single expression. The semicolon, or sequencing operator, is a binary operator that guarantees the expression to its left is evaluated before the expression to its right. This is not necessarily the case for other binary expressions. The result of the sequencing operator is the result of the right hand side expression. The sequencing operator has the lowest precedence of all operators.
<pre>
Sequencing Expression :
   Expression ; Expression
</pre>

# Simple Expressions
An expression is "simple" if it is not a sequencing expression. Some operators require their operands to be simple expressions. 
<pre>
Expression:
    Paren_Expression
    If_Expression
    While_Expression
    Identifier
    Decl_Expression
    Constant
</pre>
## Parenthesis Expressions
<pre>
Paren_Expression : (Expression)
</pre>
Parenthesis expressions have the highest precedence, and evaluate to the value of the inner expression.
## If Expressions
<pre>
If_Expression : <b>if(</b>condition=Expression<b>)</b> if_body=Simple_Expression [<b>else</b> else_body=Simple_Expression]
</pre>
If expressions evaluate to the value of the if_body that follows ```if(BooleanExpression)``` if the condition evaluates to true. The value of the condition must be a boolean or implicitly convertible to a boolean. If the condition evaluates to false, it evaluates to the else_body, or void if not present. Both the true branch and the false branch must evaluate to the same type. 
## While Expressions
<pre>
While_Expression : <b>while(</b>condition=Expression<b>)</b> body=Simple_Expression
</pre>
While expressions will repeatedly check the value of the condition and evaluate the body if the condition is true. The while expression terminates and returns the value of the last iteration of the body if the condition is false. 
## Break Expressions
<pre>
Break_Expression : <b>break</b> [Simple_Expression]
</pre>
Break expressions
## Identifier Expressions and Variables
<pre>
Identifier_Expression : identifier
</pre>
Identifier expressions are a single identifier, which may or may not be bound to a value. Unbound identifiers may only appear as the operand of a Decl_Expression. Bound identifiers are referred to as variables. An identifier expression with a bound identifier evaluates to the corresponding variable binding.

## Variable Bindings
A variable binding is a special value that represents the mapping between a value and a specific variable. Variable bindings will evaluate to the mapped value of the variable when used as operand to an operator that requires a concrete value. 

## Decl Expressions
<pre>
Decl_Expression : Type Identifier_Expression
</pre>
Declaration Expressions bind an identifier to a default value of the specified Type. All primitive types have a well defined default value. Declaration expressions return a variable binding.

## Assn Expressions
<pre>
Assn_Expression : lhs=Simple_Expression <b>=</b> rhs=Simple_Expression
</pre>
Assignment Expressions are a binary expression which requires the lhs to evaluate to a variable binding. The type of the rhs must be implicitly convertable to the type of the value mapped to by the variable binding returned by the rhs.

# Operators
Operators take a fixed number of operands and perform a computation, assignment, or declaration. An operator can be either prefix, infix, or postfix.
```
Operand : 
    Constant 
    Identifier 
    Expression
```
# The Assignment Operator and Scoping
The right hand side expression of the assignment operator belongs to a new scope. A scope may read but not modify any variables in its enclosing scopes. This ensures that all changes to the value of a variable happen in the scope it exists in, improving code clarity.

If an identifier could refer to either a variable in the current scope or the enclosing scope, it will resolve to the variable in the current scope. That is, the variable from the enclosing scope is shadowed.
<pre>
...
int i = 567;
while(i != 1) i = (
  int p = i;
  print(p);
  if(i % 2 == 0) 
    i / 2 
  else 
    i * 3 + 1
)


  


