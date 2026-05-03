#pragma once

// Registry of ast node types. Categories are either disjoint or strict subsets. Add new nodes to the most granular possible category that still makes sense

// Basic categories
#define LIT_EXPRS(X)        \
    X(IntLit)

// BinExpr -> SimpleExpr Op SimpleExpr
#define BIN_EXPRS(X)        \
    X(Add)                  \
    X(Sub)                  \
    X(Mul)                  \
    X(Div)                  \
    X(EQ)                   \
    X(NE)                   \
    X(GE)                   \
    X(LE)                   \
    X(GT)                   \
    X(LT)       

#define UNARY_EXPRS(X)      \

// Collect all simple exprs
#define SIMPLE_EXPRS(X)     \
    X(Id)                   \
    X(Paren)                \
    LIT_EXPRS(X)            \
    BIN_EXPRS(X)            \
    UNARY_EXPRS(X)          \

// Collect all exprs
#define EXPRS(X)            \
    X(Sep)                  \
    SIMPLE_EXPRS(X)

