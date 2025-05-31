/*
Common features shared between the different components of the compiler
*/
enum Token {
    Id(String),
    // Keywords
    While,
    If,
    Else,
    IntKey,
    Ret,
    Op(char), // Any operator, or in c standard terms, any punctuator
}
