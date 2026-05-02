
// Generated from /home/austin/compilers/___Language/grammar/Underscore.g4 by ANTLR 4.13.0


#include "UnderscoreListener.h"
#include "UnderscoreVisitor.h"

#include "UnderscoreParser.h"


using namespace antlrcpp;
using namespace underscore;

using namespace antlr4;

namespace {

struct UnderscoreParserStaticData final {
  UnderscoreParserStaticData(std::vector<std::string> ruleNames,
                        std::vector<std::string> literalNames,
                        std::vector<std::string> symbolicNames)
      : ruleNames(std::move(ruleNames)), literalNames(std::move(literalNames)),
        symbolicNames(std::move(symbolicNames)),
        vocabulary(this->literalNames, this->symbolicNames) {}

  UnderscoreParserStaticData(const UnderscoreParserStaticData&) = delete;
  UnderscoreParserStaticData(UnderscoreParserStaticData&&) = delete;
  UnderscoreParserStaticData& operator=(const UnderscoreParserStaticData&) = delete;
  UnderscoreParserStaticData& operator=(UnderscoreParserStaticData&&) = delete;

  std::vector<antlr4::dfa::DFA> decisionToDFA;
  antlr4::atn::PredictionContextCache sharedContextCache;
  const std::vector<std::string> ruleNames;
  const std::vector<std::string> literalNames;
  const std::vector<std::string> symbolicNames;
  const antlr4::dfa::Vocabulary vocabulary;
  antlr4::atn::SerializedATNView serializedATN;
  std::unique_ptr<antlr4::atn::ATN> atn;
};

::antlr4::internal::OnceFlag underscoreParserOnceFlag;
#if ANTLR4_USE_THREAD_LOCAL_CACHE
static thread_local
#endif
UnderscoreParserStaticData *underscoreParserStaticData = nullptr;

void underscoreParserInitialize() {
#if ANTLR4_USE_THREAD_LOCAL_CACHE
  if (underscoreParserStaticData != nullptr) {
    return;
  }
#else
  assert(underscoreParserStaticData == nullptr);
#endif
  auto staticData = std::make_unique<UnderscoreParserStaticData>(
    std::vector<std::string>{
      "file", "expr", "atom_expr", "keywords"
    },
    std::vector<std::string>{
      "", "'('", "')'", "'['", "']'", "'if'", "'else'", "'while'", "'break'", 
      "'for'", "'int'", "';'", "'<-'", "'->'", "'*'", "'/'", "'%'", "'**'", 
      "'+'", "'-'", "'=='", "'!='", "'>='", "'<='", "'>'", "'<'", "'not'", 
      "'and'", "'or'", "'xor'"
    },
    std::vector<std::string>{
      "", "", "", "", "", "", "", "", "", "", "", "SEMI", "LARROW", "RARROW", 
      "STAR", "DIV", "MOD", "MMUL", "PLUS", "MINUS", "EQ", "NE", "GE", "LE", 
      "GT", "LT", "NOT", "AND", "OR", "XOR", "INT", "ESC", "CHAR", "STRING_LIT", 
      "ID", "M_LINE_COMM", "S_LINE_COMM", "WS"
    }
  );
  static const int32_t serializedATNSegment[] = {
  	4,1,37,78,2,0,7,0,2,1,7,1,2,2,7,2,2,3,7,3,1,0,3,0,10,8,0,1,0,1,0,1,1,
  	1,1,1,1,1,1,1,1,3,1,19,8,1,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,
  	2,1,2,3,2,33,8,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,3,2,43,8,2,1,2,1,2,3,
  	2,47,8,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,2,
  	1,2,1,2,5,2,66,8,2,10,2,12,2,69,9,2,1,3,1,3,1,3,1,3,1,3,1,3,1,3,1,3,0,
  	1,4,4,0,2,4,6,0,4,1,0,14,15,1,0,18,19,1,0,22,25,1,0,20,21,87,0,9,1,0,
  	0,0,2,18,1,0,0,0,4,46,1,0,0,0,6,70,1,0,0,0,8,10,3,2,1,0,9,8,1,0,0,0,9,
  	10,1,0,0,0,10,11,1,0,0,0,11,12,5,0,0,1,12,1,1,0,0,0,13,19,3,4,2,0,14,
  	15,3,4,2,0,15,16,5,11,0,0,16,17,3,2,1,0,17,19,1,0,0,0,18,13,1,0,0,0,18,
  	14,1,0,0,0,19,3,1,0,0,0,20,21,6,2,-1,0,21,22,5,1,0,0,22,23,3,2,1,0,23,
  	24,5,2,0,0,24,47,1,0,0,0,25,26,5,5,0,0,26,27,5,1,0,0,27,28,3,2,1,0,28,
  	29,5,2,0,0,29,32,3,4,2,0,30,31,5,6,0,0,31,33,3,4,2,0,32,30,1,0,0,0,32,
  	33,1,0,0,0,33,47,1,0,0,0,34,35,5,7,0,0,35,36,5,1,0,0,36,37,3,2,1,0,37,
  	38,5,2,0,0,38,39,3,4,2,4,39,47,1,0,0,0,40,42,5,8,0,0,41,43,3,4,2,0,42,
  	41,1,0,0,0,42,43,1,0,0,0,43,47,1,0,0,0,44,47,5,34,0,0,45,47,5,30,0,0,
  	46,20,1,0,0,0,46,25,1,0,0,0,46,34,1,0,0,0,46,40,1,0,0,0,46,44,1,0,0,0,
  	46,45,1,0,0,0,47,67,1,0,0,0,48,49,10,9,0,0,49,50,7,0,0,0,50,66,3,4,2,
  	10,51,52,10,8,0,0,52,53,7,1,0,0,53,66,3,4,2,9,54,55,10,7,0,0,55,56,7,
  	2,0,0,56,66,3,4,2,8,57,58,10,6,0,0,58,59,7,3,0,0,59,66,3,4,2,7,60,61,
  	10,10,0,0,61,62,5,3,0,0,62,63,3,2,1,0,63,64,5,4,0,0,64,66,1,0,0,0,65,
  	48,1,0,0,0,65,51,1,0,0,0,65,54,1,0,0,0,65,57,1,0,0,0,65,60,1,0,0,0,66,
  	69,1,0,0,0,67,65,1,0,0,0,67,68,1,0,0,0,68,5,1,0,0,0,69,67,1,0,0,0,70,
  	71,5,5,0,0,71,72,5,6,0,0,72,73,5,9,0,0,73,74,5,7,0,0,74,75,5,8,0,0,75,
  	76,5,10,0,0,76,7,1,0,0,0,7,9,18,32,42,46,65,67
  };
  staticData->serializedATN = antlr4::atn::SerializedATNView(serializedATNSegment, sizeof(serializedATNSegment) / sizeof(serializedATNSegment[0]));

  antlr4::atn::ATNDeserializer deserializer;
  staticData->atn = deserializer.deserialize(staticData->serializedATN);

  const size_t count = staticData->atn->getNumberOfDecisions();
  staticData->decisionToDFA.reserve(count);
  for (size_t i = 0; i < count; i++) { 
    staticData->decisionToDFA.emplace_back(staticData->atn->getDecisionState(i), i);
  }
  underscoreParserStaticData = staticData.release();
}

}

UnderscoreParser::UnderscoreParser(TokenStream *input) : UnderscoreParser(input, antlr4::atn::ParserATNSimulatorOptions()) {}

UnderscoreParser::UnderscoreParser(TokenStream *input, const antlr4::atn::ParserATNSimulatorOptions &options) : Parser(input) {
  UnderscoreParser::initialize();
  _interpreter = new atn::ParserATNSimulator(this, *underscoreParserStaticData->atn, underscoreParserStaticData->decisionToDFA, underscoreParserStaticData->sharedContextCache, options);
}

UnderscoreParser::~UnderscoreParser() {
  delete _interpreter;
}

const atn::ATN& UnderscoreParser::getATN() const {
  return *underscoreParserStaticData->atn;
}

std::string UnderscoreParser::getGrammarFileName() const {
  return "Underscore.g4";
}

const std::vector<std::string>& UnderscoreParser::getRuleNames() const {
  return underscoreParserStaticData->ruleNames;
}

const dfa::Vocabulary& UnderscoreParser::getVocabulary() const {
  return underscoreParserStaticData->vocabulary;
}

antlr4::atn::SerializedATNView UnderscoreParser::getSerializedATN() const {
  return underscoreParserStaticData->serializedATN;
}


//----------------- FileContext ------------------------------------------------------------------

UnderscoreParser::FileContext::FileContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

tree::TerminalNode* UnderscoreParser::FileContext::EOF() {
  return getToken(UnderscoreParser::EOF, 0);
}

UnderscoreParser::ExprContext* UnderscoreParser::FileContext::expr() {
  return getRuleContext<UnderscoreParser::ExprContext>(0);
}


size_t UnderscoreParser::FileContext::getRuleIndex() const {
  return UnderscoreParser::RuleFile;
}

void UnderscoreParser::FileContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterFile(this);
}

void UnderscoreParser::FileContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitFile(this);
}


std::any UnderscoreParser::FileContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<UnderscoreVisitor*>(visitor))
    return parserVisitor->visitFile(this);
  else
    return visitor->visitChildren(this);
}

UnderscoreParser::FileContext* UnderscoreParser::file() {
  FileContext *_localctx = _tracker.createInstance<FileContext>(_ctx, getState());
  enterRule(_localctx, 0, UnderscoreParser::RuleFile);
  size_t _la = 0;

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(9);
    _errHandler->sync(this);

    _la = _input->LA(1);
    if ((((_la & ~ 0x3fULL) == 0) &&
      ((1ULL << _la) & 18253611426) != 0)) {
      setState(8);
      expr();
    }
    setState(11);
    match(UnderscoreParser::EOF);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- ExprContext ------------------------------------------------------------------

UnderscoreParser::ExprContext::ExprContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}

UnderscoreParser::Atom_exprContext* UnderscoreParser::ExprContext::atom_expr() {
  return getRuleContext<UnderscoreParser::Atom_exprContext>(0);
}

tree::TerminalNode* UnderscoreParser::ExprContext::SEMI() {
  return getToken(UnderscoreParser::SEMI, 0);
}

UnderscoreParser::ExprContext* UnderscoreParser::ExprContext::expr() {
  return getRuleContext<UnderscoreParser::ExprContext>(0);
}


size_t UnderscoreParser::ExprContext::getRuleIndex() const {
  return UnderscoreParser::RuleExpr;
}

void UnderscoreParser::ExprContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterExpr(this);
}

void UnderscoreParser::ExprContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitExpr(this);
}


std::any UnderscoreParser::ExprContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<UnderscoreVisitor*>(visitor))
    return parserVisitor->visitExpr(this);
  else
    return visitor->visitChildren(this);
}

UnderscoreParser::ExprContext* UnderscoreParser::expr() {
  ExprContext *_localctx = _tracker.createInstance<ExprContext>(_ctx, getState());
  enterRule(_localctx, 2, UnderscoreParser::RuleExpr);

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    setState(18);
    _errHandler->sync(this);
    switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 1, _ctx)) {
    case 1: {
      enterOuterAlt(_localctx, 1);
      setState(13);
      atom_expr(0);
      break;
    }

    case 2: {
      enterOuterAlt(_localctx, 2);
      setState(14);
      atom_expr(0);
      setState(15);
      match(UnderscoreParser::SEMI);
      setState(16);
      expr();
      break;
    }

    default:
      break;
    }
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

//----------------- Atom_exprContext ------------------------------------------------------------------

UnderscoreParser::Atom_exprContext::Atom_exprContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}


size_t UnderscoreParser::Atom_exprContext::getRuleIndex() const {
  return UnderscoreParser::RuleAtom_expr;
}

void UnderscoreParser::Atom_exprContext::copyFrom(Atom_exprContext *ctx) {
  ParserRuleContext::copyFrom(ctx);
}

//----------------- ParenContext ------------------------------------------------------------------

UnderscoreParser::ExprContext* UnderscoreParser::ParenContext::expr() {
  return getRuleContext<UnderscoreParser::ExprContext>(0);
}

UnderscoreParser::ParenContext::ParenContext(Atom_exprContext *ctx) { copyFrom(ctx); }

void UnderscoreParser::ParenContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterParen(this);
}
void UnderscoreParser::ParenContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitParen(this);
}

std::any UnderscoreParser::ParenContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<UnderscoreVisitor*>(visitor))
    return parserVisitor->visitParen(this);
  else
    return visitor->visitChildren(this);
}
//----------------- BreakContext ------------------------------------------------------------------

UnderscoreParser::Atom_exprContext* UnderscoreParser::BreakContext::atom_expr() {
  return getRuleContext<UnderscoreParser::Atom_exprContext>(0);
}

UnderscoreParser::BreakContext::BreakContext(Atom_exprContext *ctx) { copyFrom(ctx); }

void UnderscoreParser::BreakContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterBreak(this);
}
void UnderscoreParser::BreakContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitBreak(this);
}

std::any UnderscoreParser::BreakContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<UnderscoreVisitor*>(visitor))
    return parserVisitor->visitBreak(this);
  else
    return visitor->visitChildren(this);
}
//----------------- CmpContext ------------------------------------------------------------------

std::vector<UnderscoreParser::Atom_exprContext *> UnderscoreParser::CmpContext::atom_expr() {
  return getRuleContexts<UnderscoreParser::Atom_exprContext>();
}

UnderscoreParser::Atom_exprContext* UnderscoreParser::CmpContext::atom_expr(size_t i) {
  return getRuleContext<UnderscoreParser::Atom_exprContext>(i);
}

tree::TerminalNode* UnderscoreParser::CmpContext::GE() {
  return getToken(UnderscoreParser::GE, 0);
}

tree::TerminalNode* UnderscoreParser::CmpContext::LE() {
  return getToken(UnderscoreParser::LE, 0);
}

tree::TerminalNode* UnderscoreParser::CmpContext::GT() {
  return getToken(UnderscoreParser::GT, 0);
}

tree::TerminalNode* UnderscoreParser::CmpContext::LT() {
  return getToken(UnderscoreParser::LT, 0);
}

UnderscoreParser::CmpContext::CmpContext(Atom_exprContext *ctx) { copyFrom(ctx); }

void UnderscoreParser::CmpContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterCmp(this);
}
void UnderscoreParser::CmpContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitCmp(this);
}

std::any UnderscoreParser::CmpContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<UnderscoreVisitor*>(visitor))
    return parserVisitor->visitCmp(this);
  else
    return visitor->visitChildren(this);
}
//----------------- AddsubContext ------------------------------------------------------------------

std::vector<UnderscoreParser::Atom_exprContext *> UnderscoreParser::AddsubContext::atom_expr() {
  return getRuleContexts<UnderscoreParser::Atom_exprContext>();
}

UnderscoreParser::Atom_exprContext* UnderscoreParser::AddsubContext::atom_expr(size_t i) {
  return getRuleContext<UnderscoreParser::Atom_exprContext>(i);
}

tree::TerminalNode* UnderscoreParser::AddsubContext::PLUS() {
  return getToken(UnderscoreParser::PLUS, 0);
}

tree::TerminalNode* UnderscoreParser::AddsubContext::MINUS() {
  return getToken(UnderscoreParser::MINUS, 0);
}

UnderscoreParser::AddsubContext::AddsubContext(Atom_exprContext *ctx) { copyFrom(ctx); }

void UnderscoreParser::AddsubContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterAddsub(this);
}
void UnderscoreParser::AddsubContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitAddsub(this);
}

std::any UnderscoreParser::AddsubContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<UnderscoreVisitor*>(visitor))
    return parserVisitor->visitAddsub(this);
  else
    return visitor->visitChildren(this);
}
//----------------- IndexContext ------------------------------------------------------------------

UnderscoreParser::Atom_exprContext* UnderscoreParser::IndexContext::atom_expr() {
  return getRuleContext<UnderscoreParser::Atom_exprContext>(0);
}

UnderscoreParser::ExprContext* UnderscoreParser::IndexContext::expr() {
  return getRuleContext<UnderscoreParser::ExprContext>(0);
}

UnderscoreParser::IndexContext::IndexContext(Atom_exprContext *ctx) { copyFrom(ctx); }

void UnderscoreParser::IndexContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterIndex(this);
}
void UnderscoreParser::IndexContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitIndex(this);
}

std::any UnderscoreParser::IndexContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<UnderscoreVisitor*>(visitor))
    return parserVisitor->visitIndex(this);
  else
    return visitor->visitChildren(this);
}
//----------------- IdContext ------------------------------------------------------------------

tree::TerminalNode* UnderscoreParser::IdContext::ID() {
  return getToken(UnderscoreParser::ID, 0);
}

UnderscoreParser::IdContext::IdContext(Atom_exprContext *ctx) { copyFrom(ctx); }

void UnderscoreParser::IdContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterId(this);
}
void UnderscoreParser::IdContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitId(this);
}

std::any UnderscoreParser::IdContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<UnderscoreVisitor*>(visitor))
    return parserVisitor->visitId(this);
  else
    return visitor->visitChildren(this);
}
//----------------- CondContext ------------------------------------------------------------------

std::vector<UnderscoreParser::Atom_exprContext *> UnderscoreParser::CondContext::atom_expr() {
  return getRuleContexts<UnderscoreParser::Atom_exprContext>();
}

UnderscoreParser::Atom_exprContext* UnderscoreParser::CondContext::atom_expr(size_t i) {
  return getRuleContext<UnderscoreParser::Atom_exprContext>(i);
}

UnderscoreParser::ExprContext* UnderscoreParser::CondContext::expr() {
  return getRuleContext<UnderscoreParser::ExprContext>(0);
}

UnderscoreParser::CondContext::CondContext(Atom_exprContext *ctx) { copyFrom(ctx); }

void UnderscoreParser::CondContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterCond(this);
}
void UnderscoreParser::CondContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitCond(this);
}

std::any UnderscoreParser::CondContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<UnderscoreVisitor*>(visitor))
    return parserVisitor->visitCond(this);
  else
    return visitor->visitChildren(this);
}
//----------------- WhileContext ------------------------------------------------------------------

UnderscoreParser::Atom_exprContext* UnderscoreParser::WhileContext::atom_expr() {
  return getRuleContext<UnderscoreParser::Atom_exprContext>(0);
}

UnderscoreParser::ExprContext* UnderscoreParser::WhileContext::expr() {
  return getRuleContext<UnderscoreParser::ExprContext>(0);
}

UnderscoreParser::WhileContext::WhileContext(Atom_exprContext *ctx) { copyFrom(ctx); }

void UnderscoreParser::WhileContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterWhile(this);
}
void UnderscoreParser::WhileContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitWhile(this);
}

std::any UnderscoreParser::WhileContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<UnderscoreVisitor*>(visitor))
    return parserVisitor->visitWhile(this);
  else
    return visitor->visitChildren(this);
}
//----------------- EqualityContext ------------------------------------------------------------------

std::vector<UnderscoreParser::Atom_exprContext *> UnderscoreParser::EqualityContext::atom_expr() {
  return getRuleContexts<UnderscoreParser::Atom_exprContext>();
}

UnderscoreParser::Atom_exprContext* UnderscoreParser::EqualityContext::atom_expr(size_t i) {
  return getRuleContext<UnderscoreParser::Atom_exprContext>(i);
}

tree::TerminalNode* UnderscoreParser::EqualityContext::EQ() {
  return getToken(UnderscoreParser::EQ, 0);
}

tree::TerminalNode* UnderscoreParser::EqualityContext::NE() {
  return getToken(UnderscoreParser::NE, 0);
}

UnderscoreParser::EqualityContext::EqualityContext(Atom_exprContext *ctx) { copyFrom(ctx); }

void UnderscoreParser::EqualityContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterEquality(this);
}
void UnderscoreParser::EqualityContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitEquality(this);
}

std::any UnderscoreParser::EqualityContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<UnderscoreVisitor*>(visitor))
    return parserVisitor->visitEquality(this);
  else
    return visitor->visitChildren(this);
}
//----------------- IntContext ------------------------------------------------------------------

tree::TerminalNode* UnderscoreParser::IntContext::INT() {
  return getToken(UnderscoreParser::INT, 0);
}

UnderscoreParser::IntContext::IntContext(Atom_exprContext *ctx) { copyFrom(ctx); }

void UnderscoreParser::IntContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterInt(this);
}
void UnderscoreParser::IntContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitInt(this);
}

std::any UnderscoreParser::IntContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<UnderscoreVisitor*>(visitor))
    return parserVisitor->visitInt(this);
  else
    return visitor->visitChildren(this);
}
//----------------- MuldivContext ------------------------------------------------------------------

std::vector<UnderscoreParser::Atom_exprContext *> UnderscoreParser::MuldivContext::atom_expr() {
  return getRuleContexts<UnderscoreParser::Atom_exprContext>();
}

UnderscoreParser::Atom_exprContext* UnderscoreParser::MuldivContext::atom_expr(size_t i) {
  return getRuleContext<UnderscoreParser::Atom_exprContext>(i);
}

tree::TerminalNode* UnderscoreParser::MuldivContext::STAR() {
  return getToken(UnderscoreParser::STAR, 0);
}

tree::TerminalNode* UnderscoreParser::MuldivContext::DIV() {
  return getToken(UnderscoreParser::DIV, 0);
}

UnderscoreParser::MuldivContext::MuldivContext(Atom_exprContext *ctx) { copyFrom(ctx); }

void UnderscoreParser::MuldivContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterMuldiv(this);
}
void UnderscoreParser::MuldivContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitMuldiv(this);
}

std::any UnderscoreParser::MuldivContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<UnderscoreVisitor*>(visitor))
    return parserVisitor->visitMuldiv(this);
  else
    return visitor->visitChildren(this);
}

UnderscoreParser::Atom_exprContext* UnderscoreParser::atom_expr() {
   return atom_expr(0);
}

UnderscoreParser::Atom_exprContext* UnderscoreParser::atom_expr(int precedence) {
  ParserRuleContext *parentContext = _ctx;
  size_t parentState = getState();
  UnderscoreParser::Atom_exprContext *_localctx = _tracker.createInstance<Atom_exprContext>(_ctx, parentState);
  UnderscoreParser::Atom_exprContext *previousContext = _localctx;
  (void)previousContext; // Silence compiler, in case the context is not used by generated code.
  size_t startState = 4;
  enterRecursionRule(_localctx, 4, UnderscoreParser::RuleAtom_expr, precedence);

    size_t _la = 0;

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    unrollRecursionContexts(parentContext);
  });
  try {
    size_t alt;
    enterOuterAlt(_localctx, 1);
    setState(46);
    _errHandler->sync(this);
    switch (_input->LA(1)) {
      case UnderscoreParser::T__0: {
        _localctx = _tracker.createInstance<ParenContext>(_localctx);
        _ctx = _localctx;
        previousContext = _localctx;

        setState(21);
        match(UnderscoreParser::T__0);
        setState(22);
        expr();
        setState(23);
        match(UnderscoreParser::T__1);
        break;
      }

      case UnderscoreParser::T__4: {
        _localctx = _tracker.createInstance<CondContext>(_localctx);
        _ctx = _localctx;
        previousContext = _localctx;
        setState(25);
        match(UnderscoreParser::T__4);
        setState(26);
        match(UnderscoreParser::T__0);
        setState(27);
        antlrcpp::downCast<CondContext *>(_localctx)->cond = expr();
        setState(28);
        match(UnderscoreParser::T__1);
        setState(29);
        atom_expr(0);
        setState(32);
        _errHandler->sync(this);

        switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 2, _ctx)) {
        case 1: {
          setState(30);
          match(UnderscoreParser::T__5);
          setState(31);
          atom_expr(0);
          break;
        }

        default:
          break;
        }
        break;
      }

      case UnderscoreParser::T__6: {
        _localctx = _tracker.createInstance<WhileContext>(_localctx);
        _ctx = _localctx;
        previousContext = _localctx;
        setState(34);
        match(UnderscoreParser::T__6);
        setState(35);
        match(UnderscoreParser::T__0);
        setState(36);
        antlrcpp::downCast<WhileContext *>(_localctx)->cond = expr();
        setState(37);
        match(UnderscoreParser::T__1);
        setState(38);
        atom_expr(4);
        break;
      }

      case UnderscoreParser::T__7: {
        _localctx = _tracker.createInstance<BreakContext>(_localctx);
        _ctx = _localctx;
        previousContext = _localctx;
        setState(40);
        match(UnderscoreParser::T__7);
        setState(42);
        _errHandler->sync(this);

        switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 3, _ctx)) {
        case 1: {
          setState(41);
          atom_expr(0);
          break;
        }

        default:
          break;
        }
        break;
      }

      case UnderscoreParser::ID: {
        _localctx = _tracker.createInstance<IdContext>(_localctx);
        _ctx = _localctx;
        previousContext = _localctx;
        setState(44);
        match(UnderscoreParser::ID);
        break;
      }

      case UnderscoreParser::INT: {
        _localctx = _tracker.createInstance<IntContext>(_localctx);
        _ctx = _localctx;
        previousContext = _localctx;
        setState(45);
        match(UnderscoreParser::INT);
        break;
      }

    default:
      throw NoViableAltException(this);
    }
    _ctx->stop = _input->LT(-1);
    setState(67);
    _errHandler->sync(this);
    alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 6, _ctx);
    while (alt != 2 && alt != atn::ATN::INVALID_ALT_NUMBER) {
      if (alt == 1) {
        if (!_parseListeners.empty())
          triggerExitRuleEvent();
        previousContext = _localctx;
        setState(65);
        _errHandler->sync(this);
        switch (getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 5, _ctx)) {
        case 1: {
          auto newContext = _tracker.createInstance<MuldivContext>(_tracker.createInstance<Atom_exprContext>(parentContext, parentState));
          _localctx = newContext;
          pushNewRecursionContext(newContext, startState, RuleAtom_expr);
          setState(48);

          if (!(precpred(_ctx, 9))) throw FailedPredicateException(this, "precpred(_ctx, 9)");
          setState(49);
          antlrcpp::downCast<MuldivContext *>(_localctx)->op = _input->LT(1);
          _la = _input->LA(1);
          if (!(_la == UnderscoreParser::STAR

          || _la == UnderscoreParser::DIV)) {
            antlrcpp::downCast<MuldivContext *>(_localctx)->op = _errHandler->recoverInline(this);
          }
          else {
            _errHandler->reportMatch(this);
            consume();
          }
          setState(50);
          atom_expr(10);
          break;
        }

        case 2: {
          auto newContext = _tracker.createInstance<AddsubContext>(_tracker.createInstance<Atom_exprContext>(parentContext, parentState));
          _localctx = newContext;
          pushNewRecursionContext(newContext, startState, RuleAtom_expr);
          setState(51);

          if (!(precpred(_ctx, 8))) throw FailedPredicateException(this, "precpred(_ctx, 8)");
          setState(52);
          antlrcpp::downCast<AddsubContext *>(_localctx)->op = _input->LT(1);
          _la = _input->LA(1);
          if (!(_la == UnderscoreParser::PLUS

          || _la == UnderscoreParser::MINUS)) {
            antlrcpp::downCast<AddsubContext *>(_localctx)->op = _errHandler->recoverInline(this);
          }
          else {
            _errHandler->reportMatch(this);
            consume();
          }
          setState(53);
          atom_expr(9);
          break;
        }

        case 3: {
          auto newContext = _tracker.createInstance<CmpContext>(_tracker.createInstance<Atom_exprContext>(parentContext, parentState));
          _localctx = newContext;
          pushNewRecursionContext(newContext, startState, RuleAtom_expr);
          setState(54);

          if (!(precpred(_ctx, 7))) throw FailedPredicateException(this, "precpred(_ctx, 7)");
          setState(55);
          antlrcpp::downCast<CmpContext *>(_localctx)->op = _input->LT(1);
          _la = _input->LA(1);
          if (!((((_la & ~ 0x3fULL) == 0) &&
            ((1ULL << _la) & 62914560) != 0))) {
            antlrcpp::downCast<CmpContext *>(_localctx)->op = _errHandler->recoverInline(this);
          }
          else {
            _errHandler->reportMatch(this);
            consume();
          }
          setState(56);
          atom_expr(8);
          break;
        }

        case 4: {
          auto newContext = _tracker.createInstance<EqualityContext>(_tracker.createInstance<Atom_exprContext>(parentContext, parentState));
          _localctx = newContext;
          pushNewRecursionContext(newContext, startState, RuleAtom_expr);
          setState(57);

          if (!(precpred(_ctx, 6))) throw FailedPredicateException(this, "precpred(_ctx, 6)");
          setState(58);
          antlrcpp::downCast<EqualityContext *>(_localctx)->op = _input->LT(1);
          _la = _input->LA(1);
          if (!(_la == UnderscoreParser::EQ

          || _la == UnderscoreParser::NE)) {
            antlrcpp::downCast<EqualityContext *>(_localctx)->op = _errHandler->recoverInline(this);
          }
          else {
            _errHandler->reportMatch(this);
            consume();
          }
          setState(59);
          atom_expr(7);
          break;
        }

        case 5: {
          auto newContext = _tracker.createInstance<IndexContext>(_tracker.createInstance<Atom_exprContext>(parentContext, parentState));
          _localctx = newContext;
          pushNewRecursionContext(newContext, startState, RuleAtom_expr);
          setState(60);

          if (!(precpred(_ctx, 10))) throw FailedPredicateException(this, "precpred(_ctx, 10)");
          setState(61);
          match(UnderscoreParser::T__2);
          setState(62);
          antlrcpp::downCast<IndexContext *>(_localctx)->idx = expr();
          setState(63);
          match(UnderscoreParser::T__3);
          break;
        }

        default:
          break;
        } 
      }
      setState(69);
      _errHandler->sync(this);
      alt = getInterpreter<atn::ParserATNSimulator>()->adaptivePredict(_input, 6, _ctx);
    }
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }
  return _localctx;
}

//----------------- KeywordsContext ------------------------------------------------------------------

UnderscoreParser::KeywordsContext::KeywordsContext(ParserRuleContext *parent, size_t invokingState)
  : ParserRuleContext(parent, invokingState) {
}


size_t UnderscoreParser::KeywordsContext::getRuleIndex() const {
  return UnderscoreParser::RuleKeywords;
}

void UnderscoreParser::KeywordsContext::enterRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->enterKeywords(this);
}

void UnderscoreParser::KeywordsContext::exitRule(tree::ParseTreeListener *listener) {
  auto parserListener = dynamic_cast<UnderscoreListener *>(listener);
  if (parserListener != nullptr)
    parserListener->exitKeywords(this);
}


std::any UnderscoreParser::KeywordsContext::accept(tree::ParseTreeVisitor *visitor) {
  if (auto parserVisitor = dynamic_cast<UnderscoreVisitor*>(visitor))
    return parserVisitor->visitKeywords(this);
  else
    return visitor->visitChildren(this);
}

UnderscoreParser::KeywordsContext* UnderscoreParser::keywords() {
  KeywordsContext *_localctx = _tracker.createInstance<KeywordsContext>(_ctx, getState());
  enterRule(_localctx, 6, UnderscoreParser::RuleKeywords);

#if __cplusplus > 201703L
  auto onExit = finally([=, this] {
#else
  auto onExit = finally([=] {
#endif
    exitRule();
  });
  try {
    enterOuterAlt(_localctx, 1);
    setState(70);
    match(UnderscoreParser::T__4);
    setState(71);
    match(UnderscoreParser::T__5);
    setState(72);
    match(UnderscoreParser::T__8);
    setState(73);
    match(UnderscoreParser::T__6);
    setState(74);
    match(UnderscoreParser::T__7);
    setState(75);
    match(UnderscoreParser::T__9);
   
  }
  catch (RecognitionException &e) {
    _errHandler->reportError(this, e);
    _localctx->exception = std::current_exception();
    _errHandler->recover(this, _localctx->exception);
  }

  return _localctx;
}

bool UnderscoreParser::sempred(RuleContext *context, size_t ruleIndex, size_t predicateIndex) {
  switch (ruleIndex) {
    case 2: return atom_exprSempred(antlrcpp::downCast<Atom_exprContext *>(context), predicateIndex);

  default:
    break;
  }
  return true;
}

bool UnderscoreParser::atom_exprSempred(Atom_exprContext *_localctx, size_t predicateIndex) {
  switch (predicateIndex) {
    case 0: return precpred(_ctx, 9);
    case 1: return precpred(_ctx, 8);
    case 2: return precpred(_ctx, 7);
    case 3: return precpred(_ctx, 6);
    case 4: return precpred(_ctx, 10);

  default:
    break;
  }
  return true;
}

void UnderscoreParser::initialize() {
#if ANTLR4_USE_THREAD_LOCAL_CACHE
  underscoreParserInitialize();
#else
  ::antlr4::internal::call_once(underscoreParserOnceFlag, underscoreParserInitialize);
#endif
}
