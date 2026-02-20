#![cfg(test)]

use std::str::FromStr;

#[test]
pub fn kv1_from_str() {
    use super::kv1::{Block, KV1Tree, Key};

    const STRING: &'static str = r#"
        block1 {
            "key1" "value1"
        }
        block2 {
            "key2" "value2"
        }
    "#;

    let tree = KV1Tree {
        blocks: vec![
            Block {
                name: "block1".to_string(),
                blocks: vec![],
                keys: vec![Key("key1".to_string(), "value1".to_string())],
            },
            Block {
                name: "block2".to_string(),
                blocks: vec![],
                keys: vec![Key("key2".to_string(), "value2".to_string())],
            },
        ],
    };

    assert_eq!(KV1Tree::from_str(STRING).unwrap(), tree)
}

#[test]
pub fn kv1_to_str() {
    use super::kv1::{Block, KV1Tree, Key};

    const STRING: &'static str =
        "block1\n{\n\t\"key1\" \"value1\"\n}\nblock2\n{\n\t\"key2\" \"value2\"\n}\n";

    let tree = KV1Tree {
        blocks: vec![
            Block {
                name: "block1".to_string(),
                blocks: vec![],
                keys: vec![Key("key1".to_string(), "value1".to_string())],
            },
            Block {
                name: "block2".to_string(),
                blocks: vec![],
                keys: vec![Key("key2".to_string(), "value2".to_string())],
            },
        ],
    };

    assert_eq!(tree.to_string(), STRING)
}

#[test]
fn kv2_from_str() {
    use super::kv2::{Array, Block, Comment, KV2Tree, Key};

    const STRING: &'static str = r#"
        <!-- this is a test comment -->
        "block"
        {
            "key1" "type" "value1"
            "array" "element_array"
            [
            ]
            "subblock"
            {
            }
        }
    "#;

    assert_eq!(
        KV2Tree::from_str(STRING).unwrap(),
        KV2Tree {
            comment: Some(Comment("this is a test comment".to_string())),
            block: Block {
                name: "block".to_string(),
                keys: vec![Key(
                    "key1".to_string(),
                    "type".to_string(),
                    "value1".to_string()
                )],
                arrays: vec![Array {
                    name: "array".to_string(),
                    keys: vec![],
                    blocks: vec![],
                    arrays: vec![]
                }],
                blocks: vec![Block {
                    name: "subblock".to_string(),
                    keys: vec![],
                    blocks: vec![],
                    arrays: vec![],
                }],
            }
        }
    )
}

#[test]
pub fn kv2_to_str() {
    use super::kv2::{Array, Block, Comment, KV2Tree, Key};

    const STRING: &'static str = "<!-- this is a test comment -->\n\"block\"\n{\n\t\"key1\" \"type\" \"value1\"\n\t\"array\" \"element_array\"\n\t[\n\t]\n\t\"subblock\"\n\t{\n\t}\n}\n";

    let tree = KV2Tree {
        comment: Some(Comment("this is a test comment".to_string())),
        block: Block {
            name: "block".to_string(),
            arrays: vec![Array {
                name: "array".to_string(),
                arrays: vec![],
                blocks: vec![],
                keys: vec![],
            }],
            blocks: vec![Block {
                name: "subblock".to_string(),
                arrays: vec![],
                blocks: vec![],
                keys: vec![],
            }],
            keys: vec![Key(
                "key1".to_string(),
                "type".to_string(),
                "value1".to_string(),
            )],
        },
    };

    assert_eq!(tree.to_string(), STRING)
}
