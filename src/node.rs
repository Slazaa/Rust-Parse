use crate::Token;

pub trait ASTNode {
	fn token(token: &Token) -> Self;
	fn get_token(&self) -> Result<&Token, String>;
}