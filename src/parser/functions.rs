use super::*;
use crate::lexer::*;

/// Arguments passed to a function. (name, modifiers, type)
pub type Parameters = Vec<(String, Vec<Modifiers>, Type)>;
pub type CallParameters = Vec<(String, Vec<Modifiers>)>;

/// Parses a function and appends the token to the list.
///
/// # Panics
///
/// Panics if there was a mistake in the syntax.
pub fn parse_function(p: &mut Parser) {
    // TODO: remove all the clones
    let name;

    // keyword and name
    if p.tokens[p.i].kind == TokenKind::Fn
        && p.tokens[p.i + 1].kind == TokenKind::Unknown(String::from(""))
    {
        match p.tokens[p.i + 1].kind.clone() {
            TokenKind::Unknown(a) => {
                p.advance_with(2);
                name = a
            }
            _ => unreachable!(),
        }
    } else {
        panic!("syntax error or idk: {:#?}, {:?}", p, p.tokens[p.i].kind)
    }

    if p.tokens[p.i].kind != TokenKind::LeftParen {
        panic!("syntax error or idk: {:#?}, {:?}", p, p.tokens[p.i].kind)
    }

    p.advance();

    let mut parameters = Vec::new();
    let mut param_name;

    loop {
        let mut param_modifiers = Vec::new();
        let mut param_type = Type::None; // dis

        // if it's a identifier
        if p.tokens[p.i].kind == TokenKind::Unknown(String::from("")) {
            match p.tokens[p.i].kind.clone() {
                TokenKind::Unknown(a) => {
                    p.advance();

                    param_name = a
                }
                _ => unreachable!(),
            }

            // Modifiers
            if p.tokens[p.i].kind == TokenKind::Mut {
                param_modifiers.push(Modifiers::Mutable)
            }

            if p.tokens[p.i].kind == TokenKind::Const {
                param_modifiers.push(Modifiers::Constant)
            }

            // Param type
            if p.tokens[p.i].kind == TokenKind::Type(Type::None) {
                match p.tokens[p.i].kind.clone() {
                    TokenKind::Type(a) => {
                        p.advance();

                        param_type = a;
                    }
                    _ => panic!("syntax error or something: {:#?}", p),
                }
            }

            if p.tokens[p.i].kind == TokenKind::Comma {
                p.advance();
            }

            parameters.push((param_name, param_modifiers, param_type));
        }

        if p.tokens[p.i].kind == TokenKind::RightParen {
            p.advance();
            break;
        }
    }

    let return_type = match p.tokens[p.i].kind.clone() {
        TokenKind::Type(a) => a,
        _ => Type::Void,
    };

    p.add_token(ParsedToken::Function(name, parameters, return_type));
}

/// Parses a function call and appends the token to the list.
pub fn parse_function_call(p: &mut Parser) {
    // TODO: remove all the clones

    // Identifier

    let name = match p.tokens[p.i].kind.clone() {
        TokenKind::Unknown(a) => a,
        _ => panic!("syntax error"),
    };

    p.advance();

    if p.tokens[p.i].kind == TokenKind::LeftParen {
        p.advance();
    } else {
        panic!("syntax error")
    }

    let mut parameters = Vec::new();
    let mut parameter_name;

    // parameters
    loop {
        let mut parameter_modifiers = Vec::new();

        // Modifiers
        if p.tokens[p.i].kind == TokenKind::Mut {
            parameter_modifiers.push(Modifiers::Mutable);
            p.advance();
        }

        if p.tokens[p.i].kind == TokenKind::Const {
            parameter_modifiers.push(Modifiers::Constant);
            p.advance();
        }

        // Variable identifier (TODO: add support for literals)
        if p.tokens[p.i].kind == TokenKind::RightParen {
            p.advance();

            break;
        } else {
            match p.tokens[p.i].kind.clone() {
                TokenKind::Unknown(a) => parameter_name = a,
                _ => panic!("syntax error??"),
            };
        }

        p.advance();

        if p.tokens[p.i].kind == TokenKind::Comma {
            parameters.push((parameter_name, parameter_modifiers));

            p.advance();
        } else if p.tokens[p.i].kind == TokenKind::RightParen {
            parameters.push((parameter_name, parameter_modifiers));

            p.advance();

            break;
        }
    }

    p.add_token(ParsedToken::FunctionCall(name, parameters));
}

#[cfg(test)]
mod tests {
    // use crate::{
    //     lexer::*,
    //     parser::{functions::parse_function, functions::parse_function_call, Parser},
    // };
    //
    // #[test]
    // fn test_parse_function() {
    //     let code = r#"fn main() {}"#;
    //
    //     let tokens = lexer(code);
    //
    //     let mut parser = Parser::new(tokens);
    //
    //     parse_function(&mut parser);
    //
    //     let c = format!("{:?}", parser.output);
    //     let d = "[Function(\"main\", [], Void)]".to_string();
    //
    //     assert_eq!(c, d);
    // }
    // #[test]
    // fn test_parse_function_with_parameters() {
    //     let code = r#"fn main(foo void) {}"#;
    //
    //     let tokens = lexer(code);
    //
    //     let mut parser = Parser::new(tokens);
    //
    //     parse_function(&mut parser);
    //
    //     let a = format!("{:?}", parser.output);
    //     let b = "[Function(\"main\", [(\"foo\", [], Void)], Void)]".to_string();
    //
    //     assert_eq!(a, b);
    // }
    // #[test]
    // fn test_parse_function_with_return_type() {
    //     let code = r#"fn main() i32 {}"#;
    //
    //     let tokens = lexer(code);
    //
    //     let mut parser = Parser::new(tokens);
    //
    //     parse_function(&mut parser);
    //
    //     let a = format!("{:?}", parser.output);
    //     let b = "[Function(\"main\", [], I32)]".to_string();
    //
    //     assert_eq!(a, b);
    // }
    // #[test]
    // fn test_parse_function_with_multiple_parameters() {
    //     let code = r#"fn main(foo i32, bar void) {}"#;
    //
    //     let tokens = lexer(code);
    //
    //     let mut parser = Parser::new(tokens);
    //
    //     parse_function(&mut parser);
    //
    //     let a = format!("{:?}", parser.output);
    //     let b = "[Function(\"main\", [(\"foo\", [], I32), (\"bar\", [], Void)], Void)]".to_string();
    //
    //     assert_eq!(a, b);
    // }
    // #[test]
    // fn test_parse_function_with_multiple_parameters_and_a_return_type() {
    //     let code = r#"fn main(foo i32, bar void) u32 {}"#;
    //
    //     let tokens = lexer(code);
    //
    //     let mut parser = Parser::new(tokens);
    //
    //     parse_function(&mut parser);
    //
    //     let a = format!("{:?}", parser.output);
    //     let b = "[Function(\"main\", [(\"foo\", [], I32), (\"bar\", [], Void)], U32)]".to_string();
    //
    //     assert_eq!(a, b);
    // }
    // #[test]
    // fn test_parse_function_call() {
    //     let code = r#"foo();"#;
    //
    //     let tokens = lexer(code);
    //
    //     let mut parser = Parser::new(tokens);
    //
    //     parse_function_call(&mut parser);
    //
    //     let a = format!("{:?}", parser.output);
    //     let b = "[FunctionCall(\"foo\", [])]".to_string();
    //
    //     assert_eq!(a, b);
    // }
    // #[test]
    // fn test_parse_function_call_with_parameters() {
    //     let code = r#"foo(bar);"#;
    //
    //     let tokens = lexer(code);
    //
    //     let mut parser = Parser::new(tokens);
    //
    //     parse_function_call(&mut parser);
    //
    //     let a = format!("{:?}", parser.output);
    //     let b = "[FunctionCall(\"foo\", [(\"bar\", [])])]".to_string();
    //
    //     assert_eq!(a, b);
    // }
    // #[test]
    // fn test_parse_function_call_with_multiple_parameters() {
    //     let code = r#"foo(bar, idk);"#;
    //
    //     let tokens = lexer(code);
    //
    //     let mut parser = Parser::new(tokens);
    //
    //     parse_function_call(&mut parser);
    //
    //     let a = format!("{:?}", parser.output);
    //     let b = "[FunctionCall(\"foo\", [(\"bar\", []), (\"idk\", [])])]".to_string();
    //
    //     assert_eq!(a, b);
    // }
}
