
// Generated from /home/austin/compilers/___Language/grammar/Underscore.g4 by ANTLR 4.13.0

#pragma once


#include "antlr4-runtime.h"
#include "UnderscoreVisitor.h"


namespace underscore {

/**
 * This class provides an empty implementation of UnderscoreVisitor, which can be
 * extended to create a visitor which only needs to handle a subset of the available methods.
 */
class  UnderscoreBaseVisitor : public UnderscoreVisitor {
public:

  virtual std::any visitFile(UnderscoreParser::FileContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitExpr(UnderscoreParser::ExprContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitParen(UnderscoreParser::ParenContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitBreak(UnderscoreParser::BreakContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitCmp(UnderscoreParser::CmpContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitAddsub(UnderscoreParser::AddsubContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitIndex(UnderscoreParser::IndexContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitId(UnderscoreParser::IdContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitCond(UnderscoreParser::CondContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitWhile(UnderscoreParser::WhileContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitEquality(UnderscoreParser::EqualityContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitInt(UnderscoreParser::IntContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitMuldiv(UnderscoreParser::MuldivContext *ctx) override {
    return visitChildren(ctx);
  }

  virtual std::any visitKeywords(UnderscoreParser::KeywordsContext *ctx) override {
    return visitChildren(ctx);
  }


};

}  // namespace underscore
