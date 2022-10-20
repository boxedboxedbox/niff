use super::ParsedToken;
use crate::lexer::{self, TokenKind};

/// List of tokens used internally.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Num(i64),
    Ident(String),
    Operator(OpKind),
    LeftParen,
    RightParen,
}

impl Token {
    fn is_number(&self) -> bool {
        matches!(self, Token::Num(_))
    }
    fn is_identifier(&self) -> bool {
        matches!(self, Token::Ident(_))
    }
    fn is_operator(&self) -> bool {
        matches!(self, Token::Operator(_))
    }
    fn precedence(&self) -> u8 {
        // Note to self: Unary operators should have the highest priority
        match self {
            Self::Operator(OpKind::Sub) | Self::Operator(OpKind::Add) => 8,
            Self::Operator(OpKind::Div)
            | Self::Operator(OpKind::Mul)
            | Self::Operator(OpKind::Mod) => 10,
            Self::LeftParen | Self::RightParen => 13,
            _ => unimplemented!(),
        }
    }
}

/// List of operator kinds used internally.
#[derive(Debug, Clone, PartialEq, Eq)]
enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitXor,
    BitOr,
}

/// Is this a number, an identifier or another expression?
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind {
    Num(i64),
    Ident(String),
    Expr(Box<ExprToken>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprToken {
    Add(ExprKind, ExprKind),
    Sub(ExprKind, ExprKind),
    Mul(ExprKind, ExprKind),
    Div(ExprKind, ExprKind),
    Mod(ExprKind, ExprKind),
    LeftParen,
    RightParen,
    Other(ExprKind),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    expr: ExprToken,
}

fn tokenize(input: Vec<lexer::Token>) -> Vec<Token> {
    let mut res = Vec::new();
    let mut i = 0;

    while i < input.len() {
        match &input[i].kind {
            lexer::TokenKind::Plus => res.push(Token::Operator(OpKind::Add)),
            lexer::TokenKind::Minus => res.push(Token::Operator(OpKind::Sub)),
            lexer::TokenKind::Slash => res.push(Token::Operator(OpKind::Div)),
            lexer::TokenKind::Star => res.push(Token::Operator(OpKind::Mul)),
            lexer::TokenKind::Percent => res.push(Token::Operator(OpKind::Mod)),
            lexer::TokenKind::Caret => res.push(Token::Operator(OpKind::BitXor)),
            lexer::TokenKind::And => res.push(Token::Operator(OpKind::BitAnd)),
            lexer::TokenKind::Pipe => res.push(Token::Operator(OpKind::BitOr)),
            lexer::TokenKind::LeftParen => res.push(Token::LeftParen),
            lexer::TokenKind::RightParen => res.push(Token::RightParen),
            token if *token == TokenKind::IntegerLiteral(0) => match token {
                TokenKind::IntegerLiteral(a) => res.push(Token::Num(*a)),
                _ => unreachable!(),
            },
            token if *token == TokenKind::Unknown("".to_string()) => match token {
                TokenKind::Unknown(a) => res.push(Token::Ident(a.to_string())),
                _ => unreachable!(),
            },
            token if *token == TokenKind::Eof => break,
            _token => panic!("invalid token: {:?}", _token),
        }

        i += 1;
    }

    res
}

/// Turn infix syntax into postfix syntax using shunting yard algorithm.
fn convert(input: Vec<Token>) -> Vec<Token> {
    let mut operator_stack: Vec<Token> = Vec::new();
    let mut output_queue = Vec::new();

    for i in input {
        match i {
            token if token.is_number() || token.is_identifier() => output_queue.push(token),
            token if token.is_operator() => {
                loop {
                    if operator_stack.is_empty() {
                        break;
                    }

                    // TODO: remove clone if possible
                    let op = operator_stack[0].clone();

                    if op.precedence() > token.precedence() {
                        break;
                    }

                    if op == Token::LeftParen {
                        break;
                    }

                    operator_stack.pop();
                    output_queue.push(op);
                }

                operator_stack.push(token);
            }
            Token::LeftParen => operator_stack.push(Token::LeftParen),
            Token::RightParen => {
                while *operator_stack
                    .last()
                    .expect("somehow our queue is too small")
                    != Token::LeftParen
                    && !operator_stack.is_empty()
                {
                    let op = operator_stack.pop().unwrap();
                    output_queue.push(op);
                }

                operator_stack.pop();
            }
            _ => panic!("huh? Dump: {:?}", i),
        }
    }

    while !operator_stack.is_empty() {
        let op = operator_stack.pop().unwrap();
        output_queue.push(op);
    }

    output_queue
}

fn finalize(tokens: Vec<Token>) -> Expression {
    let res = 0;

    // Idea:
    // "Num(1) Num(2) Add Num(3) Add" turns into
    // -> "Add(Add(Num(1), Num(2)), 3)"

    Expression {
        expr: ExprToken::None,
    }
}

pub fn parse_expression(expr: Vec<lexer::Token>) -> Expression {
    let tokens = tokenize(expr);
    let postfix = convert(tokens);
    let result = finalize(postfix);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lexer;

    #[test]
    fn test_tokenize() {
        let i0 = lexer("1 + 2");
        let i1 = lexer("12 / 3");
        let i2 = lexer("(1 + 2) * 3");
        let i3 = lexer("abc / 3");

        let t0 = format!("{:?}", tokenize(i0));
        let t1 = format!("{:?}", tokenize(i1));
        let t2 = format!("{:?}", tokenize(i2));
        let t3 = format!("{:?}", tokenize(i3));

        let e0 = "[Num(1), Operator(Add), Num(2)]".to_string();
        let e1 = "[Num(12), Operator(Div), Num(3)]".to_string();
        let e2 = "[LeftParen, Num(1), Operator(Add), Num(2), RightParen, Operator(Mul), Num(3)]"
            .to_string();
        let e3 = "[Ident(\"abc\"), Operator(Div), Num(3)]".to_string();

        assert_eq!(t0, e0, "Test case 1 failed");
        assert_eq!(t1, e1, "Test case 2 failed");
        assert_eq!(t2, e2, "Test case 3 failed");
        assert_eq!(t3, e3, "Test case 4 failed");
    }
    #[test]
    fn test_convert() {
        let i0 = lexer("1 + 2");
        let i1 = lexer("12 / 3");
        let i2 = lexer("(1 + 2) * 3");
        let i3 = lexer("abc / 3");

        let o0 = format!("{:?}", convert(tokenize(i0)));
        let o1 = format!("{:?}", convert(tokenize(i1)));
        let o2 = format!("{:?}", convert(tokenize(i2)));
        let o3 = format!("{:?}", convert(tokenize(i3)));

        let e0 = "[Num(1), Num(2), Operator(Add)]".to_string();
        let e1 = "[Num(12), Num(3), Operator(Div)]".to_string();
        let e2 = "[Num(1), Num(2), Operator(Add), Num(3), Operator(Mul)]".to_string();
        let e3 = "[Ident(\"abc\"), Num(3), Operator(Div)]".to_string();

        assert_eq!(o0, e0, "Test case 1 failed");
        assert_eq!(o1, e1, "Test case 2 failed");
        assert_eq!(o2, e2, "Test case 3 failed");
        assert_eq!(o3, e3, "Test case 4 failed");
    }
}
