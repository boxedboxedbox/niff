use crate::lexer::{self, TokenKind};

/// List of tokens used internally.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Num(i64),
    Boolean(bool),
    Ident(String),
    Operator(OpKind),
    LeftParen,
    RightParen,
    Other(OpKind, Box<Token>, Box<Token>),
}

impl Token {
    fn is_number(&self) -> bool {
        matches!(self, Token::Num(_))
    }
    fn is_boolean(&self) -> bool {
        matches!(self, Token::Boolean(_))
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
            Self::Operator(OpKind::BitOr) => 3,
            Self::Operator(OpKind::BitXor) => 4,
            Self::Operator(OpKind::BitAnd) => 5,
            Self::Operator(OpKind::Equals) | Self::Operator(OpKind::NotEquals) => 6,
            Self::Operator(OpKind::Gt)
            | Self::Operator(OpKind::Lt)
            | Self::Operator(OpKind::GtEq)
            | Self::Operator(OpKind::LtEq) => 7,
            Self::Operator(OpKind::Sub) | Self::Operator(OpKind::Add) => 8,
            Self::Operator(OpKind::Div)
            | Self::Operator(OpKind::Mul)
            | Self::Operator(OpKind::Mod) => 10,
            Self::LeftParen | Self::RightParen => 13,
            _ => unimplemented!(),
        }
    }
    fn is_unary(&self) -> bool {
        if !self.is_operator() {
            return false;
        }

        matches!(
            self,
            Self::Operator(OpKind::Add)
                | Self::Operator(OpKind::Sub)
                | Self::Operator(OpKind::Mul)
                | Self::Operator(OpKind::Div)
                | Self::Operator(OpKind::Mod)
                | Self::Operator(OpKind::BitXor)
                | Self::Operator(OpKind::BitAnd)
                | Self::Operator(OpKind::BitOr)
                | Self::Operator(OpKind::Equals)
                | Self::Operator(OpKind::NotEquals)
                | Self::Operator(OpKind::GtEq)
                | Self::Operator(OpKind::LtEq)
                | Self::Operator(OpKind::Gt)
                | Self::Operator(OpKind::Lt)
        )
    }
    fn is_binary(&self) -> bool {
        if !self.is_operator() {
            return false;
        }

        matches!(
            self,
            Self::Operator(OpKind::Inc) | Self::Operator(OpKind::Dec)
        )
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
    Equals,
    NotEquals,
    GtEq,
    LtEq,
    Gt,
    Lt,
    Inc,
    Dec,
}

/// Is this a number, an identifier or another expression?
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprKind {
    Num(i64),
    Boolean(bool),
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
    BitXor(ExprKind, ExprKind),
    BitOr(ExprKind, ExprKind),
    BitAnd(ExprKind, ExprKind),
    Equals(ExprKind, ExprKind),
    NotEquals(ExprKind, ExprKind),
    GtEq(ExprKind, ExprKind),
    LtEq(ExprKind, ExprKind),
    Gt(ExprKind, ExprKind),
    Lt(ExprKind, ExprKind),
    Inc(ExprKind),
    Dec(ExprKind),
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
            lexer::TokenKind::DoubleEquals => res.push(Token::Operator(OpKind::Equals)),
            lexer::TokenKind::BangEquals => res.push(Token::Operator(OpKind::NotEquals)),
            lexer::TokenKind::GtEq => res.push(Token::Operator(OpKind::GtEq)),
            lexer::TokenKind::LtEq => res.push(Token::Operator(OpKind::LtEq)),
            lexer::TokenKind::GreaterThan => res.push(Token::Operator(OpKind::Gt)),
            lexer::TokenKind::LessThan => res.push(Token::Operator(OpKind::Lt)),
            lexer::TokenKind::LeftParen => res.push(Token::LeftParen),
            lexer::TokenKind::RightParen => res.push(Token::RightParen),
            token if *token == TokenKind::IntegerLiteral(0) => match token {
                TokenKind::IntegerLiteral(a) => res.push(Token::Num(*a)),
                _ => unreachable!(),
            },
            token if *token == TokenKind::True => res.push(Token::Boolean(true)),
            token if *token == TokenKind::False => res.push(Token::Boolean(false)),
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

fn pack(tokens: Vec<Token>) -> Expression {
    // TODO: make work with booleans
    let mut values = Vec::new();

    for token in tokens {
        match token {
            tok if tok.is_number() => values.push(tok),
            tok if tok.is_boolean() => values.push(tok),
            token if token.is_unary() => {
                if let (Some(val0), Some(val1)) = (values.get(0), values.get(1)) {
                    if let Token::Operator(ref op) = token {
                        let res = apply(val0, val1, op);

                        values.pop();
                        values.pop();

                        values.push(res);
                    }
                } else {
                    panic!("not enough values on the stack")
                }
            }
            token if token.is_binary() => {}
            token if token.is_identifier() => values.push(token),
            token => panic!("{:?}", token),
        }
    }

    Expression {
        expr: token_to_expr_token(&values[0]),
    }
}

fn token_to_expr_token(token: &Token) -> ExprToken {
    match token {
        Token::Num(a) => ExprToken::Other(ExprKind::Num(*a)),
        Token::Boolean(a) => ExprToken::Other(ExprKind::Boolean(*a)),
        Token::Other(op, l, r) => match &**l {
            // we know that r has to be an identifier,
            // otherwise this would have returned already.
            // (Both are numbers. If not, that's a bug in apply()).
            Token::Num(a) => {
                if let Token::Ident(b) = &**r {
                    fill(op, ExprKind::Num(*a), ExprKind::Ident(b.to_string()))
                } else {
                    panic!("not accepted {:?}", r)
                }
            }
            Token::Ident(a) => match &**r {
                Token::Ident(b) => fill(
                    op,
                    ExprKind::Ident(a.to_string()),
                    ExprKind::Ident(b.to_string()),
                ),
                Token::Num(b) => fill(op, ExprKind::Ident(a.to_string()), ExprKind::Num(*b)),
                _ => panic!("not accepted {:?}", r),
            },
            e => panic!("not accepted {:?}", e),
        },
        _ => panic!("not accepted {:?}", token),
    }
}

/// Create an ExprToken from an unary operator `op` filled with `a` and `b`.
fn fill(op: &OpKind, a: ExprKind, b: ExprKind) -> ExprToken {
    match *op {
        OpKind::Add => ExprToken::Add(a, b),
        OpKind::Sub => ExprToken::Sub(a, b),
        OpKind::Div => ExprToken::Div(a, b),
        OpKind::Mul => ExprToken::Mul(a, b),
        OpKind::Mod => ExprToken::Mod(a, b),
        OpKind::BitXor => ExprToken::BitXor(a, b),
        OpKind::BitOr => ExprToken::BitOr(a, b),
        OpKind::BitAnd => ExprToken::BitAnd(a, b),
        OpKind::Equals => ExprToken::Equals(a, b),
        OpKind::NotEquals => ExprToken::NotEquals(a, b),
        OpKind::GtEq => ExprToken::GtEq(a, b),
        OpKind::LtEq => ExprToken::LtEq(a, b),
        OpKind::Gt => ExprToken::Gt(a, b),
        OpKind::Lt => ExprToken::Lt(a, b),
        _ => panic!("not accepted {:?}", op),
    }
}

/// Create an ExprToken from an binnary operator `op` filled with `a`.
fn fill_binary(op: &OpKind, a: ExprKind) -> ExprToken {
    match *op {
        OpKind::Inc => ExprToken::Inc(a),
        OpKind::Dec => ExprToken::Dec(a),
        _ => panic!("not accepted {:?}", op),
    }
}

fn apply(l: &Token, r: &Token, op: &OpKind) -> Token {
    // TODO: get rid of clones
    match l {
        Token::Ident(ref a) => match r {
            Token::Num(ref b) => Token::Other(
                op.clone(),
                box Token::Ident(a.to_string()),
                box Token::Num(*b),
            ),
            Token::Ident(ref b) => Token::Other(
                op.clone(),
                box Token::Ident(a.to_string()),
                box Token::Ident(b.to_string()),
            ),
            _ => panic!("not accepted {:?}", r),
        },
        Token::Num(a) => match r {
            Token::Num(b) => match op {
                OpKind::Add => Token::Num(a + b),
                OpKind::Sub => Token::Num(a - b),
                OpKind::Mul => Token::Num(a * b),
                OpKind::Div => Token::Num(a / b),
                OpKind::Mod => Token::Num(a % b),
                OpKind::BitXor => Token::Num(a ^ b),
                OpKind::BitOr => Token::Num(a | b),
                OpKind::BitAnd => Token::Num(a & b),
                OpKind::Equals => Token::Boolean(a == b),
                OpKind::NotEquals => Token::Boolean(a != b),
                OpKind::GtEq => Token::Boolean(a >= b),
                OpKind::LtEq => Token::Boolean(a <= b),
                OpKind::Gt => Token::Boolean(a > b),
                OpKind::Lt => Token::Boolean(a < b),
                _ => panic!("not accepted {:?}", op),
            },
            Token::Ident(b) => Token::Other(
                op.clone(),
                box Token::Num(*a),
                box Token::Ident(b.to_string()),
            ),
            _ => panic!("not accepted {:?}", r),
        },
        _ => panic!("not accepted {:?}", l),
    }
}

pub fn parse_expression(expr: Vec<lexer::Token>) -> Expression {
    let tokens = tokenize(expr);
    let postfix = convert(tokens);
    pack(postfix)
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
        let i4 = lexer("1 == 2");

        let t0 = format!("{:?}", tokenize(i0));
        let t1 = format!("{:?}", tokenize(i1));
        let t2 = format!("{:?}", tokenize(i2));
        let t3 = format!("{:?}", tokenize(i3));
        let t4 = format!("{:?}", tokenize(i4));

        let e0 = "[Num(1), Operator(Add), Num(2)]".to_string();
        let e1 = "[Num(12), Operator(Div), Num(3)]".to_string();
        let e2 = "[LeftParen, Num(1), Operator(Add), Num(2), RightParen, Operator(Mul), Num(3)]"
            .to_string();
        let e3 = "[Ident(\"abc\"), Operator(Div), Num(3)]".to_string();
        let e4 = "[Num(1), Operator(Equals), Num(2)]".to_string();

        assert_eq!(t0, e0, "Test case 1 failed");
        assert_eq!(t1, e1, "Test case 2 failed");
        assert_eq!(t2, e2, "Test case 3 failed");
        assert_eq!(t3, e3, "Test case 4 failed");
        assert_eq!(t4, e4, "Test case 5 failed");
    }
    #[test]
    fn test_convert() {
        let i0 = lexer("1 + 2");
        let i1 = lexer("12 / 3");
        let i2 = lexer("(1 + 2) * 3");
        let i3 = lexer("abc / 3");
        let i4 = lexer("1 == 2");

        let o0 = format!("{:?}", convert(tokenize(i0)));
        let o1 = format!("{:?}", convert(tokenize(i1)));
        let o2 = format!("{:?}", convert(tokenize(i2)));
        let o3 = format!("{:?}", convert(tokenize(i3)));
        let o4 = format!("{:?}", convert(tokenize(i4)));

        let e0 = "[Num(1), Num(2), Operator(Add)]".to_string();
        let e1 = "[Num(12), Num(3), Operator(Div)]".to_string();
        let e2 = "[Num(1), Num(2), Operator(Add), Num(3), Operator(Mul)]".to_string();
        let e3 = "[Ident(\"abc\"), Num(3), Operator(Div)]".to_string();
        let e4 = "[Num(1), Num(2), Operator(Equals)]".to_string();

        assert_eq!(o0, e0, "Test case 1 failed");
        assert_eq!(o1, e1, "Test case 2 failed");
        assert_eq!(o2, e2, "Test case 3 failed");
        assert_eq!(o3, e3, "Test case 4 failed");
        assert_eq!(o4, e4, "Test case 5 failed");
    }
    #[test]
    fn test_pack() {
        let i0 = lexer("1 + 2");
        let i1 = lexer("12 + 3");
        let i2 = lexer("(1 + 2) * 3");
        let i3 = lexer("abc + 4");
        let i4 = lexer("4 + abc");
        let i5 = lexer("1 == 2");
        let i6 = lexer("abc != 3");

        let o0 = format!("{:?}", pack(convert(tokenize(i0))));
        let o1 = format!("{:?}", pack(convert(tokenize(i1))));
        let o2 = format!("{:?}", pack(convert(tokenize(i2))));
        let o3 = format!("{:?}", pack(convert(tokenize(i3))));
        let o4 = format!("{:?}", pack(convert(tokenize(i4))));
        let o5 = format!("{:?}", pack(convert(tokenize(i5))));
        let o6 = format!("{:?}", pack(convert(tokenize(i6))));

        let e0 = "Expression { expr: Other(Num(3)) }".to_string();
        let e1 = "Expression { expr: Other(Num(15)) }".to_string();
        let e2 = "Expression { expr: Other(Num(9)) }".to_string();
        let e3 = "Expression { expr: Add(Ident(\"abc\"), Num(4)) }".to_string();
        let e4 = "Expression { expr: Add(Num(4), Ident(\"abc\")) }".to_string();
        let e5 = "Expression { expr: Other(Boolean(false)) }".to_string();
        let e6 = "Expression { expr: NotEquals(Ident(\"abc\"), Num(3)) }".to_string();

        assert_eq!(o0, e0, "Test case 1 failed");
        assert_eq!(o1, e1, "Test case 2 failed");
        assert_eq!(o2, e2, "Test case 3 failed");
        assert_eq!(o3, e3, "Test case 4 failed");
        assert_eq!(o4, e4, "Test case 5 failed");
        assert_eq!(o5, e5, "Test case 6 failed");
        assert_eq!(o6, e6, "Test case 7 failed");
    }
}
