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
        self.equality()
    }

    fn equality(&mut self) -> Box<Node> {
        let mut node = self.relational();

        loop {
            if self.consume("==") {
                node = Node::eq(node, self.relational());
            } else if self.consume("!=") {
                node = Node::ne(node, self.relational());
            } else {
                return node;
            }
        }
    }

    fn relational(&mut self) -> Box<Node> {
        let mut node = self.add();

        loop {
            if self.consume("<") {
                node = Node::lt(node, self.add());
            } else if self.consume("<=") {
                node = Node::le(node, self.add());
            } else if self.consume(">") {
                node = Node::lt(self.add(), node);
            } else if self.consume(">=") {
                node = Node::le(self.add(), node);
            } else {
                return node;
            }
        }
    }

    fn add(&mut self) -> Box<Node> {
        let mut node = self.mul();

        loop {
            if self.consume("+") {
                node = Node::add(node, self.mul());
            } else if self.consume("-") {
                node = Node::sub(node, self.mul());
            } else {
                return node;
            }
        }
    }

    fn mul(&mut self) -> Box<Node> {
        let mut node = self.unary();

        loop {
            if self.consume("*") {
                node = Node::mul(node, self.unary());
            } else if self.consume("/") {
                node = Node::div(node, self.unary());
            } else {
                return node;
            }
        }
    }

    fn unary(&mut self) -> Box<Node> {
        if self.consume("+") {
            self.primary()
        } else if self.consume("-") {
            Node::sub(Node::num(0), self.primary())
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Box<Node> {
        if self.consume("(") {
            let node = self.expr();
            self.expect(")");
            node
        } else {
            Node::num(self.expect_number())
        }
    }

    fn consume(&mut self, op: &str) -> bool {
        let t = self.token.get(self.position);

        match t {
            Some(t) => match &t.kind {
                TokenKind::Reserved(c) if op.eq(c) => {
                    self.position += 1;
                    true
                }
                _ => false,
            },
            None => false,
        }
    }

    fn expect(&mut self, op: &str) {
        let t = self.token.get(self.position);

        match t {
            Some(t) => match &t.kind {
                TokenKind::Reserved(c) if op.eq(c) => {
                    self.position += 1;
                }
                _ => panic!("{}????????????????????????", op),
            },
            None => panic!("{}????????????????????????", op),
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
                _ => panic!("????????????????????????"),
            },
            None => panic!("????????????????????????"),
        }
    }
}

pub fn parse(token: Vec<Token>) -> Box<Node> {
    let mut parser = Parser::new(token);
    parser.expr()
}
