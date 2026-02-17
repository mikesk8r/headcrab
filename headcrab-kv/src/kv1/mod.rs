use std::str::FromStr;

use logos::Logos;

mod block;
mod key;
mod tokens;

pub use block::*;
pub use key::*;

use tokens::*;

#[derive(Debug, Eq, PartialEq)]
pub struct KV1Tree {
    pub blocks: Vec<Block>,
}

#[derive(Debug, PartialEq)]
pub struct ParsingError;

impl FromStr for KV1Tree {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks: Vec<Block> = vec![];
        let mut lexer = KV1Token::lexer(s);
        let mut current_block: Vec<Block> = vec![];
        let mut depth = 0;
        loop {
            if let Some(token) = lexer.next() {
                if token.is_err() {
                    // needs proper error handling but whatever
                    break;
                }

                let token = token.unwrap();
                match token {
                    KV1Token::Block => current_block.push(Block {
                        name: lexer.slice().to_string(),
                        blocks: vec![],
                        keys: vec![],
                    }),
                    KV1Token::Pair => {
                        let mut split = lexer.slice().split("\"");
                        split.next();
                        let name = split.next().unwrap();
                        split.next();
                        let value = split.next().unwrap();
                        current_block[depth - 1]
                            .keys
                            .push(Key(name.to_string(), value.to_string()));
                    }
                    KV1Token::LeftBrace => {
                        depth += 1;
                    }
                    KV1Token::RightBrace => {
                        depth -= 1;
                        if depth > 0 {
                            let current = current_block[depth].clone();
                            current_block[depth - 1].blocks.push(current);
                        } else {
                            blocks.push(current_block[depth].clone());
                            current_block = vec![]
                        }
                    }
                }
            } else {
                break;
            }
        }

        Ok(KV1Tree { blocks: blocks })
    }
}
