mod conditionals;
mod expressions;
mod functions;
mod loops;

use super::lexer::{Lexer, Token, TokenKind};
use conditionals::*;
use expressions::*;
use functions::*;

#[derive(Debug, Clone)]
pub enum Type {
    Void,
    I32,
    U32,
    U8,
    String,
    /// No type specified. '_' or simply not specified.
    None,
}

#[derive(Debug)]
pub enum Modifiers {
    Mutable,
    Constant,
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    i: usize,
    len: usize,
    output: Vec<ParsedToken>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            len: tokens.len(),
            tokens,
            i: 0,
            output: Vec::new(),
        }
    }
    /// Advances the "cursor" by one.
    fn advance(&mut self) {
        self.i += 1;
    }
    /// Advances the "cursor" by `val`.
    /// Use advance() when only advancing by one.
    ///
    /// # Panics
    ///
    /// Panics on debug mode if `val` is 0 and recommends you to use advance.
    fn advance_with(&mut self, val: usize) {
        debug_assert_ne!(
            val, 0,
            "Instead of using advance_with(), you should use advance() instead, when advancing only by one."
        );

        self.i += val;
    }
    /// Adds `token` to the `tokens` list on the object.
    fn add_token(&mut self, token: ParsedToken) {
        self.output.push(token)
    }
}

#[derive(Debug)]
pub enum ParsedToken {
    /// Represents a function call. (function to be called, parameters)
    FunctionCall(String, CallParameters),
    /// Represents a function definition. (name, parameters, return type)
    Function(String, Parameters, Type),
    /// An expression.
    Expression(Expression),
    /// An if statement. (expression)
    If(Expression),
    /// End of file.
    Eof,
}

pub fn parser(tokens: Vec<Token>) -> Vec<ParsedToken> {
    let mut parser = Parser::new(tokens);

    functions::parse_function(&mut parser);
    functions::parse_function_call(&mut parser);

    vec![]
}
