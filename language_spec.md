# Introduction
Specification of the ___ language. ___ is an expression-based programming language inspired by C. It is strongly typed.
# Notation
Syntactic components will be described in the form `component : definition`. Alternative definitions are provided on separate lines. Optional symbols will be surrounded by [] as follows `[optional]`. Literals and keywords will be **bolded**. For clarity, certain syntax components in a rule may be named as `name=Component`.
# Lexical Elements
TODO
# Types
Every expression has a static type. Type checking occurs before code generation. A program is valid only if every expression can be assigned a type and every operator is applied to operands of compatible types.

<pre>
Type :
    Scalar_Type
    Vector_Type

Scalar_Type :
    <b>bool</b>
    <b>char</b>
    <b>i8</b>
    <b>i16</b>
    <b>i32</b>
    <b>i64</b>
    <b>u8</b>
    <b>u16</b>
    <b>u32</b>
    <b>u64</b>
    <b>f16</b>
    <b>f32</b>
    <b>f64</b>
    <b>void</b>
</pre>

## Scalar Types
`bool` represents truth values and has the values `true` and `false`, or equivalently 1, 0.

`char` is an alias for `i8`. It represents an 8-bit integer value and may be written using character literals.

Signed integer types are `i8`, `i16`, `i32`, and `i64`. Unsigned integer types are `u8`, `u16`, `u32`, and `u64`. Unless otherwise stated, integer arithmetic is performed in the operand type after the usual implicit conversions have been applied.

Floating-point types are `f16`, `f32`, and `f64`.

`void` is the unit type. It has a single value and is used for expressions that complete without producing a meaningful value, such as an `if` expression without an `else` branch or a loop that performs only side effects.

## Default Values
Declarations create variables bound to the default value of their declared type:

| Type | Default value |
| --- | --- |
| `bool` | `false` |
| signed and unsigned integer types | `0` |
| floating-point types | `0.0` |
| vector types | elements initialized to default scalar value |
| `void` | the sole void value |

## Type Compatibility and Conversion
An expression of type `T` is assignable to a destination of type `U` if `T` and `U` are the same type or if an implicit conversion from `T` to `U` is defined.

Implicit conversions:

| From | To | Notes |
| --- | --- | --- |
| signed integer | wider signed integer | preserves the value |
| unsigned integer | wider unsigned integer | preserves the value |
| unsigned integer | signed integer with greater width | preserves the value |
| integer | floating-point | converts to the nearest representable value |
| `char`/`i8` | wider signed integer | follows signed integer widening rules |
| `char`/`i8` | floating-point | converts to the nearest representable value |
| scalar | `bool` | allowed only in conditions, where zero is false and nonzero is true |

Narrowing conversions, conversions from signed integers to unsigned integers, conversions from unsigned integers to signed integers of the same width, floating-point to integer conversions, and conversions between unrelated vector types are not implicit.

## Vector Types
<pre>
Vector_Type :
    Scalar_Type (<b>[</b>size=Simple_Expression<b>]</b>)+
</pre>

A vector type describes a fixed-size contiguous sequence of elements. The base type must be a scalar type. Each size expression must be an integer constant expression greater than zero.

Examples:

```___
i32[4]       // vector of four i32 values
f32[3][3]    // 3-by-3 vector, indexed as a vector of vectors
char[12]     // fixed-size character buffer
```

The type `T[N]` has `N` elements of type `T`. The type `T[M][N]` has `M` elements of type `T[N]`.

## Indexing
Indexing a value of type `T[N]` with an integer expression produces a variable binding of type `T` when the indexed value is assignable, and a value of type `T` otherwise.

```___
i32[4] xs;
xs[0] = 1;
xs[1]
```

Index expressions are checked at runtime unless the compiler can prove the index is within bounds statically. An out-of-bounds index is a runtime error.

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

## 

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


  
