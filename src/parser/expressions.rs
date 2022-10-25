use crate::lexer::{self, TokenKind};

/// List of tokens used internally.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Num(i64),
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
    Inc,
    Dec,
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
    BitXor(ExprKind, ExprKind),
    BitOr(ExprKind, ExprKind),
    BitAnd(ExprKind, ExprKind),
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

fn pack(tokens: Vec<Token>) -> Expression {
    // Idea:
    // "Num(1) Num(2) Add Num(3) Add" turns into
    // -> "Add(Add(Num(1), Num(2)), 3)"
    //
    // number => push to stack
    // unary operator => {
    //    1. take the last two items from the stack (if some)
    //    2. if they are literals, preform the operation on them and move on
    //    3. otherwise append to the stack
    // }
    // binary operator => {}
    let mut values = Vec::new();

    for token in tokens {
        println!("{:?}", token);

        match token {
            Token::Num(a) => values.push(Token::Num(a)),
            token if token.is_unary() => {
                println!("{:?} is unary", token);

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

                println!("asd asd: {:?}, {:?}", token, values);
            }
            token if token.is_binary() => println!("{:?} is binary", token),
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
    #[test]
    fn test_pack() {
        let i0 = lexer("1 + 2");
        let i1 = lexer("12 + 3");
        let i2 = lexer("(1 + 2) * 3");
        let i3 = lexer("abc + 4");

        let o0 = format!("{:?}", pack(convert(tokenize(i0))));
        let o1 = format!("{:?}", pack(convert(tokenize(i1))));
        let o2 = format!("{:?}", pack(convert(tokenize(i2))));
        let o3 = format!("{:?}", pack(convert(tokenize(i3))));

        let e0 = "Expression { expr: Other(Num(3)) }".to_string();
        let e1 = "Expression { expr: Other(Num(15)) }".to_string();
        let e2 = "Expression { expr: Other(Num(9)) }".to_string();
        let e3 = "Expression { expr: Add(Ident(\"abc\"), Num(4)) }".to_string();

        assert_eq!(o0, e0, "Test case 1 failed");
        assert_eq!(o1, e1, "Test case 2 failed");
        assert_eq!(o2, e2, "Test case 3 failed");
        assert_eq!(o3, e3, "Test case 4 failed");
    }
}
