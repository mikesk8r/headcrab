mod tokens;
mod types;

use types::*;

pub struct KVTree {
    pub blocks: Vec<Block>,
}

pub enum KVFormat {
    KV1,
    // KV2,
    // KV3
}

pub fn parse(kvtype: KVFormat, text: String) {
    match kvtype {
        KVFormat::KV1 => {
            // TODO...
        }
    }
}
