
// Generated from /home/austin/compilers/___Language/grammar/Underscore.g4 by ANTLR 4.13.0

#pragma once


#include "antlr4-runtime.h"


namespace underscore {


class  UnderscoreParser : public antlr4::Parser {
public:
  enum {
    T__0 = 1, T__1 = 2, T__2 = 3, T__3 = 4, T__4 = 5, T__5 = 6, T__6 = 7, 
    T__7 = 8, T__8 = 9, T__9 = 10, SEMI = 11, LARROW = 12, RARROW = 13, 
    STAR = 14, DIV = 15, MOD = 16, MMUL = 17, PLUS = 18, MINUS = 19, EQ = 20, 
    NE = 21, GE = 22, LE = 23, GT = 24, LT = 25, NOT = 26, AND = 27, OR = 28, 
    XOR = 29, INT = 30, ESC = 31, CHAR = 32, STRING_LIT = 33, ID = 34, M_LINE_COMM = 35, 
    S_LINE_COMM = 36, WS = 37
  };

  enum {
    RuleFile = 0, RuleExpr = 1, RuleAtom_expr = 2, RuleKeywords = 3
  };

  explicit UnderscoreParser(antlr4::TokenStream *input);

  UnderscoreParser(antlr4::TokenStream *input, const antlr4::atn::ParserATNSimulatorOptions &options);

  ~UnderscoreParser() override;

  std::string getGrammarFileName() const override;

  const antlr4::atn::ATN& getATN() const override;

  const std::vector<std::string>& getRuleNames() const override;

  const antlr4::dfa::Vocabulary& getVocabulary() const override;

  antlr4::atn::SerializedATNView getSerializedATN() const override;


  class FileContext;
  class ExprContext;
  class Atom_exprContext;
  class KeywordsContext; 

  class  FileContext : public antlr4::ParserRuleContext {
  public:
    FileContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    antlr4::tree::TerminalNode *EOF();
    ExprContext *expr();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;

    virtual std::any accept(antlr4::tree::ParseTreeVisitor *visitor) override;
   
  };

  FileContext* file();

  class  ExprContext : public antlr4::ParserRuleContext {
  public:
    ExprContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;
    Atom_exprContext *atom_expr();
    antlr4::tree::TerminalNode *SEMI();
    ExprContext *expr();

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;

    virtual std::any accept(antlr4::tree::ParseTreeVisitor *visitor) override;
   
  };

  ExprContext* expr();

  class  Atom_exprContext : public antlr4::ParserRuleContext {
  public:
    Atom_exprContext(antlr4::ParserRuleContext *parent, size_t invokingState);
   
    Atom_exprContext() = default;
    void copyFrom(Atom_exprContext *context);
    using antlr4::ParserRuleContext::copyFrom;

    virtual size_t getRuleIndex() const override;

   
  };

  class  ParenContext : public Atom_exprContext {
  public:
    ParenContext(Atom_exprContext *ctx);

    ExprContext *expr();
    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;

    virtual std::any accept(antlr4::tree::ParseTreeVisitor *visitor) override;
  };

  class  BreakContext : public Atom_exprContext {
  public:
    BreakContext(Atom_exprContext *ctx);

    Atom_exprContext *atom_expr();
    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;

    virtual std::any accept(antlr4::tree::ParseTreeVisitor *visitor) override;
  };

  class  CmpContext : public Atom_exprContext {
  public:
    CmpContext(Atom_exprContext *ctx);

    antlr4::Token *op = nullptr;
    std::vector<Atom_exprContext *> atom_expr();
    Atom_exprContext* atom_expr(size_t i);
    antlr4::tree::TerminalNode *GE();
    antlr4::tree::TerminalNode *LE();
    antlr4::tree::TerminalNode *GT();
    antlr4::tree::TerminalNode *LT();
    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;

    virtual std::any accept(antlr4::tree::ParseTreeVisitor *visitor) override;
  };

  class  AddsubContext : public Atom_exprContext {
  public:
    AddsubContext(Atom_exprContext *ctx);

    antlr4::Token *op = nullptr;
    std::vector<Atom_exprContext *> atom_expr();
    Atom_exprContext* atom_expr(size_t i);
    antlr4::tree::TerminalNode *PLUS();
    antlr4::tree::TerminalNode *MINUS();
    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;

    virtual std::any accept(antlr4::tree::ParseTreeVisitor *visitor) override;
  };

  class  IndexContext : public Atom_exprContext {
  public:
    IndexContext(Atom_exprContext *ctx);

    UnderscoreParser::ExprContext *idx = nullptr;
    Atom_exprContext *atom_expr();
    ExprContext *expr();
    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;

    virtual std::any accept(antlr4::tree::ParseTreeVisitor *visitor) override;
  };

  class  IdContext : public Atom_exprContext {
  public:
    IdContext(Atom_exprContext *ctx);

    antlr4::tree::TerminalNode *ID();
    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;

    virtual std::any accept(antlr4::tree::ParseTreeVisitor *visitor) override;
  };

  class  CondContext : public Atom_exprContext {
  public:
    CondContext(Atom_exprContext *ctx);

    UnderscoreParser::ExprContext *cond = nullptr;
    std::vector<Atom_exprContext *> atom_expr();
    Atom_exprContext* atom_expr(size_t i);
    ExprContext *expr();
    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;

    virtual std::any accept(antlr4::tree::ParseTreeVisitor *visitor) override;
  };

  class  WhileContext : public Atom_exprContext {
  public:
    WhileContext(Atom_exprContext *ctx);

    UnderscoreParser::ExprContext *cond = nullptr;
    Atom_exprContext *atom_expr();
    ExprContext *expr();
    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;

    virtual std::any accept(antlr4::tree::ParseTreeVisitor *visitor) override;
  };

  class  EqualityContext : public Atom_exprContext {
  public:
    EqualityContext(Atom_exprContext *ctx);

    antlr4::Token *op = nullptr;
    std::vector<Atom_exprContext *> atom_expr();
    Atom_exprContext* atom_expr(size_t i);
    antlr4::tree::TerminalNode *EQ();
    antlr4::tree::TerminalNode *NE();
    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;

    virtual std::any accept(antlr4::tree::ParseTreeVisitor *visitor) override;
  };

  class  IntContext : public Atom_exprContext {
  public:
    IntContext(Atom_exprContext *ctx);

    antlr4::tree::TerminalNode *INT();
    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;

    virtual std::any accept(antlr4::tree::ParseTreeVisitor *visitor) override;
  };

  class  MuldivContext : public Atom_exprContext {
  public:
    MuldivContext(Atom_exprContext *ctx);

    antlr4::Token *op = nullptr;
    std::vector<Atom_exprContext *> atom_expr();
    Atom_exprContext* atom_expr(size_t i);
    antlr4::tree::TerminalNode *STAR();
    antlr4::tree::TerminalNode *DIV();
    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;

    virtual std::any accept(antlr4::tree::ParseTreeVisitor *visitor) override;
  };

  Atom_exprContext* atom_expr();
  Atom_exprContext* atom_expr(int precedence);
  class  KeywordsContext : public antlr4::ParserRuleContext {
  public:
    KeywordsContext(antlr4::ParserRuleContext *parent, size_t invokingState);
    virtual size_t getRuleIndex() const override;

    virtual void enterRule(antlr4::tree::ParseTreeListener *listener) override;
    virtual void exitRule(antlr4::tree::ParseTreeListener *listener) override;

    virtual std::any accept(antlr4::tree::ParseTreeVisitor *visitor) override;
   
  };

  KeywordsContext* keywords();


  bool sempred(antlr4::RuleContext *_localctx, size_t ruleIndex, size_t predicateIndex) override;

  bool atom_exprSempred(Atom_exprContext *_localctx, size_t predicateIndex);

  // By default the static state used to implement the parser is lazily initialized during the first
  // call to the constructor. You can call this function if you wish to initialize the static state
  // ahead of time.
  static void initialize();

private:
};

}  // namespace underscore
