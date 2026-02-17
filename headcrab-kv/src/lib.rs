use logos::Logos;

#[cfg(test)]
mod tests;
mod tokens;
mod types;

use tokens::*;
use types::*;

#[derive(Debug, Eq, PartialEq)]
pub struct KVTree {
    pub blocks: Vec<Block>,
}

pub enum KVFormat {
    KV1,
    // KV2,
    // KV3
}

pub fn parse(kvtype: KVFormat, mut text: String) -> KVTree {
    let mut blocks: Vec<Block> = vec![];
    match kvtype {
        KVFormat::KV1 => {
            let mut lexer = KV1Token::lexer(text.as_mut_str());
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
        }
    }

    KVTree { blocks: blocks }
}
