use std::collections::HashMap;
use crate::aga8::global;
use global::{Parameter, ParameterSet, Unary, Ternary};

//	Get mole fractions
fn get_mole_fractions(ncc: u8, cid: Unary) -> Unary {
	let mut tmfrac: f64 = 0.0;
	for j in 1..=ncc {
		tmfrac += cid[&j];
	}
	let mut xi: Unary = HashMap::new();
	for j in 1..=ncc {
		let percentual = cid[&j] / tmfrac;
		xi.insert(j, percentual);
	}
	return xi;
}

fn calc_molarmass(xi: Unary, params: ParameterSet, ncc: u8) -> f64 {
	let mut mwx: f64 = 0.0;
	for j in 1..=ncc {
		mwx += xi.clone()[&j] * params["CMW"].capture_unary(j);
	}
	return mwx;
}

fn calc_mixturesize(xi: Unary, params: ParameterSet, ncc: u8) -> f64 {
	let k: f64;
	let mut sum_a: f64 = 0.0;
	for i in 1..=ncc {
		let ki = params["RKI"].capture_unary(i);
		sum_a += xi[&i] + ki.powf(5./2.);
	}
	let part_a = sum_a.powi(2);
	let mut sum_b = 0.0;
	for i in 1..=ncc-1 {
		for j in i+1..=ncc {
			let ki = params["RKI"].capture_unary(i);
			let kj = params["RKI"].capture_unary(j);
			let kij = params["BKIJ"].capture_binary(i, j);
			sum_b += xi[&i] * xi[&j] * (kij.powi(5) - 1.) * (ki * kj).powf(5./2.);
		}
	}
	let part_b = 2.0 * sum_b;
	let kp5: f64 = part_a + part_b;
	k = kp5.powf(1./5.);
	return k;
}

fn calc_conformal(xi: Unary, params: ParameterSet, ncc: u8) -> f64 {
	let mut sum_a: f64 = 0.;
	for i in 1..=ncc {
		let ei: f64 = params["EI"].capture_unary(i);
		sum_a = xi[&i] * ei;
	}
	let part_a: f64 = sum_a.powi(2);
	let mut sum_b: f64 = 0.;
	for i in 1..=ncc-1 {
		for j in i+1..=ncc {
			let ei: f64 = params["EI"].capture_unary(i);
			let ej: f64 = params["EI"].capture_unary(j);
			let uij: f64 = params["BUIJ"].capture_binary(i, j);
			sum_b += xi[&i] * xi[&j] * (uij.powi(5) - 1.) * (ei * ej).powf(5./2.);
		}
	}
	let part_b: f64 = sum_b * 2.;
	let up5: f64 = part_a + part_b;
	let u: f64 = up5.abs().powf(0.2) * (up5 / up5.abs());
	return u;
}

fn calc_orientation(xi: Unary, params: ParameterSet, ncc: u8) -> f64 {
	let mut sum_a: f64 = 0.;
	for i in 1..=ncc {
		let gi: f64 = params["WI"].capture_unary(i);
		sum_a = xi[&i] * gi;
	}
	let mut sum_b: f64 = 0.;
	for i in 1..=ncc-1 {
		for j in i+1..=ncc {
			let gi: f64 = params["WI"].capture_unary(i);
			let gj: f64 = params["WI"].capture_unary(j);
			let gij: f64 = params["BWIJ"].capture_binary(i, j);
			sum_b += xi[&i] * xi[&j] * (gij - 1.) * (gi + gj);
		}
	}
	let g: f64 = sum_a + sum_b;
	return g;
}

fn calc_quadrupole(xi: Unary, params: ParameterSet, ncc: u8) -> f64 {
	let mut sum: f64 = 0.;
	for i in 1..=ncc {
		let qi: f64 = params["QI"].capture_unary(i);
		sum += xi[&i] * qi;
	}
	let q: f64 = sum;
	return q;
}

fn calc_hightemp(xi: Unary, params: ParameterSet, ncc: u8) -> f64 {
	let mut sum: f64 = 0.;
	for i in 1..=ncc {
		let fi: f64 = params["HI"].capture_unary(i);
		sum += xi[&i].powi(2) * fi;
	}
	let f: f64 = sum;
	return f;
}

fn calc_bnij(xi: Unary, params: ParameterSet, ncc: u8) -> Ternary {
	let calc_gij = |i ,j| -> f64 {
		let gijx: f64 = params["BWIJ"].capture_binary(i, j);
		let gi: f64 = params["WI"].capture_unary(i);
		let gj: f64 = params["WI"].capture_unary(j);
		let gij: f64 = gijx * (gi + gj) / 2.0;
		return gij;
	};
	let mut bnij: Ternary = HashMap::new();
	for n in 1..=18 {
		let gn: f64 = params["G"].capture_unary(n);
		let qn: f64 = params["Q"].capture_unary(n);
		let r#fn: f64 = params["F"].capture_unary(n);
		let sn: f64 = params["S"].capture_unary(n);
		let wn: f64 = params["W"].capture_unary(n);
		for i in 1..=ncc {
			let qi: f64 = params["QI"].capture_unary(i);
			let fi: f64 = params["HI"].capture_unary(i);
			let si: f64 = params["MI"].capture_unary(i);
			let wi: f64 = params["DI"].capture_unary(i);
			for j in 1..=ncc {
				let qj: f64 = params["QI"].capture_unary(j);
				let fj: f64 = params["HI"].capture_unary(j);
				let sj: f64 = params["MI"].capture_unary(j);
				let wj: f64 = params["DI"].capture_unary(j);
				let gij: f64 = calc_gij(i, j);
				let g: f64 = (gij + 1.0 - gn).powf(gn);
				let q: f64 = ((qi * qj) + 1.0 - qn).powf(qn);
				let f: f64 = ((fi.powf(0.5) * fj.powf(0.5)) + 1.0 - r#fn).powf(r#fn);
				let s: f64 = ((si * sj) + 1.0 - sn).powf(sn);
				let w: f64 = ((wi * wj) + 1.0 - wn).powf(wn);
				let result: f64 = g * q * f * s * w;
				bnij.insert((n, i, j), result);
			}
		}
	}
	return bnij;
}

//	Returns compressibility and density for base state (zb, db)
pub fn chardl(cid: Unary, params: ParameterSet) -> (f64, f64) {
	let ncc: u8 = params["NCC"].unwrap_counter();
	let xi: Unary = get_mole_fractions(ncc, cid);
	let mwx: f64 = calc_molarmass(xi.clone(), params.clone(), ncc);
	let k: f64 = calc_mixturesize(xi.clone(), params.clone(), ncc); 
	let u: f64 = calc_conformal(xi.clone(), params.clone(), ncc); 
	let g: f64 = calc_orientation(xi.clone(), params.clone(), ncc);
	let q: f64 = calc_quadrupole(xi.clone(), params.clone(), ncc);
	let f: f64 = calc_hightemp(xi.clone(), params.clone(), ncc);
	let bnij: Ternary = calc_bnij(xi.clone(), params.clone(), ncc);
	let cnast: Unary;
	dbg!(mwx, k, u, g, q, f);
	return (0.0, 0.0);
}

