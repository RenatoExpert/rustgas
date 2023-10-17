use std::collections::HashMap;

pub type unary = HashMap<u8, f64>;
pub type binary = HashMap<(u8, u8), f64>;

#[derive(Debug, Clone)]
pub enum Parameter {
	Unary(unary),
	Binary(binary)
}

impl Parameter {
	pub fn capture_unary(&self, i: u8) -> f64 {
		match self {
			Parameter::Unary(params) => params[&i],
			Parameter::Binary(params) => panic!("capture_unary() not allowed to binary parameters!")
		}
	}
	pub fn capture_binary(&self, i: u8, j: u8) -> f64 {
		match self {
			Parameter::Unary(params) => panic!("capture_binary() not allowed to unary parameters!"),
			Parameter::Binary(params) => params[&(i, j)]
		}
	}
}

