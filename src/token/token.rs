type TokenType = String;

struct MyToken {
    myType: TokenType,
    literal: str,
}

type Token = MyToken;

enum Tokens {
    ILLEGAL,
    EOF,

    //Identifiers + Literals
    IDENT,
    INT,

    //Operators
    ASSIGN,
    PLUS,

    //Delimiters
    COMMA, 
    SEMICOLON

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    //KEYWORDS
    FUNCTION,
    LET
}