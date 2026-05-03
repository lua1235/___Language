#pragma once

// Registry of ast node types. Categories are either disjoint or strict subsets. Add new nodes to the most granular possible category that still makes sense

#include <memory>
#include <variant>
namespace ast {
// Basic categories
// LitExprs are special since they have a value-type
#define LIT_EXPRS(X)        \
    X(IntLit, int)

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
    X(LT)                   \
    X(Ass)                  \
    X(AddAss)               \
    X(SubAss)               

#define DECL_EXPRS(X)       \
    X(IntDecl)

#define UNARY_EXPRS(X)      \

// Collect all simple exprs
#define SIMPLE_EXPRS(X)     \
    X(Id)                   \
    X(Paren)                \
    X(If)                   \
    X(While)                \
    LIT_EXPRS(X)            \
    BIN_EXPRS(X)            \
    UNARY_EXPRS(X)          \

// Collect all exprs
#define EXPRS(X)            \
    X(Sep)                  \
    SIMPLE_EXPRS(X)

// Forward declare the different nodes
#define GEN(classname,...) \
    struct classname;
        EXPRS(GEN)
#undef GEN

// Create the variants
    using SimpleExpression = std::variant<
        std::monostate
        // Standard expressions
#define MAP(N,...) , std::shared_ptr<N> \
        SIMPLE_EXPRS(MAP)
#undef MAP
        >;

    using Expression = std::variant<SimpleExpression, std::shared_ptr<Sep>>;
}
