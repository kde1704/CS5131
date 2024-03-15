use pyo3::prelude::*;
use clex::{Keyword, Lexer, Token};


#[pyfunction]
fn lex(src: &str) -> Vec<String> {
    let lexer = Lexer::from(src)
        .filter(|x| x.token != Token::Comment); // filter out comments
    
    // dont care about memory :joy_cat:
    let mut tokens: Vec<String> = vec![];

    for lexeme in lexer.into_iter() {
        match lexeme.token {
            // tokenised integers digit by digit
            Token::Int => {
                tokens.extend(lexeme.slice.chars().map(|c| c.to_string()))
            }
            Token::Identifier => {
                // despite being named identifier it might be a keyword
                // we check what .keyword() is to determine whether its
                // a keyword or identifier
                tokens.push(match lexeme.keyword() {
                    // this might be the worlds most horrible code
                    Some(Keyword::Auto) => String::from("auto"),
                    Some(Keyword::Break) => String::from("break"),
                    Some(Keyword::Case) => String::from("case"),
                    Some(Keyword::Char) => String::from("char"),
                    Some(Keyword::Const) => String::from("const"),
                    Some(Keyword::Continue) => String::from("continue"),
                    Some(Keyword::Default) => String::from("default"),
                    Some(Keyword::Do) => String::from("do"),
                    Some(Keyword::Double) => String::from("double"),
                    Some(Keyword::Else) => String::from("else"),
                    Some(Keyword::Enum) => String::from("enum"),
                    Some(Keyword::Extern) => String::from("extern"),
                    Some(Keyword::Float) => String::from("float"),
                    Some(Keyword::For) => String::from("for"),
                    Some(Keyword::Goto) => String::from("goto"),
                    Some(Keyword::If) => String::from("if"),
                    Some(Keyword::Inline) => String::from("inline"),
                    Some(Keyword::Int) => String::from("int"),
                    Some(Keyword::Long) => String::from("long"),
                    Some(Keyword::Register) => String::from("register"),
                    Some(Keyword::Restrict) => String::from("restrict"),
                    Some(Keyword::Return) => String::from("return"),
                    Some(Keyword::Short) => String::from("short"),
                    Some(Keyword::Signed) => String::from("signed"),
                    Some(Keyword::SizeOf) => String::from("sizeof"),
                    Some(Keyword::Static) => String::from("static"),
                    Some(Keyword::Struct) => String::from("struct"),
                    Some(Keyword::Switch) => String::from("switch"),
                    Some(Keyword::TypeDef) => String::from("typedef"),
                    Some(Keyword::Union) => String::from("union"),
                    Some(Keyword::Unsigned) => String::from("unsigned"),
                    Some(Keyword::Void) => String::from("void"),
                    Some(Keyword::Volatile) => String::from("volatile"),
                    Some(Keyword::While) => String::from("while"),
                    Some(Keyword::AlignAs) => String::from("_Alignas"),
                    Some(Keyword::AlignOf) => String::from("_Alignof"),
                    Some(Keyword::Atomic) => String::from("_Atomic"),
                    Some(Keyword::Bool) => String::from("_Bool"),
                    Some(Keyword::Complex) => String::from("_Complex"),
                    Some(Keyword::Generic) => String::from("_Generic"),
                    Some(Keyword::Imaginary) => String::from("_Imaginary"),
                    Some(Keyword::NoReturn) => String::from("_Noreturn"),
                    Some(Keyword::StaticAssert) => String::from("_Static_assert"),
                    Some(Keyword::ThreadLocal) => String::from("_Thread_local"),
                    Some(Keyword::FuncName) => String::from("__func__"),
                    None => String::from("<|ident|>"),
                })
                // wallahi the match is finished
            }
            // we dont care about the precise value of floats and strings
            Token::Float => {
                tokens.push(String::from("<|float|>"))
            }
            Token::String => {
                tokens.push(String::from("<|str|>"))
            }
            _ => {
                tokens.push(lexeme.slice.to_string())
            }
        }
    }

    tokens
}

/// A Python module implemented in Rust.
#[pymodule]
fn lexer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(lex, m)?)?;
    Ok(())
}
