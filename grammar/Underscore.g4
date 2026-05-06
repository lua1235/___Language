grammar Underscore;

file:  expr? EOF;

expr
    : atom_expr
    | atom_expr SEMI expr
    ;

atom_expr // In descending order of precedence
    : '(' expr ')' #paren
    | atom_expr '[' idx=expr ']' #index
    | atom_expr op=(STAR|DIV) atom_expr #muldiv
    | atom_expr op=(PLUS|MINUS) atom_expr #addsub
    | atom_expr op=(GE|LE|GT|LT) atom_expr #cmp
    | atom_expr op=(EQ|NE) atom_expr #equality
    | atom_expr op=(ASS|ADDASS|SUBASS|MULASS|DIVASS) atom_expr #assn
    | 'if''('cond=expr')' atom_expr ('else' atom_expr)? #cond
    | 'while''('cond=expr')' atom_expr #while
    | 'break' (atom_expr)? #break
    | ID #id
    | INT #int
    ;
  
keywords: 'if''else''for''while''break''int';
// Lexer Rules
fragment DIGIT : [0-9];

// Make these named tokens for ease of use
SEMI : ';';
LARROW : '<-';
RARROW : '->';
STAR : '*';
DIV : '/';
MOD : '%';
MMUL : '**';
PLUS : '+';
MINUS : '-';
EQ : '==';
NE : '!=';
GE : '>=';
LE : '<=';
GT : '>';
LT : '<';
ASS : '=';
ADDASS : '+=';
SUBASS : '-=';
MULASS : '*=';
DIVASS : '/=';
NOT : 'not';
AND : 'and';
OR : 'or';
XOR : 'xor';

INT : DIGIT+ ;

ESC : '\\' [0abtnr"'\\] ;

CHAR : '\'' (ESC | .) '\'' ;

STRING_LIT : '"' (ESC | .)*? '"' ;

ID : [a-zA-Z_] [a-zA-Z0-9_]* ;


M_LINE_COMM : '/*' .*? '*/'   -> channel(HIDDEN) ;
S_LINE_COMM : '//' ~[\r\n]* -> channel(HIDDEN) ;

// Skip whitespace
WS : [ \t\r\n]+ -> channel(HIDDEN) ;
