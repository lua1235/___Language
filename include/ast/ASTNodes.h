#pragma once
// Nodes are forward declared to be made available for std::variant
namespace ast {
struct Sep; // The seperator simple_expression ; expression

// Arithmetic expressions
struct Id; // Identifiers
struct IntLit; // Int Literals
               
struct Add;
struct Sub; 
struct Mul;
struct Div;

struct EQ;
struct NE;

struct GE;
struct LE;
struct GT;
struct LT;
struct Paren;
}
