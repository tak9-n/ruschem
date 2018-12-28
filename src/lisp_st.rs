#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LispAst {
    Identifier(String),
    True,
    False,
    String(String),
}
