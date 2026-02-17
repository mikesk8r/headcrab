#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    pub name: String,
    pub blocks: Vec<Block>,
    pub keys: Vec<super::Key>,
}

impl Block {
    pub fn as_string(&self) -> String {
        let mut block_string = String::new();

        block_string += format!("{} {{", self.name).as_str();

        for key in self.keys.clone() {
            block_string += format!("\t{}\n", key.as_string()).as_str();
        }

        for block in self.blocks.clone() {
            block_string += &block.as_string();
        }

        block_string += "}}";

        block_string
    }
}
