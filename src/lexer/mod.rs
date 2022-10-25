mod comments;
mod keywords;
mod other;
mod sequences;
mod strings;

use crate::parser::Type;

#[derive(Debug)]
pub struct Token {
    /// What token is it?
    pub kind: TokenKind,
    /// How many bytes/characters was consumed to obtain it. Not used currently.
    pub size: usize,
}

impl From<(TokenKind, usize)> for Token {
    fn from(tuple: (TokenKind, usize)) -> Self {
        Self {
            kind: tuple.0,
            size: tuple.1,
        }
    }
}

// A token.
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum TokenKind {
    /// "="
    Eq,
    /// "+"
    Plus,
    /// "-"
    Minus,
    /// "%"
    Percent,
    /// "<"
    LessThan,
    /// ">"
    GreaterThan,
    /// "/"
    Slash,
    /// "*"
    Star,
    /// "&"
    And,
    /// "^"
    Caret,
    /// "|"
    Pipe,
    /// "?"
    Question,
    /// "!"
    Bang,
    /// ";"
    Semicolon,
    /// ":"
    Colon,
    /// "("
    LeftParen,
    /// ")"
    RightParen,
    /// "["
    LeftBrace,
    /// "]"
    RightBrace,
    /// "{"
    LeftBracket,
    /// "}"
    RightBracket,
    /// ","
    Comma,
    /// "."
    Dot,
    /// "'"
    Quote,
    /// "var"
    Var,
    /// "fn"
    Fn,
    /// "if"
    If,
    /// "elif"
    Elif,
    /// "else"
    Else,
    /// "while"
    While,
    /// "for"
    For,
    /// "return"
    Return,
    /// "mut"
    Mut,
    /// "const"
    Const,
    /// "false"
    False,
    /// "true"
    True,
    /// An integer literal (value)
    IntegerLiteral(i64),
    /// A float literal (value)
    FloatLiteral(f64),
    /// A string literal (data)
    StringLiteral(String),
    /// A char literal (character)
    CharLiteral(char),
    /// Unknown or identifier (identifier/unknown token)
    Unknown(String),
    /// "::"
    DoubleColon,
    /// "=="
    DoubleEquals,
    /// "!="
    BangEquals,
    /// ">="
    GtEq,
    /// "<="
    LtEq,
    /// ".."
    DoubleDot,
    /// "=>"
    FatArrow,
    /// "->"
    Arrow,
    /// Type
    Type(Type),
    /// End of input
    Eof,
}

impl PartialEq for TokenKind {
    /// Note: this does some things that probably don't expect. Please read the code.
    fn eq(&self, other: &Self) -> bool {
        // TODO: implement for the rest of the enum members.
        match (self, other) {
            (TokenKind::Unknown(_), TokenKind::Unknown(_)) => true,
            (TokenKind::Type(_), TokenKind::Type(_)) => true,
            (TokenKind::IntegerLiteral(_), TokenKind::IntegerLiteral(_)) => true,
            (TokenKind::FloatLiteral(_), TokenKind::FloatLiteral(_)) => true,
            (TokenKind::StringLiteral(_), TokenKind::StringLiteral(_)) => true,
            _ => matches!(
                (self, other),
                (TokenKind::Fn, TokenKind::Fn)
                    | (TokenKind::Var, TokenKind::Var)
                    | (TokenKind::Eq, TokenKind::Eq)
                    | (TokenKind::Semicolon, TokenKind::Semicolon)
                    | (TokenKind::Colon, TokenKind::Colon)
                    | (TokenKind::Dot, TokenKind::Dot)
                    | (TokenKind::Comma, TokenKind::Comma)
                    | (TokenKind::Slash, TokenKind::Slash)
                    | (TokenKind::Star, TokenKind::Star)
                    | (TokenKind::LessThan, TokenKind::LessThan)
                    | (TokenKind::GreaterThan, TokenKind::GreaterThan)
                    | (TokenKind::LeftParen, TokenKind::LeftParen)
                    | (TokenKind::RightParen, TokenKind::RightParen)
                    | (TokenKind::LeftBrace, TokenKind::LeftBrace)
                    | (TokenKind::RightBrace, TokenKind::RightBrace)
                    | (TokenKind::LeftBracket, TokenKind::LeftBracket)
                    | (TokenKind::RightBracket, TokenKind::RightBracket)
                    | (TokenKind::Quote, TokenKind::Quote)
                    | (TokenKind::Caret, TokenKind::Caret)
                    | (TokenKind::And, TokenKind::And)
                    | (TokenKind::Pipe, TokenKind::Pipe)
                    | (TokenKind::Percent, TokenKind::Percent)
                    | (TokenKind::DoubleEquals, TokenKind::DoubleEquals)
                    | (TokenKind::BangEquals, TokenKind::BangEquals)
                    | (TokenKind::GtEq, TokenKind::GtEq)
                    | (TokenKind::LtEq, TokenKind::LtEq)
                    | (TokenKind::Eof, TokenKind::Eof)
            ),
        }
    }
}

/// Stores information used by the lexer.
#[derive(Debug)]
pub struct Lexer {
    i: usize,
    src: Vec<char>,
    tokens: Vec<Token>,
    len: usize,
}

impl Lexer {
    #[inline]
    fn new(src: &str) -> Lexer {
        Lexer {
            i: 0,
            src: src.chars().collect(),
            tokens: Vec::new(),
            len: src.len(),
        }
    }
    /// Advances the "cursor" by one.
    #[inline(always)]
    fn advance(&mut self) {
        self.i += 1;
    }
    /// Advances the "cursor" by `val`.
    /// Use advance() when only advancing by one.
    ///
    /// # Panics
    ///
    /// Panics on debug mode if `val` is 0.
    fn advance_with(&mut self, val: usize) {
        debug_assert_ne!(
            val, 0,
            "You should use advance() instead, when advancing only by one."
        );

        self.i += val;
    }
    /// Adds `token` to the `tokens` list on the object.
    fn add_token(&mut self, token: Token) {
        self.tokens.push(token)
    }
    /// Is `c` a whitespace character?
    #[inline]
    fn is_whitespace(c: char) -> bool {
        matches!(
            c,
            ' '
            | '\t'
            | '\n'
            | '\r'
            | '\u{000B}' // vertical tab
            | '\u{000C}' // form feed
        )
    }
    /// Skips whitespace characters.
    #[inline]
    fn skip_whitespace(&mut self) {
        while self.len > self.i {
            if Lexer::is_whitespace(self.src[self.i]) {
                self.advance();
            } else {
                break;
            }
        }
    }
}

pub fn lexer(src: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(src);

    while lexer.len != lexer.i {
        // whitespaces
        lexer.skip_whitespace();

        // comments
        if comments::skip_comments(&mut lexer) {
            continue;
        }

        // keywords
        if keywords::lex_keyword(&mut lexer) {
            continue;
        }

        // sequences
        if sequences::lex_sequence(&mut lexer) {
            continue;
        }

        // character literals
        if strings::lex_char(&mut lexer) {
            continue;
        }

        // other
        match lexer.src.get(lexer.i) {
            Some('=') => {
                lexer.add_token((TokenKind::Eq, 1).into());
                lexer.advance();
            }
            Some('(') => {
                lexer.add_token((TokenKind::LeftParen, 1).into());
                lexer.advance();
            }
            Some(')') => {
                lexer.add_token((TokenKind::RightParen, 1).into());
                lexer.advance();
            }
            Some('[') => {
                lexer.add_token((TokenKind::LeftBrace, 1).into());
                lexer.advance();
            }
            Some(']') => {
                lexer.add_token((TokenKind::RightBrace, 1).into());
                lexer.advance();
            }
            Some('{') => {
                lexer.add_token((TokenKind::LeftBracket, 1).into());
                lexer.advance();
            }
            Some('}') => {
                lexer.add_token((TokenKind::RightBracket, 1).into());
                lexer.advance();
            }
            Some('!') => {
                lexer.add_token((TokenKind::Bang, 1).into());
                lexer.advance();
            }
            Some('.') => {
                lexer.add_token((TokenKind::Dot, 1).into());
                lexer.advance();
            }
            Some(',') => {
                lexer.add_token((TokenKind::Comma, 1).into());
                lexer.advance();
            }
            Some(';') => {
                lexer.add_token((TokenKind::Semicolon, 1).into());
                lexer.advance();
            }
            Some(':') => {
                lexer.add_token((TokenKind::Colon, 1).into());
                lexer.advance();
            }
            Some('\'') => {
                lexer.add_token((TokenKind::Quote, 1).into());
                lexer.advance();
            }
            Some('*') => {
                lexer.add_token((TokenKind::Star, 1).into());
                lexer.advance();
            }
            Some('/') => {
                lexer.add_token((TokenKind::Slash, 1).into());
                lexer.advance();
            }
            Some('<') => {
                lexer.add_token((TokenKind::LessThan, 1).into());
                lexer.advance();
            }
            Some('>') => {
                lexer.add_token((TokenKind::GreaterThan, 1).into());
                lexer.advance();
            }
            Some('?') => {
                lexer.add_token((TokenKind::Question, 1).into());
                lexer.advance();
            }
            Some('+') => {
                lexer.add_token((TokenKind::Plus, 1).into());
                lexer.advance();
            }
            Some('-') => {
                lexer.add_token((TokenKind::Minus, 1).into());
                lexer.advance();
            }
            Some('^') => {
                lexer.add_token((TokenKind::Caret, 1).into());
                lexer.advance();
            }
            Some('|') => {
                lexer.add_token((TokenKind::Pipe, 1).into());
                lexer.advance();
            }
            Some('&') => {
                lexer.add_token((TokenKind::And, 1).into());
                lexer.advance();
            }
            Some('%') => {
                lexer.add_token((TokenKind::Percent, 1).into());
                lexer.advance();
            }
            token => {
                // Unknown or identifier and type
                if lexer.len > lexer.i {
                    other::lex_number(&mut lexer);
                    strings::lex_string(&mut lexer);
                    other::lex_type(&mut lexer);
                    other::lex_unknown_or_ident(&mut lexer);
                } else {
                    panic!(
                        "unknown token at index [{}]: {:?}, before it: {:?}",
                        lexer.i,
                        token,
                        lexer.src.get(lexer.i - 1)
                    );
                }
            }
        }
    }

    lexer.add_token(Token {
        kind: TokenKind::Eof,
        size: 0,
    });

    lexer.tokens
}

#[cfg(test)]
mod tests {
    use crate::lexer::*;

    #[test]
    fn test_lexer() {
        let code = "fn main() {}";

        let tokens = lexer(code);

        let a = format!("{:?}", tokens);
        let b = format!(
            "{:?}",
            vec![
                Token {
                    kind: TokenKind::Fn,
                    size: 3
                },
                Token {
                    kind: TokenKind::Unknown("main".to_string()),
                    size: 4
                },
                Token {
                    kind: TokenKind::LeftParen,
                    size: 1
                },
                Token {
                    kind: TokenKind::RightParen,
                    size: 1
                },
                Token {
                    kind: TokenKind::LeftBracket,
                    size: 1
                },
                Token {
                    kind: TokenKind::RightBracket,
                    size: 1
                },
                Token {
                    kind: TokenKind::Eof,
                    size: 0
                }
            ]
        );

        assert_eq!(a, b);
    }
    #[test]
    fn test_partialeq_impl_for_tokenkind_with_unknown() {
        let token = Token {
            kind: TokenKind::Unknown("main".to_string()),
            size: 3,
        };

        // This returns true, which is expected behaviour.
        // I know it's weird, but I was lazy. Make it better if it bothers you.
        // Warning: You have to fix ~200 lines of code that rely on that as well.
        assert_eq!(token.kind, TokenKind::Unknown("".to_string()))
    }
}
