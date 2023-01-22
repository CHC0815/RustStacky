#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    NUMBER,
    ADD,
    SUB,
    PRINT,
    MUL,
    DIV,
    DUP,
    SWAP,
    DROP,
    EMIT,
    WORD,
    EQ,
    LT,
    GT,
    COLON,
    SEMICOLON,
    DEBUG_STACK,
    DEBUG_DICT,
    EOF,
    IF,
    ELSE,
    THEN,
    IF_WORD,
    PUTS,
    STRING,
    AND,
    OR,
    INVERT,
    MOD,
    DO,
    LOOP,
    CR,
}
