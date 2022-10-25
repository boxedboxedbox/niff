use super::{Lexer, Token, TokenKind};

/// Checks for sequences like "::" and "=>".
pub fn lex_sequence(l: &mut Lexer) -> bool {
    // "=="
    if l.src.get(l.i) == Some('=').as_ref() && l.src.get(l.i + 1) == Some('=').as_ref() {
        l.advance_with(2);

        l.add_token(Token {
            kind: TokenKind::DoubleEquals,
            size: 2,
        });

        return true;
    }

    // "!="
    if l.src.get(l.i) == Some('!').as_ref() && l.src.get(l.i + 1) == Some('=').as_ref() {
        l.advance_with(2);

        l.add_token(Token {
            kind: TokenKind::BangEquals,
            size: 2,
        });

        return true;
    }

    // ">="
    if l.src.get(l.i) == Some('>').as_ref() && l.src.get(l.i + 1) == Some('=').as_ref() {
        l.advance_with(2);

        l.add_token(Token {
            kind: TokenKind::GtEq,
            size: 2,
        });

        return true;
    }

    // "<="
    if l.src.get(l.i) == Some('<').as_ref() && l.src.get(l.i + 1) == Some('=').as_ref() {
        l.advance_with(2);

        l.add_token(Token {
            kind: TokenKind::LtEq,
            size: 2,
        });

        return true;
    }

    // "::"
    if l.src.get(l.i) == Some(':').as_ref() && l.src.get(l.i + 1) == Some(':').as_ref() {
        l.advance_with(2);

        l.add_token(Token {
            kind: TokenKind::DoubleColon,
            size: 2,
        });

        return true;
    }

    // ".."
    if l.src.get(l.i) == Some('.').as_ref() && l.src.get(l.i + 1) == Some('.').as_ref() {
        l.advance_with(2);

        l.add_token(Token {
            kind: TokenKind::DoubleDot,
            size: 2,
        });

        return true;
    }

    // "->"
    if l.src.get(l.i) == Some('-').as_ref() && l.src.get(l.i + 1) == Some('>').as_ref() {
        l.advance_with(2);

        l.add_token(Token {
            kind: TokenKind::Arrow,
            size: 2,
        });

        return true;
    }

    // "=>"
    if l.src.get(l.i) == Some('=').as_ref() && l.src.get(l.i + 1) == Some('>').as_ref() {
        l.advance_with(2);

        l.add_token(Token {
            kind: TokenKind::FatArrow,
            size: 2,
        });

        return true;
    }

    false
}

// TODO: write test for every sequence
#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn test_sequence_double_colon() {
        let src = "a::b";
        let mut lexer = Lexer::new(src);

        lexer.advance();

        lex_sequence(&mut lexer);

        let a = format!("{:?}", lexer.tokens.last().unwrap());
        let b = format!(
            "{:?}",
            Token {
                kind: TokenKind::DoubleColon,
                size: 2
            }
        );

        assert_eq!(a, b);
        assert_eq!(lexer.src[lexer.i], 'b');
    }

    #[test]
    fn test_sequence_double_dot() {
        let src = "0..23";
        let mut lexer = Lexer::new(src);

        lexer.advance();

        lex_sequence(&mut lexer);

        let a = format!("{:?}", lexer.tokens.last().unwrap());
        let b = format!(
            "{:?}",
            Token {
                kind: TokenKind::DoubleDot,
                size: 2
            }
        );

        assert_eq!(a, b);
        assert_eq!(lexer.src[lexer.i], '2');
    }
}
