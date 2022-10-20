use super::*;

/// A range where there is either an identifier or a syntax error.
pub fn lex_unknown_or_ident(l: &mut Lexer) {
    let mut res = String::new();
    let start = l.i;
    let mut end = l.i;

    while l.len > l.i {
        match l.src[l.i] {
            'a'..='z' => {
                end += 1;
                res.push(l.src[l.i])
            }
            'A'..='Z' => {
                end += 1;
                res.push(l.src[l.i])
            }
            '0'..='9' => {
                end += 1;
                res.push(l.src[l.i])
            }
            '_' => {
                end += 1;
                res.push(l.src[l.i])
            }
            _ => break,
        }

        l.advance();
    }

    if start == end {
        return;
    }

    l.add_token(Token {
        kind: TokenKind::Unknown(res),
        size: (end - start),
    });
}

pub fn lex_type(l: &mut Lexer) {
    // void
    if l.src.get(l.i) == Some('v').as_ref()
        && l.src.get(l.i + 1) == Some('o').as_ref()
        && l.src.get(l.i + 2) == Some('i').as_ref()
        && l.src.get(l.i + 3) == Some('d').as_ref()
    {
        l.advance_with(4);

        l.add_token(Token {
            kind: TokenKind::Type(Type::Void),
            size: 4,
        });

        return;
    }

    // i32
    if l.src.get(l.i) == Some('i').as_ref()
        && l.src.get(l.i + 1) == Some('3').as_ref()
        && l.src.get(l.i + 2) == Some('2').as_ref()
    {
        l.advance_with(3);

        l.add_token(Token {
            kind: TokenKind::Type(Type::I32),
            size: 3,
        });

        return;
    }

    // u32
    if l.src.get(l.i) == Some('u').as_ref()
        && l.src.get(l.i + 1) == Some('3').as_ref()
        && l.src.get(l.i + 2) == Some('2').as_ref()
    {
        l.advance_with(3);

        l.add_token(Token {
            kind: TokenKind::Type(Type::U32),
            size: 3,
        });
    }
}

/// Turns a number into tokens. Doesn't support floating point numbers.
pub fn lex_number(l: &mut Lexer) {
    let mut res = String::new();
    let start = l.i;
    let mut end = l.i;

    while l.len > l.i {
        match l.src[l.i] {
            '0'..='9' => {
                end += 1;
                res.push(l.src[l.i])
            }
            _ => break,
        }

        l.advance();
    }

    if res.is_empty() {
        return;
    }

    l.add_token(Token {
        kind: TokenKind::IntegerLiteral(res.parse().expect("idk")),
        size: end - start,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_void() {
        let src = " void2";
        let mut lexer = Lexer::new(src);

        lexer.advance();

        lex_type(&mut lexer);

        let a = format!("{:?}", lexer.tokens.last().unwrap());
        let b = format!(
            "{:?}",
            Token {
                kind: TokenKind::Type(Type::Void),
                size: 4
            }
        );

        assert_eq!(a, b);
        assert_eq!(lexer.src[lexer.i], '2');
    }

    #[test]
    fn test_type_i32() {
        let src = " i320";
        let mut lexer = Lexer::new(src);

        lexer.advance();

        lex_type(&mut lexer);

        let a = format!("{:?}", lexer.tokens.last().unwrap());
        let b = format!(
            "{:?}",
            Token {
                kind: TokenKind::Type(Type::I32),
                size: 3
            }
        );

        assert_eq!(a, b);
        assert_eq!(lexer.src[lexer.i], '0');
    }

    #[test]
    fn test_type_u32() {
        let src = " u320";
        let mut lexer = Lexer::new(src);

        lexer.advance();

        lex_type(&mut lexer);

        let a = format!("{:?}", lexer.tokens.last().unwrap());
        let b = format!(
            "{:?}",
            Token {
                kind: TokenKind::Type(Type::U32),
                size: 3
            }
        );

        assert_eq!(a, b);
        assert_eq!(lexer.src[lexer.i], '0');
    }
}
