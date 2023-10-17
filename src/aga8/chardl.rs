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
		let Ki = params["RKI"].capture_unary(i);
		sum_a += xi[&i] + Ki.powf(5./2.);
	}
	let part_a = sum_a.powf(2.);
	let mut sum_b = 0.0;
	for i in 1..=ncc-1 {
		for j in i+1..=ncc {
			let Ki = params["RKI"].capture_unary(i);
			let Kj = params["RKI"].capture_unary(j);
			let Kij = params["BKIJ"].capture_binary(i, j);
			sum_b += xi[&i] * xi[&j] * (Kij.powf(5.) - 1.) * (Ki * Kj).powf(5./2.);
		}
	}
	let part_b = 2.0 * sum_b;
	let kp5: f64 = part_a + part_b;
	k = kp5.powf(1./5.);
	return k;
}

//	Returns compressibility and density for base state (zb, db)
pub fn chardl(cid: Unary, params: ParameterSet) -> (f64, f64) {
	let ncc: u8 = params["NCC"].unwrap_counter();
	let xi: Unary = get_mole_fractions(ncc, cid);
	let mwx: f64 = calc_molarmass(xi.clone(), params.clone(), ncc);
	let k: f64 = calc_mixturesize(xi.clone(), params.clone(), ncc); 
	dbg!(k);
	return (0.0, 0.0);
}

