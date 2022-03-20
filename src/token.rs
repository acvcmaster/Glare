#[derive(Debug)]
pub enum Token {
    String(String),
    Number(i64),
}