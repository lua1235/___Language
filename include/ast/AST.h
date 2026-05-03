#pragma once
#include <variant>
#include <memory>
#include "ASTNodes.h"
#include "Type.h"

// AST for the underscore programming language.
namespace ast {
// Forward declare the
#define GEN(classname) \
    struct classname;
        EXPRS(GEN)
#undef GEN

    using SimpleExpression = std::variant<
        std::monostate
        // Standard expressions
#define MAP(N) , std::shared_ptr<N> \
        SIMPLE_EXPRS(MAP)
#undef MAP
        >;

    using Expression = std::variant<SimpleExpression, std::shared_ptr<Sep>>;


    // Properties shared by most nodes
    struct Pos {
        size_t row;
        size_t col;
    };

    struct RetType {
        std::shared_ptr<Type> type = nullptr;
    };

    struct BinExpr {
        SimpleExpression lhs, rhs;
    };

    struct UnExpr {
        SimpleExpression inner;
    };


    // AST node definitions
    struct IntLit : Pos, RetType {

    };

    struct Sep : Pos, RetType {
        SimpleExpression car;
        Expression cdr;
    };
    // Generate Binary Operator expressions
#define MAP(N) \
    struct N : BinExpr, Pos, RetType {\
        N(SimpleExpression lhs, SimpleExpression rhs, size_t row, size_t col) :\
            BinExpr{lhs, rhs}, Pos{row, col} {}\
    };
    BIN_EXPRS(MAP)
#undef MAP
#define MAP(N) \
    struct N : UnExpr, Pos, RetType {\
        N(SimpleExpression inner, size_t row, size_t col) :\
            UnExpr{inner}, Pos{row, col} {}\
    };
    UNARY_EXPRS(MAP)
#undef MAP
}

