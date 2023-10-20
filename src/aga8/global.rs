use std::collections::HashMap;

pub type Unary = HashMap<u8, f64>;
pub type Binary = HashMap<(u8, u8), f64>;
pub type Ternary = HashMap<(u8, u8, u8), f64>;

#[derive(Debug, Clone)]
pub enum Parameter {
	Counter(u8),
	Attribute(f64),
	Unary(Unary),
	Binary(Binary)
}

impl Parameter {
	pub fn unwrap_counter(&self) -> u8 {
		match self {
			Parameter::Counter(counter) => return *counter,
			_ => panic!("Variable is not a Counter")
		}
	}
	pub fn capture_unary(&self, i: u8) -> f64 {
		match self {
			Parameter::Counter(_params) => panic!("capture_unary() not allowed to be used on Counters!"),
			Parameter::Attribute(_params) => panic!("capture_unary() not allowed to be used on Attributes!"),
			Parameter::Unary(params) => params[&i],
			Parameter::Binary(_params) => panic!("capture_unary() not allowed to be used on binary parameters!")
		}
	}
	pub fn capture_binary(&self, i: u8, j: u8) -> f64 {
		match self {
			Parameter::Counter(_params) => panic!("capture_binary() not allowed to be used on Counters!"),
			Parameter::Attribute(_params) => panic!("capture_binary() not allowed to be used on Attributes!"),
			Parameter::Unary(_params) => panic!("capture_binary() not allowed to be used on unary parameters!"),
			Parameter::Binary(params) => {
				if let Some(&value) = params.get(&(i, j)) {
					value
				} else {
					dbg!(&self, i, j);
					panic!("key not found for binary parameters");
				}
			}
		}
	}
}

pub type ParameterSet = HashMap<&'static str, Parameter>;

