# Introduction
Specification of the ___ language. ___ is an expression-based programming language inspired by C. It is strongly typed.

This document describes the intended language. The current compiler implementation may support only a subset of this specification.
# Notation
Syntactic components will be described in the form `component : definition`. Alternative definitions are provided on separate lines. Optional symbols will be surrounded by [] as follows `[optional]`. Literals and keywords will be **bolded**. For clarity, certain syntax components in a rule may be named as `name=Component`.
# Lexical Elements
TODO
# Types
Every expression has a static type. Type checking occurs before code generation. A program is valid only if every expression can be assigned a type and every operator is applied to operands of compatible types.

<pre>
Type :
    Scalar_Type
    Tensor_Type

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

Signed integer types are `i8`, `i16`, `i32`, and `i64`. Unsigned integer types are `u8`, `u16`, `u32`, and `u64`. Unless otherwise stated, integer arithmetic is performed in the result type selected by the arithmetic conversion rules.

Floating-point types are `f16`, `f32`, and `f64`.

`void` is the unit type. It has a single value and is used for expressions that complete without producing a meaningful value, such as an `if` expression without an `else` branch or a loop that performs only side effects.

## Default Values
Declarations create variables bound to the default value of their declared type:

| Type | Default value |
| --- | --- |
| `bool` | `false` |
| signed and unsigned integer types | `0` |
| floating-point types | `0.0` |
| tensor types | elements initialized to default scalar value |
| `void` | the sole void value |

## Type Compatibility and Conversion
An expression of type `T` is assignable to a destination of type `U` if `T` and `U` are the same type or if an implicit conversion from `T` to `U` is defined.

Implicit conversions:

| From | To | Notes |
| --- | --- | --- |
| signed integer | wider signed integer | preserves the value |
| signed integer | narrower signed integer | truncates to the destination bit width |
| signed integer | unsigned integer | truncates to the destination bit width |
| unsigned integer | wider unsigned integer | preserves the value |
| unsigned integer | narrower unsigned integer | truncates to the destination bit width |
| unsigned integer | wider signed integer | preserves the value |
| unsigned integer | narrower signed integer | truncates to the destination bit width |
| integer | floating-point | converts to the nearest representable value |
| `char`/`i8` | wider signed integer | follows signed integer widening rules |
| `char`/`i8` | floating-point | converts to the nearest representable value |
| scalar | `bool` | allowed only in conditions, where zero is false and nonzero is true |

For conversions to a signed integer type, the resulting bit pattern is interpreted as a two's-complement signed value.

Conversions from floating-point types to integer types are not implicit.

Tensor types may convert to another tensor type with the same rank and equal dimension values if each element is implicitly convertible to the destination element type. Shape equality is checked statically when possible and at runtime otherwise. Tensor conversion is applied elementwise.

## Arithmetic Conversions
Binary integer arithmetic is performed in the wider operand type. If both integer operands have the same width and one is signed while the other is unsigned, arithmetic is performed in the unsigned type of that width.

Binary arithmetic between an integer and a floating-point value is performed in the floating-point operand type. Binary arithmetic between two floating-point values is performed in the wider floating-point type.

## Tensor Types
<pre>
Tensor_Type :
    Scalar_Type (<b>[</b>size=Simple_Expression<b>]</b>)+
</pre>

A tensor type describes a fixed-size contiguous sequence of elements. The base type must be a scalar type. Each size expression must evaluate to a non-negative integer. Tensor size expressions are evaluated when the tensor is created, and the tensor's shape is fixed after creation.

Examples:

```___
i32[4]       // tensor of four i32 values
f32[3][3]    // 3-by-3 tensor, indexed as a tensor of tensors
char[12]     // fixed-size character buffer
```

The type `T[N]` has `N` elements of type `T`. The type `T[M][N]` has `M` elements of type `T[N]`.

## Function Types
<pre>
Function_Type :
    Type <b>(</b>Type_List?<b>)</b>

Type_List :
    Type
    Type <b>,</b> Type_List

Function types of the form T(S1, ... SN) denote a value of type T which depends on N other values of types S1...SN respectively. Function types are classified as scalar types, even if the type T is a Tensor. A value of type T(S1, ... SN) is called a T-valued function

```___
i32(i32, i32) // function which returns i32 and depends on two other i32s
i32[10](i8) // tensor of ten i32s which depends on an i8
f16[10](i8)[5][5] // 5-by-5 tensor of functio 
```

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
    Index_Expression
    Identifier
    Decl_Expression
    Constant
</pre>
## Parenthesis Expressions
<pre>
Paren_Expression : 
    (Expression)
</pre>
Parenthesis expressions have the highest precedence, and evaluate to the value of the inner expression.
## Index Expressions
<pre>
Index_Expression : target=Simple_Expression <b>[</b>index=Simple_Expression<b>]</b>
</pre>

Index expressions access one element of a tensor value. The target expression must be of tensor type `T[N]`, and the index expression must evaluate to an integer that is implicitly convertible to `u64`. The index value is converted to `u64` before indexing. The result has type `T`.

Indexing a value of type `T[N]` produces a variable binding of type `T` when the target expression evaluates to a variable binding, and a value of type `T` otherwise.

```___
i32[4] u;
u[0] = 1;
u[1]
```

Index expressions are checked at runtime unless the compiler can prove the index is within bounds statically. An out-of-bounds index is a runtime error.

## If Expressions
<pre>
If_Expression : 
    <b>if(</b>condition=Expression<b>)</b> if_body=Simple_Expression [<b>else</b> else_body=Simple_Expression]
</pre>
If expressions evaluate to the value of the `if_body` that follows `if(BooleanExpression)` if the condition evaluates to true. The value of the condition must be a boolean or implicitly convertible to a boolean. If an `else_body` is present and the condition evaluates to false, the if expression evaluates to the `else_body`. When an `else_body` is present, both branches must evaluate to the same type. When an `else_body` is not present, the `if_body` must evaluate to `void` and the if expression has type `void`.
## While Expressions
<pre>
While_Expression : 
    <b>while(</b>condition=Expression<b>)</b> body=Simple_Expression
</pre>
While expressions repeatedly check the value of the condition and evaluate the body if the condition is true. The value of the condition must be a boolean or implicitly convertible to a boolean. A while expression has the same type as its body. If the condition becomes false after one or more iterations, the while expression evaluates to the value of the last body evaluation. If the condition is false before the first iteration, the while expression evaluates to the default value of the body type.
## Break Expressions
<pre>
Break_Expression : 
    <b>break</b> [Simple_Expression]
</pre>
Break expressions are valid only within the body of a `while` expression, including expressions nested inside that body. A break expression exits the innermost enclosing `while`. If the break expression has an operand, the operand type must match the enclosing while body type. The while expression evaluates to that operand value. A bare `break` exits with the default value of the enclosing while body type.
## Identifier Expressions and Variables
<pre>
Identifier_Expression : identifier
</pre>
Identifier expressions are a single identifier, which may or may not be bound to a value. Unbound identifiers may only appear as the operand of a `Decl_Expression`. Bound identifiers are referred to as variables. An identifier expression with a bound identifier evaluates to the corresponding variable binding.

## Variable Bindings
A variable binding is a special value that represents the mapping between a value and a specific variable. Variable bindings will evaluate to the mapped value of the variable when used as operand to an operator that requires a concrete value. 

## Decl Expressions
<pre>
Decl_Expression : 
    Type Identifier_Expression
</pre>
Declaration Expressions bind an unbound identifier to the default value of the specified `Type`. All primitive types have a well defined default value. Declaration expressions return a variable binding.

Declaration has higher precedence than assignment. Therefore `i32 x = 10` is parsed as `(i32 x) = 10`: the declaration expression first creates a variable binding initialized to the declared type's default value, and the assignment expression then updates that binding.

## Assn Expressions
<pre>
Assn_Expression : 
    lhs=Simple_Expression <b>=</b> rhs=Simple_Expression
</pre>
Assignment Expressions are a binary expression which requires the `lhs` to evaluate to a variable binding. The type of the `rhs` must be implicitly convertible to the type of the value mapped to by the variable binding returned by the `lhs`. An assignment expression evaluates to the updated `lhs` variable binding.

## Parameterized Index Assignment

A parameterized index assignment is an assignment expression whose `lhs` contains one or more unbound identifiers in tensor index positions. Each such identifier becomes an index parameter for the assignment.

<pre>
Parameterized_Index_Assignment :
    lhs=Parameterized_Index_Expression <b>=</b> rhs=Simple_Expression
    lhs=Parameterized_Index_Expression Compound_Assn_Operator rhs=Simple_Expression

Parameterized_Index_Expression :
    Simple_Expression

Compound_Assn_Operator :
    <b>+=</b>
    <b>-=</b>
    <b>*=</b>
    <b>/=</b>
</pre>

For each index parameter, the assignment is evaluated once for every value in that parameter's inferred domain. The parameter is bound only within the parameterized assignment expression and has type `u64`.

The domain of an index parameter is inferred from the tensor dimensions where it appears in the `lhs`. For a parameter that appears in one dimension of size `N`, the parameter ranges from `0` to `N - 1`. If the same parameter appears in multiple `lhs` index positions, its domain is the intersection of those dimensions, equivalent to ranging from `0` to `min(N0, N1, ...) - 1`.

Examples:

```___
i32[4] v;
v[i] = i;       // assigns v[0] = 0, v[1] = 1, v[2] = 2, v[3] = 3

i32[50] fib[i] = // Generate first 50 elements of fibonacci sequence using DP
    if(i <= 1) 
        i 
    else 
        fib[i-1] + fib[i-2];

i32[4][4] A[i][i] = 1;     // assigns the diagonal. None-diagonal elements are bound to default values
```

Parameterized assignments are evaluated in row-major order. Parameters introduced by earlier `lhs` indexes are outer loop parameters, and parameters introduced by later `lhs` indexes are inner loop parameters. If a parameter appears more than once, its loop position is determined by its first occurrence in a left-to-right traversal of `lhs` index expressions. For `A[i][j] = rhs`, all `j` values are evaluated for a fixed `i` before moving to the next `i`.

The compiler may reorder, vectorize, or parallelize a parameterized assignment only when it can prove that the observable result is unchanged.

Compound parameterized assignment is equivalent to applying the compound operation at each selected element in row-major order, with the `lhs` element evaluated only once per parameter combination. A compound assignment expression evaluates to the updated `lhs` variable binding.

```___
i32[4] w;
w[i] += 1;
```

A parameterized index assignment evaluates to the variable binding for the assigned tensor.

# Operators
Operators take a fixed number of operands and perform a computation, assignment, or declaration. An operator can be either prefix, infix, or postfix.
```
Operand : 
    Constant 
    Identifier 
    Expression
```

## Operator Precedence
Operator precedence determines how an expression is grouped when parentheses are not present. Operators with higher precedence bind more tightly than operators with lower precedence. Parentheses may be used to override the default grouping.

| Precedence | Syntax | Name | Associativity |
| --- | --- | --- | --- |
| 1 | `(expr)` | parenthesized expression | n/a |
| 2 | `Type identifier` | declaration | n/a |
| 3 | `expr[expr]` | indexing | left |
| 4 | `*`, `/` | multiplication and division | left |
| 5 | `+`, `-` | addition and subtraction | left |
| 6 | `>=`, `<=`, `>`, `<` | comparison | left |
| 7 | `==`, `!=` | equality | left |
| 8 | `=`, `+=`, `-=`, `*=`, `/=` | assignment | left |
| 9 | `if(expr) expr [else expr]` | if expression | n/a |
| 9 | `while(expr) expr` | while expression | n/a |
| 9 | `break [expr]` | break expression | n/a |
| 10 | `;` | sequencing | right |

Lower precedence numbers bind more tightly. For example, declaration has higher precedence than assignment, so `i32 x = 10` is grouped as `(i32 x) = 10`. Multiplication has higher precedence than addition, so `x + y * z` is grouped as `x + (y * z)`.

Control-flow expressions bind more tightly than sequencing, so in `while(cond) a; b`, the loop body is `a` and the full expression is `(while(cond) a); b`.

# The Assignment Operator and Scoping
The right hand side expression of the assignment operator belongs to a new scope. A scope may read but not modify any variables in its enclosing scopes. This ensures that all changes to the value of a variable happen in the scope it exists in, improving code clarity.

If an identifier could refer to either a variable in the current scope or the enclosing scope, it will resolve to the variable in the current scope. That is, the variable from the enclosing scope is shadowed.
<pre>
...
i32 i = 567;
while(i != 1) i = (
  i32 p = i;
  print(p);
  if(i % 2 == 0) 
    i / 2 
  else 
    i * 3 + 1
)


  
