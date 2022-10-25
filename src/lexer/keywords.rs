use super::*;

// TODO: Make this function more organized. (Split into multiple functions/macros or something.)
/// Checks if there is any keywords.
/// Returns false if not, true if one was found.
///
/// If you're wondering where the token went, it was added to the tokens list in the Lexer struct.
pub fn lex_keyword(l: &mut Lexer) -> bool {
    // "fn" + space
    if l.src.get(l.i) == Some('f').as_ref()
        && l.src.get(l.i + 1) == Some('n').as_ref()
        && l.src.get(l.i + 2) == Some(' ').as_ref()
    {
        l.advance_with(3);

        l.add_token(Token {
            kind: TokenKind::Fn,
            size: 3,
        });

        return true;
    }

    // "if"
    if l.src.get(l.i) == Some('i').as_ref() && l.src.get(l.i + 1) == Some('f').as_ref() {
        l.advance_with(2);

        l.add_token(Token {
            kind: TokenKind::If,
            size: 2,
        });

        return true;
    }

    // "elif"
    if l.src.get(l.i) == Some('e').as_ref()
        && l.src.get(l.i + 1) == Some('l').as_ref()
        && l.src.get(l.i + 2) == Some('i').as_ref()
        && l.src.get(l.i + 3) == Some('f').as_ref()
    {
        l.advance_with(4);

        l.add_token(Token {
            kind: TokenKind::Elif,
            size: 4,
        });

        return true;
    }

    // "else"
    if l.src.get(l.i) == Some('e').as_ref()
        && l.src.get(l.i + 1) == Some('l').as_ref()
        && l.src.get(l.i + 2) == Some('s').as_ref()
        && l.src.get(l.i + 3) == Some('e').as_ref()
    {
        l.advance_with(4);

        l.add_token(Token {
            kind: TokenKind::Else,
            size: 4,
        });

        return true;
    }

    // "for"
    if l.src.get(l.i) == Some('f').as_ref()
        && l.src.get(l.i + 1) == Some('o').as_ref()
        && l.src.get(l.i + 2) == Some('r').as_ref()
    {
        l.advance_with(3);

        l.add_token(Token {
            kind: TokenKind::For,
            size: 3,
        });

        return true;
    }

    // "while"
    if l.src.get(l.i) == Some('w').as_ref()
        && l.src.get(l.i + 1) == Some('h').as_ref()
        && l.src.get(l.i + 2) == Some('i').as_ref()
        && l.src.get(l.i + 3) == Some('l').as_ref()
        && l.src.get(l.i + 4) == Some('e').as_ref()
    {
        l.advance_with(5);

        l.add_token(Token {
            kind: TokenKind::While,
            size: 5,
        });

        return true;
    }

    // "return"
    if l.src.get(l.i) == Some('r').as_ref()
        && l.src.get(l.i + 1) == Some('e').as_ref()
        && l.src.get(l.i + 2) == Some('t').as_ref()
        && l.src.get(l.i + 3) == Some('u').as_ref()
        && l.src.get(l.i + 4) == Some('r').as_ref()
        && l.src.get(l.i + 5) == Some('n').as_ref()
    {
        l.advance_with(6);

        l.add_token(Token {
            kind: TokenKind::Return,
            size: 6,
        });

        return true;
    }

    // "var"
    if l.src.get(l.i) == Some('v').as_ref()
        && l.src.get(l.i + 1) == Some('a').as_ref()
        && l.src.get(l.i + 2) == Some('r').as_ref()
    {
        l.advance_with(4);

        l.add_token(Token {
            kind: TokenKind::Var,
            size: 4,
        });

        return true;
    }

    // "false"
    // TODO: if you have an identifier "falseasd", it will result in tokens: False and Unknown("asd"),
    // instead of the intended: Unknown("falseasd")
    if l.src.get(l.i) == Some('f').as_ref()
        && l.src.get(l.i + 1) == Some('a').as_ref()
        && l.src.get(l.i + 2) == Some('l').as_ref()
        && l.src.get(l.i + 3) == Some('s').as_ref()
        && l.src.get(l.i + 4) == Some('e').as_ref()
    {
        l.advance_with(5);

        l.add_token(Token {
            kind: TokenKind::False,
            size: 5,
        });

        return true;
    }

    // "true"
    // TODO: if you have an identifier "trueasd", it will result in tokens: True and Unknown("asd"),
    // instead of the intended: Unknown("trueasd")
    if l.src.get(l.i) == Some('t').as_ref()
        && l.src.get(l.i + 1) == Some('r').as_ref()
        && l.src.get(l.i + 2) == Some('u').as_ref()
        && l.src.get(l.i + 3) == Some('e').as_ref()
    {
        l.advance_with(4);

        l.add_token(Token {
            kind: TokenKind::True,
            size: 4,
        });

        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::super::Lexer;
    use super::*;

    #[test]
    fn test_lex_keyword_fn() {
        let src = "fn main() {}";
        let mut lexer = Lexer::new(src);

        lex_keyword(&mut lexer);

        let a = format!("{:?}", lexer.tokens.last().unwrap());
        let b = format!(
            "{:?}",
            Token {
                kind: TokenKind::Fn,
                size: 3
            }
        );

        assert_eq!(a, b);
        assert_eq!(lexer.src[lexer.i], 'm');
    }
    #[test]
    fn test_lex_keyword_if() {
        let src = "if(a == b) {}";
        let mut lexer = Lexer::new(src);

        lex_keyword(&mut lexer);

        let a = format!("{:?}", lexer.tokens.last().unwrap());
        let b = format!(
            "{:?}",
            Token {
                kind: TokenKind::If,
                size: 2
            }
        );

        assert_eq!(a, b);
        assert_eq!(lexer.src[lexer.i], '(');
    }
    #[test]
    fn test_lex_keyword_elif() {
        let src = "elif(a == b) {}";
        let mut lexer = Lexer::new(src);

        lex_keyword(&mut lexer);

        let a = format!("{:?}", lexer.tokens.last().unwrap());
        let b = format!(
            "{:?}",
            Token {
                kind: TokenKind::Elif,
                size: 4
            }
        );

        assert_eq!(a, b);
        assert_eq!(lexer.src[lexer.i], '(');
    }
    #[test]
    fn test_lex_keyword_else() {
        let src = "else {}";
        let mut lexer = Lexer::new(src);

        lex_keyword(&mut lexer);

        let a = format!("{:?}", lexer.tokens.last().unwrap());
        let b = format!(
            "{:?}",
            Token {
                kind: TokenKind::Else,
                size: 4
            }
        );

        assert_eq!(a, b);
        assert_eq!(lexer.src[lexer.i], ' ');
    }
    #[test]
    fn test_lex_keyword_for() {
        let src = "for(a = 1; b < 2; a++) {}";
        let mut lexer = Lexer::new(src);

        lex_keyword(&mut lexer);

        let a = format!("{:?}", lexer.tokens.last().unwrap());
        let b = format!(
            "{:?}",
            Token {
                kind: TokenKind::For,
                size: 3
            }
        );

        assert_eq!(a, b);
        assert_eq!(lexer.src[lexer.i], '(');
    }
    #[test]
    fn test_lex_keyword_while() {
        let src = "while(a == b) {}";
        let mut lexer = Lexer::new(src);

        lex_keyword(&mut lexer);

        let a = format!("{:?}", lexer.tokens.last().unwrap());
        let b = format!(
            "{:?}",
            Token {
                kind: TokenKind::While,
                size: 5
            }
        );

        assert_eq!(a, b);
        assert_eq!(lexer.src[lexer.i], '(');
    }
    #[test]
    fn test_lex_keyword_return() {
        let src = "return a;";
        let mut lexer = Lexer::new(src);

        lex_keyword(&mut lexer);

        let a = format!("{:?}", lexer.tokens.last().unwrap());
        let b = format!(
            "{:?}",
            Token {
                kind: TokenKind::Return,
                size: 6
            }
        );

        assert_eq!(a, b);
        assert_eq!(lexer.src[lexer.i], ' ');
    }
    #[test]
    fn test_lex_keyword_var() {
        let src = "var something = 3;";
        let mut lexer = Lexer::new(src);

        lex_keyword(&mut lexer);

        let a = format!("{:?}", lexer.tokens.last().unwrap());
        let b = format!(
            "{:?}",
            Token {
                kind: TokenKind::Var,
                size: 4
            }
        );

        assert_eq!(a, b);
        assert_eq!(lexer.src[lexer.i], 's');
    }
    #[test]
    fn test_lex_keyword_false() {
        let src = "false";
        let mut lexer = Lexer::new(src);

        lex_keyword(&mut lexer);

        let a = format!("{:?}", lexer.tokens.last().unwrap());
        let b = format!(
            "{:?}",
            Token {
                kind: TokenKind::False,
                size: 5
            }
        );

        assert_eq!(a, b);
    }
    #[test]
    fn test_lex_keyword_true() {
        let src = "true";
        let mut lexer = Lexer::new(src);

        lex_keyword(&mut lexer);

        let a = format!("{:?}", lexer.tokens.last().unwrap());
        let b = format!(
            "{:?}",
            Token {
                kind: TokenKind::True,
                size: 4
            }
        );

        assert_eq!(a, b);
    }
}
