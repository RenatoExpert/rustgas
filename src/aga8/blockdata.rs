use serde_json::{Result, Value};
use serde::Deserialize;
use std::fs::File;
use std::string::String;
use std::io::Read;
use std::collections::HashMap;

fn read_table(table_num: u8) -> Value {
	let path: String = format!("/var/rustgas/aga_tables/AGA8/table{}.json", table_num);
	let mut file = File::open(path).unwrap();
	let mut string_json = String::new();
	file.read_to_string(&mut string_json);
	let table: Value = serde_json::from_str(&string_json).unwrap();
	return table;
}

fn get_table4() -> (HashMap<u8, f64>) {
	let table_4 = read_table(4);
	let mut A: HashMap<u8, f64> = HashMap::new();
	for cid in 1..59 {
		let value = *&table_4["data"][cid.to_string()]["a"].clone().as_f64().unwrap();
		A.insert(cid, value);
	}
	return A;
}

fn get_table5() -> (HashMap<u8, f64>, HashMap<u8, f64>) {
	let table_5 = read_table(5);
	let default: f64 = *&table_5["default_value"].clone().as_f64().unwrap();
	let mut QIB: HashMap<u8, f64> = HashMap::new();
	let mut HIB: HashMap<u8, f64> = HashMap::new();
	let fetch = | cid: u8, parameter: &str | -> f64 {
		let index: String = cid.to_string();
		let value: f64 = *&table_5["data"][index][parameter].clone().as_f64().unwrap_or(default);
		return value;
	};
	for cid in 1..22 {
		QIB.insert(cid, fetch(cid, "Q"));
		HIB.insert(cid, fetch(cid, "F"));
	}
	return (QIB, HIB);
}


pub fn blockdata() {
	//	Equation of state parameters
	let (A) = get_table4();
	//	Individual Component Parameters
	let (QIB, HIB) = get_table5();
	dbg!(QIB, HIB);
	/*
	let A get A parameters from table 4
	*/
	let table_6 = read_table(6);
}

