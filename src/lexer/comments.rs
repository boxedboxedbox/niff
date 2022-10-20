/// Functions to lex comments.
///
/// Comments are just discarded and no interesting things are done with them.
/// I don't plan on doing anything cool with comments in the future.
use super::Lexer;

pub fn skip_comments(l: &mut Lexer) -> bool {
    if l.src.get(l.i) == Some('/').as_ref() {
        if l.src.get(l.i + 1) == Some('/').as_ref() {
            l.advance_with(2);
            skip_line_comment(l);

            return true;
        } else if l.src.get(l.i + 1) == Some('*').as_ref() {
            l.advance_with(2);
            skip_block_comment(l);

            return true;
        }
    }

    false
}

// Skips a line comment.
fn skip_line_comment(l: &mut Lexer) {
    while l.len > l.i {
        if l.src[l.i] == '\n' {
            l.advance();

            return;
        }

        l.advance();
    }
}

// Skips a block comment.
fn skip_block_comment(l: &mut Lexer) {
    while l.len > l.i {
        if l.src[l.i] == '*' && l.src[l.i + 1] == '/' {
            l.advance_with(2);

            return;
        }

        l.advance();
    }

    panic!("syntax error: no terminating \"*/\" found")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{lexer, Lexer, Token, TokenKind};

    #[test]
    fn test_skip_line_comment() {
        let c0 = "// hello world\na";
        let c1 = "// hello world\n";

        let mut l0 = Lexer::new(c0);
        let mut l1 = Lexer::new(c1);

        skip_line_comment(&mut l0);
        skip_line_comment(&mut l1);

        assert_eq!(l0.src.get(l0.i), Some('a').as_ref(), "Test case 1 failed");
        assert_eq!(l1.src.get(l1.i), None, "Test case 2 failed");
    }
    #[test]
    fn test_skip_block_comment() {
        let c0 = "/* hello world */a";
        let c1 = "/* hello\n world */b";
        let c2 = "/* hello world */";

        let mut l0 = Lexer::new(c0);
        let mut l1 = Lexer::new(c1);
        let mut l2 = Lexer::new(c2);

        skip_block_comment(&mut l0);
        skip_block_comment(&mut l1);
        skip_block_comment(&mut l2);

        assert_eq!(l0.src.get(l0.i), Some('a').as_ref(), "Test case 1 failed");
        assert_eq!(l1.src.get(l1.i), Some('b').as_ref(), "Test case 2 failed");
        assert_eq!(l2.src.get(l2.i), None, "Test case 3 failed");
    }
    #[test]
    fn test_lexer_with_line_comments() {
        let c0 = "// hello world\n";
        let c1 = "// hello world\n;";

        let t0 = lexer(c0);
        let t1 = lexer(c1);

        let o0 = format!("{:?}", t0);
        let o1 = format!("{:?}", t1);

        let e0 = format!(
            "{:?}",
            vec![Token {
                kind: TokenKind::Eof,
                size: 0
            }]
        );
        let e1 = format!(
            "{:?}",
            vec![
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

        assert_eq!(o0, e0, "Test case 1 failed");
        assert_eq!(o1, e1, "Test case 2 failed");
    }
    #[test]
    fn test_lexer_with_block_comments() {
        let c0 = "/* hello world */";
        let c1 = "/* hello world */;";
        let c2 = "/* hello\n world */;";

        let t0 = lexer(c0);
        let t1 = lexer(c1);
        let t2 = lexer(c2);

        let o0 = format!("{:?}", t0);
        let o1 = format!("{:?}", t1);
        let o2 = format!("{:?}", t2);

        let e0 = format!(
            "{:?}",
            vec![Token {
                kind: TokenKind::Eof,
                size: 0
            }]
        );
        let e1 = format!(
            "{:?}",
            vec![
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
                    kind: TokenKind::Eof,
                    size: 0
                }
            ]
        );

        assert_eq!(o0, e0, "Test case 1 failed");
        assert_eq!(o1, e1, "Test case 2 failed");
        assert_eq!(o2, e2, "Test case 3 failed");
    }
}
