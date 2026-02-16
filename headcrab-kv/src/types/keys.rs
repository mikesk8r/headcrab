#[derive(Clone, Eq, PartialEq)]
pub struct Key {
    pub name: String,
    pub value: String,
}

impl Key {
    pub fn as_string(&self) -> String {
        return format!(r#""{}" "{}""#, self.name, self.value);
    }
}
