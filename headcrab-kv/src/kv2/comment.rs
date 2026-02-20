#[derive(Debug, Eq, PartialEq)]
pub struct Comment(pub String);

impl ToString for Comment {
    fn to_string(&self) -> String {
        format!("<!-- {} -->\n", self.0)
    }
}
