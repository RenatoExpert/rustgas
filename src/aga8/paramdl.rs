use std::collections::HashMap;

use crate::aga8::global;

fn get_ncc(cid: HashMap<u8, f64>) -> u8 {
	let mut ncc: u8 = 0;
	for component in 1..=21 {
		if cid[&component] > 0.0 {
			ncc += 1;
		} else {
			break;
		}
	}
	return ncc;
}

pub fn paramdl(cid: HashMap<u8, f64>, block_data: HashMap<&str, global::Parameter>) {
	let ncc: u8 = get_ncc(cid);
	let told: f64 = 0.0;
	let rgas: f64 = 8.31451e-3;
	let tlow: f64 = 0.0;
	let thigh: f64 = 10000.0;
	let plow: f64 = 0.5e-9;
	let phigh: f64 = 275.0;
	let dhigh: f64 = 12.0;
	//	Constants from block data
}

