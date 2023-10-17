use serde_json::{Result, Value};
use serde::Deserialize;
use std::fs::File;
use std::string::String;
use std::io::Read;
use std::collections::HashMap;

use crate::aga8::global;
use global::{Parameter, Unary, Binary};

fn read_table(table_num: u8) -> Value {
	let path: String = format!("/var/rustgas/aga_tables/AGA8/table{}.json", table_num);
	let mut file = File::open(path).unwrap();
	let mut string_json = String::new();
	file.read_to_string(&mut string_json);
	let table: Value = serde_json::from_str(&string_json).unwrap();
	return table;
}

fn get_table4() -> (Unary) {
	let table_4 = read_table(4);
	let mut a: Unary = HashMap::new();
	for cid in 1..=58 {
		let value = *&table_4["data"][cid.to_string()]["a"].clone().as_f64().unwrap();
		a.insert(cid, value);
	}
	return (a);
}

fn get_table5() -> (Unary, Unary, Unary, Unary, Unary, Unary, Unary, Unary) {
	let table_5 = read_table(5);
	let default: f64 = *&table_5["default_value"].clone().as_f64().unwrap();
	let mut qib: Unary = HashMap::new();
	let mut hib: Unary = HashMap::new();
	let mut rkib: Unary = HashMap::new();
	let mut eib: Unary = HashMap::new();
	let mut wib: Unary = HashMap::new();
	let mut cmwb: Unary = HashMap::new();
	let mut mib: Unary = HashMap::new();
	let mut dib: Unary = HashMap::new();
	let fetch = | cid: u8, parameter: &str | -> f64 {
		let index: String = cid.to_string();
		let value: f64 = *&table_5["data"][index][parameter].clone().as_f64().unwrap_or(default);
		return value;
	};
	for cid in 1..=21 {
		qib.insert(cid, fetch(cid, "Q"));
		hib.insert(cid, fetch(cid, "F"));
		rkib.insert(cid, fetch(cid, "K"));
		eib.insert(cid, fetch(cid, "E"));
		wib.insert(cid, fetch(cid, "G"));
		cmwb.insert(cid, fetch(cid, "M"));
		mib.insert(cid, fetch(cid, "S"));
		dib.insert(cid, fetch(cid, "W"));
	}
	return (qib, hib, rkib, eib, wib, cmwb, mib, dib);
}

fn get_table6() -> (Binary, Binary, Binary, Binary) {
	let table_6 = read_table(6);
	let default: f64 = *&table_6["default_value"].clone().as_f64().unwrap();
	let mut buijb: Binary = HashMap::new();
	let mut bkijb: Binary = HashMap::new();
	let mut beijb: Binary = HashMap::new();
	let mut bwijb: Binary = HashMap::new();
	let fetch = | i_num: u8, j_num: u8, parameter: &str | -> f64 {
		let i: String = i_num.to_string();
		let j: String = j_num.to_string();
		let value: f64 = *&table_6["data"][i][j][parameter].clone().as_f64().unwrap_or(default);
		return value;
	};
	for i in 1..=20 {
		for j in i+1..=21 {
			buijb.insert((i, j), fetch(i, j, "U"));
			bkijb.insert((i, j), fetch(i, j, "K"));
			beijb.insert((i, j), fetch(i, j, "E"));
			bwijb.insert((i, j), fetch(i, j, "G"));
		}
	}
	return (buijb, bkijb, beijb, bwijb);
}

pub fn blockdata() -> HashMap<&'static str, Parameter> {
	//	Equation of state parameters
	let (a) = get_table4();
	//	Individual Component Parameters
	let (qib, hib, rkib, eib, wib, cmwb, mib, dib) = get_table5();
	let (buij, bkijb, beijb, bwijb) = get_table6();
	let data: HashMap<&str, Parameter> = HashMap::from([
		("A", Parameter::Unary(a)),
		("QIB", Parameter::Unary(qib)),
		("HIB", Parameter::Unary(hib)),
		("RKIB", Parameter::Unary(rkib)),
		("EIB", Parameter::Unary(eib)),
		("WIB", Parameter::Unary(wib)),
		("CMWB", Parameter::Unary(cmwb)),
		("MIB", Parameter::Unary(mib)),
		("DIB", Parameter::Unary(dib)),
		("BUIJ", Parameter::Binary(buij)),
		("BKIJB", Parameter::Binary(bkijb)),
		("BEIJB", Parameter::Binary(beijb)),
		("BWIJB", Parameter::Binary(bwijb))
	]);
	return data;
}

