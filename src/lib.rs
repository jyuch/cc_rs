pub mod parse;
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

impl Token {
    #[allow(unused)]
    pub fn reserved(op: char) -> Self {
        Self {
            kind: TokenKind::Reserved(op),
        }
    }

    #[allow(unused)]
    pub fn num(value: u32) -> Self {
        Self {
            kind: TokenKind::Num(value),
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Add { lhs: Box<Node>, rhs: Box<Node> },
    Sub { lhs: Box<Node>, rhs: Box<Node> },
    Mul { lhs: Box<Node>, rhs: Box<Node> },
    Div { lhs: Box<Node>, rhs: Box<Node> },
    Num(u32),
}

impl Node {
    fn add(lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
        Box::new(Node::Add { lhs, rhs })
    }

    fn sub(lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
        Box::new(Self::Sub { lhs, rhs })
    }

    fn mul(lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
        Box::new(Self::Mul { lhs, rhs })
    }

    fn div(lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
        Box::new(Self::Div { lhs, rhs })
    }

    fn num(value: u32) -> Box<Self> {
        Box::new(Self::Num(value))
    }
}
