/// A block containing keys and optionally sub-blocks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    pub name: String,
    pub arrays: Vec<super::Array>,
    pub blocks: Vec<Block>,
    pub keys: Vec<super::Key>,
}

impl super::ArrayOrBlock for Block {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn push_array(&mut self, array: super::Array) {
        self.arrays.push(array)
    }

    fn push_block(&mut self, block: super::Block) {
        self.blocks.push(block)
    }

    fn push_key(&mut self, key: super::Key) {
        self.keys.push(key)
    }
}

impl Block {
    pub fn to_strings(&self) -> Vec<String> {
        let mut block_string = vec![];

        block_string.push(format!("\"{}\"\n", self.name));
        block_string.push("{\n".to_string());

        for key in self.keys.clone() {
            block_string.push(format!("\t{}\n", key.to_string()));
        }

        for array in self.arrays.clone() {
            for string in array.to_strings() {
                block_string.push(format!("\t{}", string));
            }
        }

        for block in self.blocks.clone() {
            for string in block.to_strings() {
                block_string.push(format!("\t{}", string));
            }
        }

        block_string.push("}\n".to_string());

        block_string
    }
}
