#[derive(Debug)]
pub enum TokenKind {
    Reserved(char),
    Num(u32),
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
}

struct Input {
    source: Vec<char>,
    position: usize,
}

impl Input {
    fn new(source: Vec<char>) -> Self {
        Input {
            source,
            position: 0,
        }
    }

    fn is_space(&self) -> bool {
        self.source[self.position].eq(&' ')
    }

    fn is_digit(&self) -> bool {
        self.source[self.position].is_digit(10)
    }

    fn is_end(&self) -> bool {
        self.source.len() <= self.position
    }

    fn get(&self) -> Option<char> {
        self.source.get(self.position).map(|it| it.clone())
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn get_number_and_advance(&mut self) -> Option<u32> {
        if !self.is_digit() {
            None
        } else {
            let mut sum: u32 = 0;
            let mut len = 0;

            for c in self.source[self.position..].iter() {
                if let Some(v) = c.to_digit(10) {
                    sum = sum * 10 + v;
                    len += 1;
                } else {
                    break;
                }
            }

            self.position += len;
            Some(sum)
        }
    }
}

pub fn tokenize(source: Vec<char>) -> Vec<Token> {
    let mut input = Input::new(source);
    let mut token = vec![];

    while !input.is_end() {
        if input.is_space() {
            input.advance();
            continue;
        }

        let c = input
            .get()
            .expect("このメッセージが表示されていたらバグです");

        if c.eq(&'+') || c.eq(&'-') {
            input.advance();
            token.push(Token {
                kind: TokenKind::Reserved(c),
            });
            continue;
        }

        if input.is_digit() {
            let n = input
                .get_number_and_advance()
                .expect("このメッセージが表示されていたらバグです");
            token.push(Token {
                kind: TokenKind::Num(n),
            });
            continue;
        }

        panic!("トークナイズできません")
    }

    token
}
