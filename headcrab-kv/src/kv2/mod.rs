use std::str::FromStr;

use logos::Logos;

mod array;
mod block;
mod comment;
mod key;
mod tokens;

pub use array::*;
pub use block::*;
pub use comment::*;
pub use key::*;

use tokens::*;

trait ArrayOrBlock: Send + Sync {
    fn as_any(&self) -> &dyn std::any::Any;
    fn push_array(&mut self, array: Array);
    fn push_block(&mut self, block: Block);
    fn push_key(&mut self, key: Key);
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub struct KV2Tree {
    pub comment: Option<Comment>,
    pub block: Block,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct ParsingError;

impl FromStr for KV2Tree {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lexer = KV2Token::lexer(s);
        let mut stack: Vec<Box<dyn ArrayOrBlock>> = vec![];
        let mut comment: Option<Comment> = None;
        let mut depth = 0;
        // this is so bad
        let mut in_array_before_before = false;
        let mut in_array_before = false;
        let mut in_array = false;
        loop {
            if let Some(token) = lexer.next() {
                if token.is_err() {
                    break;
                }

                let token = token.unwrap();
                println!("{:?} {}", token, stack.len());
                match token {
                    KV2Token::Array => {
                        stack.push(Box::new(Array {
                            name: lexer.slice()[1..lexer.slice().len() - 17].to_string(),
                            arrays: vec![],
                            blocks: vec![],
                            keys: vec![],
                        }));
                    }
                    KV2Token::Block => {
                        stack.push(Box::new(Block {
                            name: lexer.slice()[1..lexer.slice().len() - 1].to_string(),
                            arrays: vec![],
                            blocks: vec![],
                            keys: vec![],
                        }));
                    }
                    KV2Token::InitialComment => {
                        comment = Some(Comment(
                            lexer.slice()[4..lexer.slice().len() - 3].trim().to_string(),
                        ));
                    }
                    KV2Token::LeftBrace => {
                        depth += 1;
                        in_array_before_before = in_array_before;
                        in_array_before = in_array;
                        in_array = false;
                    }
                    KV2Token::LeftBracket => {
                        depth += 1;
                        in_array_before_before = in_array_before;
                        in_array_before = in_array;
                        in_array = false
                    }
                    KV2Token::PairWithType => {
                        let mut split = lexer.slice().split("\"");
                        split.next();
                        let key = split.next().unwrap();
                        split.next();
                        let key_type = split.next().unwrap();
                        split.next();
                        let value = split.next().unwrap();
                        let key = Key(key.to_string(), key_type.to_string(), value.to_string());
                        stack[depth - 1].push_key(key);
                    }
                    KV2Token::RightBrace => {
                        depth -= 1;
                        in_array_before = in_array_before_before;
                        in_array = in_array_before;
                        if depth > 0 {
                            let current = stack
                                .remove(depth)
                                .as_any()
                                .downcast_ref::<Block>()
                                .unwrap()
                                .to_owned();
                            stack[depth - 1].push_block(current);
                        }
                    }
                    KV2Token::RightBracket => {
                        depth -= 1;
                        in_array_before = in_array_before_before;
                        in_array = in_array_before;
                        if depth > 0 {
                            let current = stack
                                .remove(depth)
                                .as_any()
                                .downcast_ref::<Array>()
                                .unwrap()
                                .to_owned();
                            stack[depth - 1].push_array(current);
                        }
                    }
                }
            } else {
                break;
            }
        }

        Ok(KV2Tree {
            comment: comment,
            block: stack[0]
                .as_any()
                .downcast_ref::<Block>()
                .unwrap()
                .to_owned(),
        })
    }
}

impl ToString for KV2Tree {
    fn to_string(&self) -> String {
        let mut string = "".to_string();

        if let Some(comment) = &self.comment {
            string += comment.to_string().as_str();
        }

        for line in self.block.to_strings() {
            string += line.as_str();
        }

        string
    }
}
