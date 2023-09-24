pub use logos::*;
use super::*;

#[derive(Debug, Clone, Logos)]
pub enum HighlightToken {
    #[regex(r"([\d_]+|0x[\da-fA-F_]+|0b[01_]+)")]
    Integer,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 0)]
    Ident,

    #[token(";")]
    #[regex(r"//[^\n]*")]
    #[regex(r"[\s]")]
    Unused,

    #[token("(")]
    #[token(")")]
    #[token("[")]
    #[token("]")]
    #[token("{")]
    #[token("}")]
    Brackets,

    #[regex(r"(\+|\-|\*|/|%|&|\||\^|<<|>>)(=)?")]
    #[regex(r"(<|>|!|==|!=|<=|>=|&&|\|\||=|\.|::)")]
    Operator,

    #[token("var")]
    #[token("break")]
    #[token("continue")]
    #[token("return")]
    Keyword,

    #[token(",")]
    Comma,
}

impl HighlightToken {
    pub fn highlight(&self, next: Option<&Self>, text: &str) -> String {
        use HighlightToken::*;
        String::new() + "\x1b[" +
        match (self, next) {
            (Ident, Some(Brackets))
                => "34",
            (Ident | Brackets | Comma, _)
                => "0",
            (Integer, _)
                => "33",
            (Keyword | Operator, _)
                => "35",
            (Unused, _)
                => "90",
        } + "m" + text + "\x1b[0m"
    }
}