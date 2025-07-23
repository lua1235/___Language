#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    // Meta Control Tokens
    EOL, // End of line
    EOF, // End of line
    INVAL, // Invalid token
    // Keywords
    While,
    If,
    Else,
    IntKey,
    CharKey,
    Ret,
    // Value tokens
    IntConst(i32),
    CharConst(char),
    StrConst(String),
    Id(String),
    // Operators
    LCurly, // {
    RCurly, // }
    LParen, // (
    RParen, // )
    LBrack, // [
    RBrack, // ]
    Semi, // ;
    Colon, // :
    Comma, // ,

    Add, // +
    Inc, // ++
    AddAss, // +=
              
    Sub, // -
    Dec, // --
    SubAss, // -= 

    Star, // *
    MulAss, // *=

    Div, // /
    DivAss, // /=


    Assign, // =
    Equal, // ==
           
    Not, // !
    NotEq, // !=

    GT, // >
    GE, // >=

    LT, // <
    LE, // <=
}
