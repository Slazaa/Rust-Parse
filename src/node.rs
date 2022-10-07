use crate::Token;

pub trait ASTNode {
	fn new_token(token: &Token) -> Self;
	fn is_token(&self) -> bool;
	fn token(&self) -> Result<&Token, String>;
}
