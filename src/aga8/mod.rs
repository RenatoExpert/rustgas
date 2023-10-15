use std::collections::HashMap;

use serde::Deserialize;
use std::fs::File;
use std::io::Read;
const json_table4: &str = include_str!("../aga_tables/AGA8/table4.json");
dbg!(json_table4);

fn get_ncc(cid: HashMap<u8, f64>) -> u8 {
	let mut ncc: u8 = 0;
	for component in 1..21 {
		if cid[&component] > 0.0 {
			ncc += 1;
		} else {
			break;
		}
	}
	return ncc;
}

fn get_xi(ncc: u8, cid: HashMap<u8, f64>) -> HashMap<u8, f64> {
	let mut tmfrac: f64 = 0.0;
	for j in 1..ncc {
		tmfrac += cid[&j];
	}
	let mut xi: HashMap<u8, f64> = HashMap::new();
	for j in 1..ncc {
		let percentual = cid[&j] / tmfrac;
		xi.insert(j, percentual);
	}
	return xi;
}

//	Returns compressibility and density for base state (zb, db)
fn chardl(ncc: u8, cid: HashMap<u8, f64>) -> (f64, f64) {
	let xi = get_xi(ncc, cid);
	let mut mwx: f64 = 0.0;
	//	Get M from table 5
	//let cmw
	/*
	for j in 1..ncc {
		mwx += xi[&j] * table_5[&j]['M'];
	}
	*/
	dbg!(xi);
	return (0.0, 0.0);
}

pub fn detail(cid: HashMap<u8, f64>, tk: f64, pmp: f64) {
	let ncc = get_ncc(cid.clone());
	dbg!(ncc);
	let xi = cid.clone();
	let (zb, db) = chardl(ncc, xi);
	dbg!(zb, db);
	/*
	temp(tk);
	d = ddetail(pmp, tk);
	dbg!(d);
	z = zdetail(d, tk);
	dbg!(z);
	fpv = dsqrt(zb/z);
	dbg!(fpv);
	*/
}

