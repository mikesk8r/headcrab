use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
#[logos(skip "[[:space:]]+")]
pub enum KV1Token {
    #[regex("[[:alnum:]]+")]
    Block,
    #[regex("\"[^\"]+\"[ \t]+\"[^\"]+\"")]
    Pair,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
}
