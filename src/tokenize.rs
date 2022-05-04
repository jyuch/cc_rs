use crate::Token;

#[derive(Debug, Clone)]
struct Input<'a> {
    source: &'a Vec<char>,
    position: usize,
}

impl<'a> Input<'a> {
    fn new(source: &'a Vec<char>) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    fn is_space(&self) -> bool {
        match self.source.get(self.position) {
            Some(c) => c.eq(&' '),
            None => false,
        }
    }

    fn is_digit(&self, radix: u32) -> bool {
        match self.source.get(self.position) {
            Some(c) => c.is_digit(radix),
            None => false,
        }
    }

    fn is_end(&self) -> bool {
        self.source.len() <= self.position
    }

    fn get(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }

    fn advance(&self) -> Self {
        Self {
            source: self.source,
            position: self.position + 1,
        }
    }
}

#[derive(Debug)]
struct Tokenizer<'a> {
    input: Input<'a>,
}

impl<'a> Tokenizer<'a> {
    fn new(source: &'a Vec<char>) -> Self {
        Self {
            input: Input::new(source),
        }
    }

    fn is_space(&self) -> bool {
        self.input.is_space()
    }

    fn is_end(&self) -> bool {
        self.input.is_end()
    }

    fn advance(&mut self) {
        self.input = self.input.advance()
    }

    fn try_consume(&mut self, reserved: &str) -> bool {
        let mut source = self.input.clone();

        for it in reserved.chars() {
            match source.get() {
                Some(c) if it == c => {
                    source = source.advance();
                }
                _ => return false,
            }
        }

        self.input = source;
        true
    }

    fn try_consume_number(&mut self, radix: u32) -> Option<u32> {
        if !self.input.is_digit(radix) {
            None
        } else {
            let mut sum: u32 = 0;

            loop {
                if self.input.is_digit(radix) {
                    let c = self
                        .input
                        .get()
                        .expect("このメッセージが表示されていたらバグです")
                        .to_digit(radix)
                        .expect("このメッセージが表示されていたらバグです");
                    sum = sum * radix + c;
                    self.input = self.input.advance();
                } else {
                    return Some(sum);
                }
            }
        }
    }
}

pub fn tokenize(source: Vec<char>) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new(&source);
    let mut token = vec![];

    while !tokenizer.is_end() {
        if tokenizer.is_space() {
            tokenizer.advance();
            continue;
        }

        if tokenizer.try_consume("+") {
            token.push(Token::reserved("+".to_string()));
            continue;
        }

        if tokenizer.try_consume("-") {
            token.push(Token::reserved("-".to_string()));
            continue;
        }

        if tokenizer.try_consume("*") {
            token.push(Token::reserved("*".to_string()));
            continue;
        }

        if tokenizer.try_consume("/") {
            token.push(Token::reserved("/".to_string()));
            continue;
        }

        if tokenizer.try_consume("(") {
            token.push(Token::reserved("(".to_string()));
            continue;
        }

        if tokenizer.try_consume(")") {
            token.push(Token::reserved(")".to_string()));
            continue;
        }

        if let Some(value) = tokenizer.try_consume_number(10) {
            token.push(Token::num(value));
        }
    }

    token
}

#[cfg(test)]
mod tokenizer_tests {
    use super::*;

    #[test]
    fn try_consume() {
        let source = "abc123".chars().collect();
        let mut tokenizer = Tokenizer::new(&source);

        assert!(tokenizer.try_consume("abc"));
        assert!(tokenizer.try_consume("123"));
    }

    #[test]
    fn try_consume_number() {
        let source = "123".chars().collect();
        let mut tokenizer = Tokenizer::new(&source);

        assert_eq!(Some(123), tokenizer.try_consume_number(10));
    }
}
