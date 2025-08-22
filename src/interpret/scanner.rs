use log::debug;

use crate::interpret::lexer::Token;
#[derive(Debug)]
pub struct Scanner {
    tokens: Vec<Token>,
    curr: usize,
}
#[derive(Debug, Clone)]
pub enum Errros {
    NoLeftParan(String),
    NoRightParan(String),
}
impl Scanner {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, curr: 0 }
    }
    pub fn current_debug(&self) -> Option<&Token> {
        self.tokens.get(self.curr)
    }
    pub fn get_next_token(&mut self) -> Option<&Token> {
        self.curr += 1;
        let token = self.tokens.get(self.curr);
        return token;
    }
    pub fn consume_token(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.curr);
        self.curr += 1;
        return token;
    }
    pub fn peek1(&self) -> Option<&Token> {
        if let token = self.tokens.get(self.curr + 1) {
            return token;
        } else {
            return None;
        }
    }
    pub fn advance(&mut self) {
        self.curr += 1;
        debug!(
            "advanced, current token is {:?}",
            self.tokens.get(self.curr)
        );
    }
    pub fn peek2(&self) -> Option<&Token> {
        if let token = self.tokens.get(self.curr + 2) {
            return token;
        } else {
            return None;
        }
    }
}
