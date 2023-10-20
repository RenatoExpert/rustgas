use std::collections::HashMap;

pub type Unary = HashMap<u8, f64>;
pub type Binary = HashMap<(u8, u8), f64>;
pub type Ternary = HashMap<(u8, u8, u8), f64>;

#[derive(Debug, Clone)]
pub enum Parameter {
	Counter(u8),
	Attribute(f64),
	Unary(Unary),
	Binary(Binary),
	Ternary(Ternary)
}

impl Parameter {
	pub fn unwrap_attribute(&self) -> f64 {
		match self {
			Parameter::Attribute(value) => return *value,
			_ => panic!("variable is not a counter")
		}
	}
	pub fn unwrap_counter(&self) -> u8 {
		match self {
			Parameter::Counter(counter) => return *counter,
			_ => panic!("variable is not a counter")
		}
	}
	pub fn capture_unary(&self, i: u8) -> f64 {
		match self {
			Parameter::Unary(params) => params[&i],
			_ => panic!("Datatype not allowed to use capture_unary!")
		}
	}
	pub fn capture_binary(&self, i: u8, j: u8) -> f64 {
		match self {
			Parameter::Binary(params) => {
				if let Some(&value) = params.get(&(i, j)) {
					value
				} else {
					dbg!(&self, i, j);
					panic!("key not found for binary parameters");
				}
			},
			_ => panic!("Datatype not allowed to use capture_binary!")
		}
	}
	pub fn capture_ternary(&self, n: u8, i: u8, j: u8) -> f64 {
		match self {
			Parameter::Ternary(params) => {
				if let Some(&value) = params.get(&(n, i, j)) {
					value
				} else {
					dbg!(&self, n, i, j);
					panic!("key not found for ternary parameters");
				}
			},
			_ => panic!("Datatype not allowed to use capture_binary!")
		}
	}
}

pub type ParameterSet = HashMap<&'static str, Parameter>;

