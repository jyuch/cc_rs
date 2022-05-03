pub mod tokenize;

#[derive(Debug)]
pub enum TokenKind {
    Reserved(char),
    Num(u32),
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
}
