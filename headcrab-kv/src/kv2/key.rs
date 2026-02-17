/// A generic key with a string value and type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Key(pub String, pub String, pub String);

impl ToString for Key {
    fn to_string(&self) -> String {
        return format!(r#""{}" "{}" "{}""#, self.0, self.1, self.2);
    }
}
