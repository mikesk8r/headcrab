/// A generic key with a string value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Key(pub String, pub String);

impl ToString for Key {
    fn to_string(&self) -> String {
        return format!(r#""{}" "{}""#, self.0, self.1);
    }
}
