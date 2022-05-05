pub mod generator;
pub mod parse;
pub mod tokenize;

#[derive(Debug)]
pub enum TokenKind {
    Reserved(String),
    Num(u32),
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
}

impl Token {
    pub fn reserved(op: String) -> Self {
        Self {
            kind: TokenKind::Reserved(op),
        }
    }

    pub fn num(value: u32) -> Self {
        Self {
            kind: TokenKind::Num(value),
        }
    }
}

#[derive(Debug)]
pub enum Node {
    /// 加算
    Add { lhs: Box<Node>, rhs: Box<Node> },
    /// 減算
    Sub { lhs: Box<Node>, rhs: Box<Node> },
    /// 乗算
    Mul { lhs: Box<Node>, rhs: Box<Node> },
    /// 除算
    Div { lhs: Box<Node>, rhs: Box<Node> },
    /// 等しい
    Eq { lhs: Box<Node>, rhs: Box<Node> },
    /// 等しくない
    Ne { lhs: Box<Node>, rhs: Box<Node> },
    // より大きい
    //Gt { lhs: Box<Node>, rhs: Box<Node> },
    // 以上
    //Ge { lhs: Box<Node>, rhs: Box<Node> },
    /// より小さい
    Lt { lhs: Box<Node>, rhs: Box<Node> },
    /// 以下
    Le { lhs: Box<Node>, rhs: Box<Node> },
    /// 数値
    Num(u32),
}

impl Node {
    /// 加算
    fn add(lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
        Box::new(Self::Add { lhs, rhs })
    }

    /// 減算
    fn sub(lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
        Box::new(Self::Sub { lhs, rhs })
    }

    /// 乗算
    fn mul(lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
        Box::new(Self::Mul { lhs, rhs })
    }

    /// 除算
    fn div(lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
        Box::new(Self::Div { lhs, rhs })
    }

    /// 等しい
    fn eq(lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
        Box::new(Self::Eq { lhs, rhs })
    }

    /// 等しくない
    fn ne(lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
        Box::new(Self::Ne { lhs, rhs })
    }

    // より大きい
    //fn gt(lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
    //    Box::new(Self::Gt { lhs, rhs })
    //}

    // 以上
    //fn ge(lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
    //    Box::new(Self::Ge { lhs, rhs })
    //}

    //// より小さい
    fn lt(lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
        Box::new(Self::Lt { lhs, rhs })
    }

    /// 以下
    fn le(lhs: Box<Node>, rhs: Box<Node>) -> Box<Self> {
        Box::new(Self::Le { lhs, rhs })
    }

    /// 数値
    fn num(value: u32) -> Box<Self> {
        Box::new(Self::Num(value))
    }
}
