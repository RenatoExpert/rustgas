use serde_json::{Result, Value};
use serde::Deserialize;
use std::fs::File;
use std::string::String;
use std::io::Read;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Table {
    aga_num: u8,
    year: u32,
    table_num: u8,
    default_value: f64,
    data: HashMap<String, DataEntry>
}

#[derive(Clone, Debug)]
struct DataEntry {
    a: f64,
    b: u8,
    u: Option<f64> // Use Option<f64> for fields that are optional in the JSON
}

fn read_table(table_num: u8) -> Value {
	let path: String = format!("/var/rustgas/aga_tables/AGA8/table{}.json", table_num);
	let mut file = File::open(path).unwrap();
	let mut string_json = String::new();
	file.read_to_string(&mut string_json);
	let table: Value = serde_json::from_str(&string_json).unwrap();
	return table;
}

pub fn blockdata() {
	//	Equation of state parameters
	let table_4 = read_table(4);
	/*
	let A get A parameters from table 4
	*/
	let table_5 = read_table(5);
	let table_6 = read_table(6);
}

