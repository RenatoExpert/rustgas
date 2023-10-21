use crate::aga8::global::ParameterSet;

fn zdetail(d: f64, t: f64, bmix: f64, params: ParameterSet, non_temp: ParameterSet) -> f64 {
	let k = non_temp.get("K").unwrap().clone().unwrap_attribute();
	//	Needs bmix, k, table4
	let z: f64;
	let dbig: f64 = k.powi(3) * d;
	let part1: f64 = dbig * bmix / k.powi(3);
	let mut sum_2: f64 = 0.0;
	let cnast_param = non_temp.get("Cnast").unwrap();
	dbg!(cnast_param);
	for n in 13..=18 {
		let un: f64 = params["U"].capture_unary(n); 
		let cnast = non_temp.get("Cnast").unwrap().clone().capture_unary(n);
		sum_2 += cnast * t.powf(-un);
	}
	let part2: f64 = dbig * sum_2;
	let mut sum_3: f64 = 0.0;
	for n in 13..=58 {
		let cnast = non_temp.get("Cnast").unwrap().clone().capture_unary(n);
		let bn: f64 = params["B"].capture_unary(n); 
		let cn: f64 = params["C"].capture_unary(n); 
		let kn: f64 = params["K"].capture_unary(n); 
		let un: f64 = params["U"].capture_unary(n); 
		let part1: f64 = cnast * t.powf(-un);
		let part2: f64 = bn - (cn * kn * dbig.powf(kn));
		let part3: f64 = dbig.powf(bn);
		let part4: f64 = (-cn * dbig.powf(kn)).exp(); 
		sum_3 += part1 * part2 * part3 * part4;
	}
	let part3: f64 = sum_3;
	z = 1.0 + part1 - part2 + part3;
	dbg!(z);
	return z;
}

fn pdetail(d: f64, t: f64, bmix: f64, params: ParameterSet, non_temp: ParameterSet) -> f64 {
	let z: f64 = zdetail(d, t, bmix, params.clone(), non_temp.clone());
	let r = params.get("RGAS").unwrap().clone().unwrap_attribute();
	let pressure: f64 = z * d * r * t;
	dbg!(pressure);
	return pressure;
}

fn braket(t: f64, p: f64, bmix: f64, params: ParameterSet, non_temp: ParameterSet) -> (f64, f64, f64, f64) {
	let k = non_temp.get("K").unwrap().clone().unwrap_attribute();
	let u = non_temp.get("U").unwrap().clone().unwrap_attribute();
	let rgas = params.get("RGAS").unwrap().clone().unwrap_attribute();
	dbg!(k, rgas);
	let (rho, rhol, rhoh, prhol, prhoh): (f64, f64, f64, f64, f64);
	let mut rho1 = 0.0;
	let mut rho2;
	let mut p1 = 0.0;
	let mut rhomax = 1.0 / k;
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
	let imax = 200;
	let mut code = 0;
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
		let p2 = pdetail(rho2, t, bmix, params.clone(), non_temp.clone());
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

pub fn ddetail(p: f64, t: f64, bmix: f64, non_temp: ParameterSet, params: ParameterSet) -> f64 {
	let imax = 150;
	let epsp = 1.0e-6;
	let epsr = 1.0e-6;
	let epsmin = 1.0e-7;
	let code = 0;
	//	removed rho
	let (rhol, rhoh, prhol, prhoh) = braket(t, p, bmix, params.clone(), non_temp.clone());
	//	There is more code here
	let d = 0.0;
	return d;
}

