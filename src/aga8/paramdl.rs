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
	dbg!(ncc);
}

