/// An array containing keys or sub-blocks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Array {
    pub name: String,
    pub arrays: Vec<Array>,
    pub blocks: Vec<super::Block>,
    pub keys: Vec<super::Key>,
}

impl Array {
    pub fn to_strings(&self) -> Vec<String> {
        let mut block_string = vec![];

        block_string.push(format!("\"{}\" \"element_array\"\n", self.name));
        block_string.push("[\n".to_string());

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

        block_string.push("]\n".to_string());

        block_string
    }
}
