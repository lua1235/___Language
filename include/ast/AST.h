#pragma once
#include <variant>
#include <memory>
#include "ASTNodes.h"

// AST for the underscore programming language.
namespace ast {
    using SimpleExpression = std::variant<
        std::shared_ptr<Id>, 
        std::shared_ptr<IntLit>, 
        std::shared_ptr<Add>, 
        std::shared_ptr<Sub>, 
        std::shared_ptr<Mul>, 
        std::shared_ptr<Div>, 
        std::shared_ptr<EQ>, 
        std::shared_ptr<NE>, 
        std::shared_ptr<GE>, 
        std::shared_ptr<LE>, 
        std::shared_ptr<GT>, 
        std::shared_ptr<LT>, 
        std::shared_ptr<Paren>>;

    using Expression = std::variant<Sep, SimpleExpression>;

    
    


}

