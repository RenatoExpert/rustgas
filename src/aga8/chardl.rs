use std::collections::HashMap;
use crate::aga8::global;
use global::{Parameter, ParameterSet, Unary};

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
	let part_a = sum_a.powf(2.);
	let mut sum_b = 0.0;
	for i in 1..=ncc-1 {
		for j in i+1..=ncc {
			let ki = params["RKI"].capture_unary(i);
			let kj = params["RKI"].capture_unary(j);
			let kij = params["BKIJ"].capture_binary(i, j);
			sum_b += xi[&i] * xi[&j] * (kij.powf(5.) - 1.) * (ki * kj).powf(5./2.);
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
	let part_a: f64 = sum_a.powf(2.);
	let mut sum_b: f64 = 0.;
	for i in 1..=ncc-1 {
		for j in i+1..=ncc {
			let ei: f64 = params["EI"].capture_unary(i);
			let ej: f64 = params["EI"].capture_unary(j);
			let uij: f64 = params["BUIJ"].capture_binary(i, j);
			sum_b += xi[&i] * xi[&j] * (uij.powf(5.) - 1.) * (ei * ej).powf(5./2.);
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
		let qi: f64 = params["QI"].capture_unary(i);
		sum += xi[&i].powi(2) * qi;
	}
	let f: f64 = sum;
	return f;
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
	dbg!(mwx, k, u, g, q, f);
	return (0.0, 0.0);
}

