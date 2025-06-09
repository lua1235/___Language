/*
Common features shared between the different components of the compiler
*/
#[derive(Debug, Clone)]
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
    Ret,
    // Value tokens
    Id(String),
    IntConst(i32),
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
