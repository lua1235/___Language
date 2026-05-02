
// Generated from /home/austin/compilers/___Language/grammar/Underscore.g4 by ANTLR 4.13.0

#pragma once


#include "antlr4-runtime.h"


namespace underscore {


class  UnderscoreLexer : public antlr4::Lexer {
public:
  enum {
    T__0 = 1, T__1 = 2, T__2 = 3, T__3 = 4, T__4 = 5, T__5 = 6, T__6 = 7, 
    T__7 = 8, T__8 = 9, T__9 = 10, SEMI = 11, LARROW = 12, RARROW = 13, 
    STAR = 14, DIV = 15, MOD = 16, MMUL = 17, PLUS = 18, MINUS = 19, EQ = 20, 
    NE = 21, GE = 22, LE = 23, GT = 24, LT = 25, NOT = 26, AND = 27, OR = 28, 
    XOR = 29, INT = 30, ESC = 31, CHAR = 32, STRING_LIT = 33, ID = 34, M_LINE_COMM = 35, 
    S_LINE_COMM = 36, WS = 37
  };

  explicit UnderscoreLexer(antlr4::CharStream *input);

  ~UnderscoreLexer() override;


  std::string getGrammarFileName() const override;

  const std::vector<std::string>& getRuleNames() const override;

  const std::vector<std::string>& getChannelNames() const override;

  const std::vector<std::string>& getModeNames() const override;

  const antlr4::dfa::Vocabulary& getVocabulary() const override;

  antlr4::atn::SerializedATNView getSerializedATN() const override;

  const antlr4::atn::ATN& getATN() const override;

  // By default the static state used to implement the lexer is lazily initialized during the first
  // call to the constructor. You can call this function if you wish to initialize the static state
  // ahead of time.
  static void initialize();

private:

  // Individual action functions triggered by action() above.

  // Individual semantic predicate functions triggered by sempred() above.

};

}  // namespace underscore
