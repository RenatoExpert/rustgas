use std::collections::HashMap;

pub type unary = HashMap<u8, f64>;
pub type binary = HashMap<(u8, u8), f64>;

#[derive(Debug)]
pub enum Parameter {
	Unary(unary),
	Binary(binary)
}
