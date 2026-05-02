
// Generated from /home/austin/compilers/___Language/grammar/Underscore.g4 by ANTLR 4.13.0

#pragma once


#include "antlr4-runtime.h"
#include "UnderscoreParser.h"


namespace underscore {

/**
 * This class defines an abstract visitor for a parse tree
 * produced by UnderscoreParser.
 */
class  UnderscoreVisitor : public antlr4::tree::AbstractParseTreeVisitor {
public:

  /**
   * Visit parse trees produced by UnderscoreParser.
   */
    virtual std::any visitFile(UnderscoreParser::FileContext *context) = 0;

    virtual std::any visitExpr(UnderscoreParser::ExprContext *context) = 0;

    virtual std::any visitParen(UnderscoreParser::ParenContext *context) = 0;

    virtual std::any visitBreak(UnderscoreParser::BreakContext *context) = 0;

    virtual std::any visitCmp(UnderscoreParser::CmpContext *context) = 0;

    virtual std::any visitAddsub(UnderscoreParser::AddsubContext *context) = 0;

    virtual std::any visitIndex(UnderscoreParser::IndexContext *context) = 0;

    virtual std::any visitId(UnderscoreParser::IdContext *context) = 0;

    virtual std::any visitCond(UnderscoreParser::CondContext *context) = 0;

    virtual std::any visitWhile(UnderscoreParser::WhileContext *context) = 0;

    virtual std::any visitEquality(UnderscoreParser::EqualityContext *context) = 0;

    virtual std::any visitInt(UnderscoreParser::IntContext *context) = 0;

    virtual std::any visitMuldiv(UnderscoreParser::MuldivContext *context) = 0;

    virtual std::any visitKeywords(UnderscoreParser::KeywordsContext *context) = 0;


};

}  // namespace underscore
