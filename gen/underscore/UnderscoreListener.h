
// Generated from /home/austin/compilers/___Language/grammar/Underscore.g4 by ANTLR 4.13.0

#pragma once


#include "antlr4-runtime.h"
#include "UnderscoreParser.h"


namespace underscore {

/**
 * This interface defines an abstract listener for a parse tree produced by UnderscoreParser.
 */
class  UnderscoreListener : public antlr4::tree::ParseTreeListener {
public:

  virtual void enterFile(UnderscoreParser::FileContext *ctx) = 0;
  virtual void exitFile(UnderscoreParser::FileContext *ctx) = 0;

  virtual void enterExpr(UnderscoreParser::ExprContext *ctx) = 0;
  virtual void exitExpr(UnderscoreParser::ExprContext *ctx) = 0;

  virtual void enterParen(UnderscoreParser::ParenContext *ctx) = 0;
  virtual void exitParen(UnderscoreParser::ParenContext *ctx) = 0;

  virtual void enterBreak(UnderscoreParser::BreakContext *ctx) = 0;
  virtual void exitBreak(UnderscoreParser::BreakContext *ctx) = 0;

  virtual void enterCmp(UnderscoreParser::CmpContext *ctx) = 0;
  virtual void exitCmp(UnderscoreParser::CmpContext *ctx) = 0;

  virtual void enterAddsub(UnderscoreParser::AddsubContext *ctx) = 0;
  virtual void exitAddsub(UnderscoreParser::AddsubContext *ctx) = 0;

  virtual void enterIndex(UnderscoreParser::IndexContext *ctx) = 0;
  virtual void exitIndex(UnderscoreParser::IndexContext *ctx) = 0;

  virtual void enterId(UnderscoreParser::IdContext *ctx) = 0;
  virtual void exitId(UnderscoreParser::IdContext *ctx) = 0;

  virtual void enterCond(UnderscoreParser::CondContext *ctx) = 0;
  virtual void exitCond(UnderscoreParser::CondContext *ctx) = 0;

  virtual void enterWhile(UnderscoreParser::WhileContext *ctx) = 0;
  virtual void exitWhile(UnderscoreParser::WhileContext *ctx) = 0;

  virtual void enterEquality(UnderscoreParser::EqualityContext *ctx) = 0;
  virtual void exitEquality(UnderscoreParser::EqualityContext *ctx) = 0;

  virtual void enterInt(UnderscoreParser::IntContext *ctx) = 0;
  virtual void exitInt(UnderscoreParser::IntContext *ctx) = 0;

  virtual void enterMuldiv(UnderscoreParser::MuldivContext *ctx) = 0;
  virtual void exitMuldiv(UnderscoreParser::MuldivContext *ctx) = 0;

  virtual void enterKeywords(UnderscoreParser::KeywordsContext *ctx) = 0;
  virtual void exitKeywords(UnderscoreParser::KeywordsContext *ctx) = 0;


};

}  // namespace underscore
