use std::collections::HashMap;

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
pub fn chardl(ncc: u8, cid: HashMap<u8, f64>) -> (f64, f64) {
	let xi = get_xi(ncc, cid);
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
