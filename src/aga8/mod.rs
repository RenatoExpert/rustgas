//	Generate composition from hash
use std::collections::HashMap;
fn generate_mixture(composition: HashMap<u8, f64>) -> HashMap<u8, u8> {
	let mut mixture: HashMap<u8, u8> = HashMap::new();
	let mut n = 1;
	for (cid, percent) in composition {
		if percent > 0.0 {
			mixture.insert(n, cid);
			n += 1;
		}
	}
	return mixture;
}
/*
	composition -> CID: Percent
	mixture -> n: CID
*/

//	=====================================================================
pub fn detail(composition: HashMap<u8, f64>, tk: f64, pmp: f64) {
	println!("Hello");
	/*
	paramdl(ncc, cid);
	chardl(ncc, xi, zb, db);
	temp(tk);
	d = ddetail(pmp, tk);
	dbg!(d);
	z = zdetail(d, tk);
	dbg!(z);
	fpv = dsqrt(zb/z);
	dbg!(fpv);
	*/
}

