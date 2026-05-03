//Properties commonly shared among AST nodes

#include "Type.h"
#include "ast/ASTNodes.h"
#include <memory>

namespace ast {
    // Position of expression in input file
    struct Pos {
        size_t row;
        size_t col;
    };

    // Type of expression return value
    struct RetType {
        std::shared_ptr<Type> type = nullptr;
    };

    // Binary expression operands
    struct BinExpr {
        SimpleExpression lhs, rhs;
    };

    // Unary expression operand
    struct UnExpr {
        SimpleExpression inner;
    };

    // Const expression value
    template<typename T>
    struct LitExpr {
        const T value;
    };
}
