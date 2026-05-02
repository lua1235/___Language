
// Generated from /home/austin/compilers/___Language/grammar/Underscore.g4 by ANTLR 4.13.0

#pragma once


#include "antlr4-runtime.h"
#include "UnderscoreListener.h"


namespace underscore {

/**
 * This class provides an empty implementation of UnderscoreListener,
 * which can be extended to create a listener which only needs to handle a subset
 * of the available methods.
 */
class  UnderscoreBaseListener : public UnderscoreListener {
public:

  virtual void enterFile(UnderscoreParser::FileContext * /*ctx*/) override { }
  virtual void exitFile(UnderscoreParser::FileContext * /*ctx*/) override { }

  virtual void enterExpr(UnderscoreParser::ExprContext * /*ctx*/) override { }
  virtual void exitExpr(UnderscoreParser::ExprContext * /*ctx*/) override { }

  virtual void enterParen(UnderscoreParser::ParenContext * /*ctx*/) override { }
  virtual void exitParen(UnderscoreParser::ParenContext * /*ctx*/) override { }

  virtual void enterBreak(UnderscoreParser::BreakContext * /*ctx*/) override { }
  virtual void exitBreak(UnderscoreParser::BreakContext * /*ctx*/) override { }

  virtual void enterCmp(UnderscoreParser::CmpContext * /*ctx*/) override { }
  virtual void exitCmp(UnderscoreParser::CmpContext * /*ctx*/) override { }

  virtual void enterAddsub(UnderscoreParser::AddsubContext * /*ctx*/) override { }
  virtual void exitAddsub(UnderscoreParser::AddsubContext * /*ctx*/) override { }

  virtual void enterIndex(UnderscoreParser::IndexContext * /*ctx*/) override { }
  virtual void exitIndex(UnderscoreParser::IndexContext * /*ctx*/) override { }

  virtual void enterId(UnderscoreParser::IdContext * /*ctx*/) override { }
  virtual void exitId(UnderscoreParser::IdContext * /*ctx*/) override { }

  virtual void enterCond(UnderscoreParser::CondContext * /*ctx*/) override { }
  virtual void exitCond(UnderscoreParser::CondContext * /*ctx*/) override { }

  virtual void enterWhile(UnderscoreParser::WhileContext * /*ctx*/) override { }
  virtual void exitWhile(UnderscoreParser::WhileContext * /*ctx*/) override { }

  virtual void enterEquality(UnderscoreParser::EqualityContext * /*ctx*/) override { }
  virtual void exitEquality(UnderscoreParser::EqualityContext * /*ctx*/) override { }

  virtual void enterInt(UnderscoreParser::IntContext * /*ctx*/) override { }
  virtual void exitInt(UnderscoreParser::IntContext * /*ctx*/) override { }

  virtual void enterMuldiv(UnderscoreParser::MuldivContext * /*ctx*/) override { }
  virtual void exitMuldiv(UnderscoreParser::MuldivContext * /*ctx*/) override { }

  virtual void enterKeywords(UnderscoreParser::KeywordsContext * /*ctx*/) override { }
  virtual void exitKeywords(UnderscoreParser::KeywordsContext * /*ctx*/) override { }


  virtual void enterEveryRule(antlr4::ParserRuleContext * /*ctx*/) override { }
  virtual void exitEveryRule(antlr4::ParserRuleContext * /*ctx*/) override { }
  virtual void visitTerminal(antlr4::tree::TerminalNode * /*node*/) override { }
  virtual void visitErrorNode(antlr4::tree::ErrorNode * /*node*/) override { }

};

}  // namespace underscore
