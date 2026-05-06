#pragma once
#include "ASTNodes.h"
#include "ASTProps.h"

// The actual definitions for the AST nodes
namespace ast {

    // AST node definitions
    // Seperator expression node
    struct Sep : Pos, RetType {
        SimpleExpression car;
        Expression cdr;
    };

    // If expression node
    struct If : Pos, RetType {
        Expression cond;
        SimpleExpression if_body, else_body;
    };

    // While expression node
    struct While : Pos, RetType {
        Expression cond;
        SimpleExpression body;
    };

    // Generate Literal expression nodes
#define MAP(N,Ty) \
    struct N : LitExpr<Ty>, Pos, RetType {\
        N(Ty value, size_t row, size_t col) :\
            LitExpr{value}, Pos{row, col} {}\
    };
    LIT_EXPRS(MAP)
#undef MAP

    // Generate Declaration expression nodes
#define MAP(N) \
    struct N : DeclExpr, Pos, RetType {\
        N(std::shared_ptr<Id> name, size_t row, size_t col) :\
            DeclExpr{nullptr, name}, Pos{row, col} {}\
    };
    DECL_EXPRS(MAP)
#undef MAP

    // Generate Binary expression nodes
#define MAP(N) \
    struct N : BinExpr, Pos, RetType {\
        N(SimpleExpression lhs, SimpleExpression rhs, size_t row, size_t col) :\
            BinExpr{lhs, rhs}, Pos{row, col} {}\
    };
    BIN_EXPRS(MAP)
#undef MAP

    // Generate Unary expression nodes 
#define MAP(N) \
    struct N : UnExpr, Pos, RetType {\
        N(SimpleExpression inner, size_t row, size_t col) :\
            UnExpr{inner}, Pos{row, col} {}\
    };
    UNARY_EXPRS(MAP)
#undef MAP
}

