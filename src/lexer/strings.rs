use super::*;

/// Lex a string literal.
pub fn lex_string(l: &mut Lexer) {
    if l.src.get(l.i) == Some('"').as_ref() {
        let mut res = String::new();
        let mut ctr = 1;

        l.advance();

        while l.len > l.i {
            if l.src[l.i] == '"' {
                l.advance();
                ctr += 1;
                break;
            }

            if l.src.get(l.i) == Some('\\').as_ref() && l.src.get(l.i + 1) == Some('"').as_ref() {
                l.advance_with(2);
                ctr += 2;
                res.push('"');
            } else if l.src.get(l.i).is_none() || l.src.get(l.i + 1).is_none() {
                break;
            } else {
                res.push(l.src[l.i]);
                l.advance();
                ctr += 1;
            }
        }

        l.add_token(Token {
            kind: TokenKind::StringLiteral(res),
            size: ctr,
        });
    }
}

/// Lex a character literal.
pub fn lex_char(l: &mut Lexer) -> bool {
    if l.src[l.i] == '\'' {
        let res;
        let mut ctr = 1;

        l.advance();

        if l.src.get(l.i) == Some('\\').as_ref() && l.src.get(l.i + 1) == Some('\'').as_ref() {
            res = '\'';

            ctr += 2;
            l.advance_with(2);
        } else if l.src.get(l.i) == Some('\\').as_ref() && l.src.get(l.i + 1) == Some('\\').as_ref()
        {
            res = '\\';

            ctr += 2;
            l.advance_with(2);
        } else {
            res = l.src[l.i];

            ctr += 1;
            l.advance();
        }

        println!("{:?}", l);
        if l.src.get(l.i) == Some('\'').as_ref() {
            ctr += 1;
            l.advance();

            l.add_token(Token {
                kind: TokenKind::CharLiteral(res),
                size: ctr,
            });

            return true;
        } else {
            panic!("syntax error: missing quote");
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::{lex_char, lex_string};
    use crate::lexer::*;

    #[test]
    fn test_lex_char() {
        let c0 = "'a'";
        let c1 = "'1'";
        let c2 = "';'";
        let c3 = "'\''";
        let c4 = "'\\\\'";

        let mut l0 = Lexer::new(c0);
        let mut l1 = Lexer::new(c1);
        let mut l2 = Lexer::new(c2);
        let mut l3 = Lexer::new(c3);
        let mut l4 = Lexer::new(c4);

        lex_char(&mut l0);
        lex_char(&mut l1);
        lex_char(&mut l2);
        lex_char(&mut l3);
        lex_char(&mut l4);

        let o0 = format!("{:?}", l0.tokens);
        let o1 = format!("{:?}", l1.tokens);
        let o2 = format!("{:?}", l2.tokens);
        let o3 = format!("{:?}", l3.tokens);
        let o4 = format!("{:?}", l4.tokens);

        let e0 = format!(
            "{:?}",
            vec![Token {
                kind: TokenKind::CharLiteral('a'),
                size: 3
            }]
        );
        let e1 = format!(
            "{:?}",
            vec![Token {
                kind: TokenKind::CharLiteral('1'),
                size: 3
            }]
        );
        let e2 = format!(
            "{:?}",
            vec![Token {
                kind: TokenKind::CharLiteral(';'),
                size: 3
            }]
        );
        let e3 = format!(
            "{:?}",
            vec![Token {
                kind: TokenKind::CharLiteral('\''),
                size: 3
            }]
        );
        let e4 = format!(
            "{:?}",
            vec![Token {
                kind: TokenKind::CharLiteral('\\'),
                size: 4
            }]
        );

        assert_eq!(o0, e0, "Test case 1 failed");
        assert_eq!(o1, e1, "Test case 2 failed");
        assert_eq!(o2, e2, "Test case 3 failed");
        assert_eq!(o3, e3, "Test case 4 failed");
        assert_eq!(o4, e4, "Test case 5 failed");
    }
    #[test]
    fn test_lex_string() {
        let str0 = "\"Hello, World!\"";
        let str1 = "\"Hello, World!\";";
        let str2 = "\"Hello,\\\" World!\"";

        let mut l0 = Lexer::new(str0);
        let mut l1 = Lexer::new(str1);
        let mut l2 = Lexer::new(str2);

        lex_string(&mut l0);
        lex_string(&mut l1);
        lex_string(&mut l2);

        let o0 = format!("{:?}", l0.tokens);
        let o1 = format!("{:?}", l1.tokens);
        let o2 = format!("{:?}", l2.tokens);

        let expected = format!(
            "{:?}",
            vec![Token {
                kind: TokenKind::StringLiteral("Hello, World!".to_string()),
                size: 15
            }]
        );
        let expected2 = format!(
            "{:?}",
            vec![Token {
                kind: TokenKind::StringLiteral("Hello,\" World!".to_string()),
                size: 17
            }]
        );

        assert_eq!(o0, expected, "Test case 1 failed");
        assert_eq!(o1, expected, "Test case 2 failed");
        assert_eq!(o2, expected2, "Test case 3 failed");
    }
    #[test]
    fn test_lexer_with_strings() {
        let str0 = "\"Hello, World!\"";
        let str1 = "\"Hello, World!\";";
        let str2 = ";\"Hello, World!\"";
        let str3 = "\"Hello,\\\" World!\"";

        let t0 = lexer(str0);
        let t1 = lexer(str1);
        let t2 = lexer(str2);
        let t3 = lexer(str3);

        let o0 = format!("{:?}", t0);
        let o1 = format!("{:?}", t1);
        let o2 = format!("{:?}", t2);
        let o3 = format!("{:?}", t3);

        let e0 = format!(
            "{:?}",
            vec![
                Token {
                    kind: TokenKind::StringLiteral("Hello, World!".to_string()),
                    size: 15
                },
                Token {
                    kind: TokenKind::Eof,
                    size: 0
                }
            ]
        );
        let e1 = format!(
            "{:?}",
            vec![
                Token {
                    kind: TokenKind::StringLiteral("Hello, World!".to_string()),
                    size: 15
                },
                Token {
                    kind: TokenKind::Semicolon,
                    size: 1
                },
                Token {
                    kind: TokenKind::Eof,
                    size: 0
                }
            ]
        );
        let e2 = format!(
            "{:?}",
            vec![
                Token {
                    kind: TokenKind::Semicolon,
                    size: 1
                },
                Token {
                    kind: TokenKind::StringLiteral("Hello, World!".to_string()),
                    size: 15
                },
                Token {
                    kind: TokenKind::Eof,
                    size: 0
                }
            ]
        );
        let e3 = format!(
            "{:?}",
            vec![
                Token {
                    kind: TokenKind::StringLiteral("Hello,\" World!".to_string()),
                    size: 17
                },
                Token {
                    kind: TokenKind::Eof,
                    size: 0
                }
            ]
        );

        assert_eq!(o0, e0, "Test case 1 failed");
        assert_eq!(o1, e1, "Test case 2 failed");
        assert_eq!(o2, e2, "Test case 3 failed");
        assert_eq!(o3, e3, "Test case 4 failed");
    }
    #[test]
    fn test_lexer_with_characters() {
        let c0 = "'a'";
        let c1 = "'1'";
        let c2 = "';'";

        let t0 = lexer(c0);
        let t1 = lexer(c1);
        let t2 = lexer(c2);

        let o0 = format!("{:?}", t0);
        let o1 = format!("{:?}", t1);
        let o2 = format!("{:?}", t2);

        let e0 = format!(
            "{:?}",
            vec![
                Token {
                    kind: TokenKind::CharLiteral('a'),
                    size: 3
                },
                Token {
                    kind: TokenKind::Eof,
                    size: 0
                }
            ]
        );
        let e1 = format!(
            "{:?}",
            vec![
                Token {
                    kind: TokenKind::CharLiteral('1'),
                    size: 3
                },
                Token {
                    kind: TokenKind::Eof,
                    size: 0
                }
            ]
        );
        let e2 = format!(
            "{:?}",
            vec![
                Token {
                    kind: TokenKind::CharLiteral(';'),
                    size: 3
                },
                Token {
                    kind: TokenKind::Eof,
                    size: 0
                }
            ]
        );

        assert_eq!(o0, e0, "Test case 1 failed");
        assert_eq!(o1, e1, "Test case 2 failed");
        assert_eq!(o2, e2, "Test case 3 failed");
    }
    #[test]
    fn test_lexer_with_characters_with_escapes() {
        let c0 = "'\\''";
        let c1 = "'\\\\'";

        let t0 = lexer(c0);
        let t1 = lexer(c1);

        let o0 = format!("{:?}", t0);
        let o1 = format!("{:?}", t1);

        let e0 = format!(
            "{:?}",
            vec![
                Token {
                    kind: TokenKind::CharLiteral('\''),
                    size: 4
                },
                Token {
                    kind: TokenKind::Eof,
                    size: 0
                }
            ]
        );
        let e1 = format!(
            "{:?}",
            vec![
                Token {
                    kind: TokenKind::CharLiteral('\\'),
                    size: 4
                },
                Token {
                    kind: TokenKind::Eof,
                    size: 0
                }
            ]
        );

        assert_eq!(o0, e0, "Test case 1 failed");
        assert_eq!(o1, e1, "Test case 2 failed");
    }
}
