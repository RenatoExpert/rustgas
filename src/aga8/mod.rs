use std::collections::HashMap;
//mod paramdl;
mod chardl;
mod blockdata;

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

pub fn detail(cid: HashMap<u8, f64>, tk: f64, pmp: f64) {
	let ncc = get_ncc(cid.clone());
	let blockdata = blockdata::blockdata();
	//let params = paramdl::paramdl(ncc, cid.clone());
	//dbg!(params);
	let xi = cid.clone();
	let (zb, db) = chardl::chardl(ncc, xi);
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

