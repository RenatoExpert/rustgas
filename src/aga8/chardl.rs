use std::collections::HashMap;
use crate::aga8::global;
use global::{Parameter, ParameterSet, Unary};

//	Get mole fractions
fn get_mole_fractions(ncc: u8, cid: Unary) -> Unary {
	let mut tmfrac: f64 = 0.0;
	for j in 1..ncc {
		tmfrac += cid[&j];
	}
	let mut xi: Unary = HashMap::new();
	for j in 1..ncc {
		let percentual = cid[&j] / tmfrac;
		xi.insert(j, percentual);
	}
	return xi;
}

//	Returns compressibility and density for base state (zb, db)
pub fn chardl(cid: Unary, params: ParameterSet) -> (f64, f64) {
	let ncc: u8 = params["NCC"].unwrap_counter();
	let xi: Unary = get_mole_fractions(ncc, cid);
	let mut mwx: f64 = 0.0;
	//	Get M from table 5
	//let cmw
	/*
	for j in 1..ncc {
		mwx += xi[&j] * table_5[&j]['M'];
	}
	*/
	return (0.0, 0.0);
}
