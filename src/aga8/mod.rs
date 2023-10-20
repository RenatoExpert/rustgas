mod global;
mod blockdata;
mod paramdl;
mod chardl;

pub fn detail(cid: global::Unary, tk: f64, pmp: f64) {
	let block_data: global::ParameterSet = blockdata::blockdata();
	let params: global::ParameterSet = paramdl::paramdl(cid.clone(), block_data);
	let _chardl = chardl::chardl(cid.clone(), params.clone());
	dbg!(tk, pmp);
	/*
	let xi = cid.clone();
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

