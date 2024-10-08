use std::collections::HashMap;

use crate::aga8::global::{Unary, Binary, ParameterSet, Parameter};

fn get_ncc(cid: HashMap<u8, f64>) -> u8 {
	let mut ncc: u8 = 0;
	let mut count: bool = false;
	for reverse in 1..=21 {
		let component: u8 = 22 - reverse;
		if cid[&component] > 0.0 {
			count = true;
		}
		if count {
			ncc += 1;
		}
	}
	dbg!(ncc);
	return ncc;
}

fn get_constants(block_data: ParameterSet, ncc: u8) -> (Unary, Unary, Unary, Unary, Unary, Unary, Unary, Unary) {
	let mut cmw: Unary = HashMap::new();
	let mut rki: Unary = HashMap::new();
	let mut ei: Unary = HashMap::new();
	let mut wi: Unary = HashMap::new();
	let mut qi: Unary = HashMap::new();
	let mut hi: Unary = HashMap::new();
	let mut mi: Unary = HashMap::new();
	let mut di: Unary = HashMap::new();
	let extract = |param, j| -> f64 {
		return block_data.get(param).unwrap().capture_unary(j);
	};
	for j in 1..=ncc {
		cmw.insert(j, extract("CMWB", j));
		rki.insert(j, extract("RKIB", j));
		ei.insert(j, extract("EIB", j));
		wi.insert(j, extract("WIB", j));
		qi.insert(j, extract("QIB", j));
		hi.insert(j, extract("HIB", j));
		mi.insert(j, extract("MIB", j));
		di.insert(j, extract("DIB", j));
	}
	return (cmw, rki, ei, wi, qi, hi, mi, di);
}

fn get_interaction(block_data: ParameterSet, ncc: u8) -> (Binary, Binary, Binary, Binary) {
	let mut beij: Binary = HashMap::new();
	let mut bkij: Binary = HashMap::new();
	let mut bwij: Binary = HashMap::new();
	let mut buij: Binary = HashMap::new();
	let extract = |param, j, k| -> f64 {
		return block_data.get(param).unwrap().capture_binary(j, k);
	};
	for j in 1..=ncc {
		for k in 1..=ncc {
			beij.insert((j, k), extract("BEIJB", j, k));
			bkij.insert((j, k), extract("BKIJB", j, k));
			bwij.insert((j, k), extract("BWIJB", j, k));
			buij.insert((j, k), extract("BUIJB", j, k));
		}
	}
	return (beij, bkij, bwij, buij);
}

pub fn paramdl(cid: HashMap<u8, f64>, block_data: ParameterSet) -> ParameterSet {
	let ncc: u8 = get_ncc(cid);
	let told: f64 = 0.0;
	let rgas: f64 = 8.31451e-3;
	let tlow: f64 = 0.0;
	let thigh: f64 = 10000.0;
	let plow: f64 = 0.5e-9;
	let phigh: f64 = 275.0;
	let dhigh: f64 = 12.0;
	//	Constants from block data
	let (cmw, rki, ei, wi, qi, hi, mi, di) = get_constants(block_data.clone(), ncc);
	//	Binary parameters
	let (beij, bkij, bwij, buij) = get_interaction(block_data.clone(), ncc);
	let data: ParameterSet = HashMap::from([
		("A", block_data.get("A").unwrap().clone()),
		("B", block_data.get("B").unwrap().clone()),
		("C", block_data.get("C").unwrap().clone()),
		("K", block_data.get("K").unwrap().clone()),
		("U", block_data.get("U").unwrap().clone()),
		("G", block_data.get("G").unwrap().clone()),
		("Q", block_data.get("Q").unwrap().clone()),
		("F", block_data.get("F").unwrap().clone()),
		("S", block_data.get("S").unwrap().clone()),
		("W", block_data.get("W").unwrap().clone()),
		("NCC", Parameter::Counter(ncc)),
		("TOLD", Parameter::Attribute(told)),
		("TOLD", Parameter::Attribute(told)),
		("RGAS", Parameter::Attribute(rgas)),
		("TLOW", Parameter::Attribute(tlow)),
		("THIGH", Parameter::Attribute(thigh)),
		("PLOW", Parameter::Attribute(plow)),
		("PHIGH", Parameter::Attribute(phigh)),
		("DHIGH", Parameter::Attribute(dhigh)),
		("QI", Parameter::Unary(qi)),
		("HI", Parameter::Unary(hi)),
		("RKI", Parameter::Unary(rki)),
		("EI", Parameter::Unary(ei)),
		("WI", Parameter::Unary(wi)),
		("CMW", Parameter::Unary(cmw)),
		("MI", Parameter::Unary(mi)),
		("DI", Parameter::Unary(di)),
		("BUIJ", Parameter::Binary(buij)),
		("BKIJ", Parameter::Binary(bkij)),
		("BEIJ", Parameter::Binary(beij)),
		("BWIJ", Parameter::Binary(bwij))
	]);
	return data;
}

