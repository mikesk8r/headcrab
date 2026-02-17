use std::str::FromStr;

mod array;
mod block;
mod comment;
mod key;
mod tokens;

pub use array::*;
pub use block::*;
pub use comment::*;
pub use key::*;

#[derive(Debug, Eq, PartialEq)]
pub struct KV2Tree {
    pub comment: Option<Comment>,
    pub block: Block,
}

#[derive(Debug, PartialEq)]
pub struct ParsingError;

impl FromStr for KV2Tree {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // todo
        Ok(KV2Tree {
            comment: None,
            block: Block {
                name: "".to_string(),
                arrays: vec![],
                blocks: vec![],
                keys: vec![],
            },
        })
    }
}
