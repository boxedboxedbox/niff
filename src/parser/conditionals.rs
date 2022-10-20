use super::expressions::*;
use super::*;

/// Parses an if statement.
pub fn parse_if_statement(p: &mut Parser) {
    // if (expr) {}
    if p.tokens[p.i].kind == TokenKind::If && p.tokens[p.i + 1].kind == TokenKind::LeftParen {
        p.advance_with(2);

        let mut expr = Vec::new();

        // while p.tokens[p.i].kind != TokenKind::RightParen {
        if p.len < p.i {
            panic!("syntax error: missing ')'");
        }

        // expr.push(p.tokens[p.i]);
        // }

        let condition = parse_expression(expr);

        if p.tokens[p.i].kind != TokenKind::RightParen {
            panic!("syntax error")
        }

        p.add_token(ParsedToken::If(condition));
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         lexer::*,
//         parser::{conditionals::parse_if_statement, Parser},
//     };
//
//     #[test]
//     fn test_parse_if_statement() {
//         let code = r#"if (a > b) {}"#;
//
//         let tokens = lexer(code);
//
//         let a = format!("{:?}", tokens);
//         let b = format!(
//             "{:?}",
//             vec![
//                 Token {
//                     kind: TokenKind::If,
//                     size: 2
//                 },
//                 Token {
//                     kind: TokenKind::LeftParen,
//                     size: 1
//                 },
//                 Token {
//                     kind: TokenKind::Unknown("a".to_string()),
//                     size: 1
//                 },
//                 Token {
//                     kind: TokenKind::GreaterThan,
//                     size: 1
//                 },
//                 Token {
//                     kind: TokenKind::Unknown("b".to_string()),
//                     size: 1
//                 },
//                 Token {
//                     kind: TokenKind::RightParen,
//                     size: 1
//                 },
//                 Token {
//                     kind: TokenKind::LeftBracket,
//                     size: 1
//                 },
//                 Token {
//                     kind: TokenKind::RightBracket,
//                     size: 1
//                 },
//                 Token {
//                     kind: TokenKind::Eof,
//                     size: 0
//                 }
//             ]
//         );
//
//         assert_eq!(a, b);
//
//         let mut parser = Parser::new(tokens);
//
//         parse_if_statement(&mut parser);
//
//         let c = format!("{:?}", parser.output);
//         let d = "[If()]".to_string();
//
//         assert_eq!(c, d);
//     }
// }
