#[test]
pub fn test_blocks() {
    let kv = r#"
        block1
        {
            "key1" "0"
            "key2" "string"
        }
        block2
        {
            "key1" "1"
            "key2" "different string"
        }
        "#
    .to_string();
    let tree = super::KVTree {
        blocks: vec![
            super::Block {
                name: "block1".to_string(),
                blocks: vec![],
                keys: vec![
                    super::Key("key1".to_string(), "0".to_string()),
                    super::Key("key2".to_string(), "string".to_string()),
                ],
            },
            super::Block {
                name: "block2".to_string(),
                blocks: vec![],
                keys: vec![
                    super::Key("key1".to_string(), "1".to_string()),
                    super::Key("key2".to_string(), "different string".to_string()),
                ],
            },
        ],
    };
    assert_eq!(super::parse(super::KVFormat::KV1, kv), tree);
}
