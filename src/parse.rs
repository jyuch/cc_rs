use crate::{Node, Token, TokenKind};

struct Parser {
    token: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(token: Vec<Token>) -> Self {
        Self { token, position: 0 }
    }

    fn expr(&mut self) -> Box<Node> {
        let mut node = self.mul();

        loop {
            if self.consume('+') {
                node = Node::add(node, self.mul());
            } else if self.consume('-') {
                node = Node::sub(node, self.mul());
            } else {
                return node;
            }
        }
    }

    fn mul(&mut self) -> Box<Node> {
        let mut node = self.primary();

        loop {
            if self.consume('*') {
                node = Node::mul(node, self.primary());
            } else if self.consume('/') {
                node = Node::div(node, self.primary());
            } else {
                return node;
            }
        }
    }

    fn primary(&mut self) -> Box<Node> {
        if self.consume('(') {
            let node = self.expr();
            self.expect(')');
            node
        } else {
            Node::num(self.expect_number())
        }
    }

    fn consume(&mut self, op: char) -> bool {
        let t = self.token.get(self.position);

        match t {
            Some(t) => match t.kind {
                TokenKind::Reserved(c) if op.eq(&c) => {
                    self.position += 1;
                    true
                }
                _ => false,
            },
            None => false,
        }
    }

    fn expect(&mut self, op: char) {
        let t = self.token.get(self.position);

        match t {
            Some(t) => match t.kind {
                TokenKind::Reserved(c) if op.eq(&c) => {
                    self.position += 1;
                }
                _ => panic!("{}ではありません。", op),
            },
            None => panic!("{}ではありません。", op),
        }
    }

    fn expect_number(&mut self) -> u32 {
        let t = self.token.get(self.position);

        match t {
            Some(t) => match t.kind {
                TokenKind::Num(i) => {
                    self.position += 1;
                    i
                }
                _ => panic!("数ではありません"),
            },
            None => panic!("数ではありません"),
        }
    }
}

pub fn parse(token: Vec<Token>) -> Box<Node> {
    let mut parser = Parser::new(token);
    parser.expr()
}
