use std::collections::HashMap;

pub type Unary = HashMap<u8, f64>;
pub type Binary = HashMap<(u8, u8), f64>;

#[derive(Debug, Clone)]
pub enum Parameter {
	Unary(Unary),
	Binary(Binary)
}

impl Parameter {
	pub fn capture_unary(&self, i: u8) -> f64 {
		match self {
			Parameter::Unary(params) => params[&i],
			Parameter::Binary(_params) => panic!("capture_unary() not allowed to binary parameters!")
		}
	}
	pub fn capture_binary(&self, i: u8, j: u8) -> f64 {
		match self {
			Parameter::Unary(_params) => panic!("capture_binary() not allowed to unary parameters!"),
			Parameter::Binary(params) => params[&(i, j)]
		}
	}
}

pub type ParameterSet = HashMap<&'static str, Parameter>;
