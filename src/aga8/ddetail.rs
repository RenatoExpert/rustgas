use crate::aga8::global::ParameterSet;

fn pdetail(rho: f64, t: f64) -> f64 {
	let pressure = 0.0;
	return pressure;
}

fn braket(t: f64, p: f64, k: f64, u: f64, rgas: f64) -> (f64, f64, f64, f64) {
	let (code, rho, rhol, rhoh, prhol, prhoh): (u8, f64, f64, f64, f64, f64);
	let rho1 = 0.0;
	let mut rho2;
	let p1 = 0.0;
	let rhomax = 1.0 / k;
	if t > 1.2593 * u {
		rhomax = 20.0 * rhomax
	}
	let videal = rgas * t / p;
	if bmix.abs() < 0.167 * videal {
		rho2 = 0.95 / (videal * bmix);
	} else {
		rho2 = 1.15 / videal;
	}
	//	Note: pressure (p2) at density rho2 not yet calculated
	let mut del = rho2 / 2.0;
	let code = 0;
	let imax = 200;
	for it in 1..=imax {
		if code != 2 && rho2 > rhomax {
			//	Density in braket exceeds maximum allowable density
			code = 2;
			dbg!("Density in braket exceeded/ Default density used");
			let del = 0.01 * (rhomax - rho1) + p/(rgas * t)/20.0;
			rho2 = rho1 + del;
			continue;
		}
		//	Calculate pressure P2 at density RHO2
		let p2 = pdetail(rho2, t);
		//	Test value of P2 relative to P and relative to P1
		if p2 > p {
			//	The density of root is bracketed (P1<P and P2>P)
			rhol = rho1;
			prhol = p1;
			rhoh = rho2;
			prhoh = p2;
			return (rhol, rhoh, prhol, prhoh) 
		} else if p2 > p1 && code == 2 {
			//	Retain constant value for del (CODE=2)
			rho1 = rho2;
			p1 = p2;
			rho2 = rho1 + del;
			continue;
		} else if p2 > p1 && code == 0 {
			//	Increase value for del (CODE = 0)
			del = 2.0 * del;
			rho1 = rho2;
			p1 = p2;
			rho2 = rho1 + del;
			continue;
		} else {
			panic!("Code 1: Derivative of pressure with respect to density is negative");
		}
	}
	//	Maximum number of iterations exceeded
	panic!("Code 3: Maximum number of iterations exceeded");
}

pub fn ddetail(p: f64, t: f64, non_temp: ParameterSet) -> f64 {
	let imax = 150;
	let epsp = 1.0e-6;
	let epsr = 1.0e-6;
	let epsmin = 1.0e-7;
	let code = 0;
	//	removed rho
	//	let k = non_temp.get("K").get_attribute();
	//	let rgas = non_temp.get("RGAS").get_attribute();
	//(rhol, rhoh, prhol, prhoh) = braket(t, p, k, u, rgas);
	//	There is more code here
	let d = 0.0;
	return d;
}

