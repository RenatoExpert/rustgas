use std::collections::HashMap;
mod global;
mod blockdata;
mod paramdl;
//mod chardl;

pub fn detail(cid: HashMap<u8, f64>, tk: f64, pmp: f64) {
	let block_data: HashMap<&'static str, global::Parameter> = blockdata::blockdata();
	let params = paramdl::paramdl(cid, block_data);
	//dbg!(params);
	/*
	let xi = cid.clone();
	let (zb, db) = chardl::chardl(ncc, xi);
	dbg!(zb, db);
	temp(tk);
	d = ddetail(pmp, tk);
	dbg!(d);
	z = zdetail(d, tk);
	dbg!(z);
	fpv = dsqrt(zb/z);
	dbg!(fpv);
	*/
}

