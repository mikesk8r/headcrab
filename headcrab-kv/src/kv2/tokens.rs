use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
#[logos(skip "[[:space:]]+")]
pub enum KV2Token {
    #[regex("<!--[^<!-]+-->")]
    InitialComment,
    #[regex("\"[[:alnum:]]+\"")]
    Block,
    #[regex("\"[[:alnum:]]+\" \"element_array\"")]
    Array,
    #[regex("\"[^\"]+\"[ \t]+\"[^\"]+\"[ \t]+\"[^\"]+\"")]
    PairWithType,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
}
