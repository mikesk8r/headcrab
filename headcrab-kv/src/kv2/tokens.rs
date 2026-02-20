use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
#[logos(skip "[[:space:]]+")]
pub enum KV2Token {
    #[regex("<!--[^<!-]+-->")]
    InitialComment,
    #[regex("\"[^\"]+\" \"element_array\"")]
    Array,
    #[regex("\"[^\"]+\"")]
    Block,
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
