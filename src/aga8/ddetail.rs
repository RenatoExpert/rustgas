fn braket() {
	let code = 0;
	let imax = 200;
	let rho1 = 0.0;
	let p1 = 0.0;
	let rhomax = 1.0/rk3p0 ???;
	if T > 1.2593 * UU ??? {
		rhomax = 20 * rhomax
	}
	videal = rgas * t / p;
	if bmix.abs() < 0.167 * videal) {
		rho2 = 0.95.0 / (videal * bmix);
	} else {
		rho2 = 1.15 / videal;
	}
	//	Note: pressure (p2) at density rho2 not yet calculated
	let del = rho2 / 2.0;
	let it = 0
	//	Start iterative density search loop
	let it += 1;
	if it > imax {
		//	Maximum number of iterations exceeded
		code = 3;
		panic!("Maximum number of iterations exceeded");
		rho = rho2;
		return
	}
	if code != 2 && rho2 > rhomax {
		//	Density in braket exceeds maximum allowable density
		code = 2;
		panic!("Density in braket exceeded/ Default density used");
		let del = 0.01 * (rhomax - rho1) + p/(rgas*)/20.0;
		rho2 = rho1 + del;
		goto it += 1 ...
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
		return;
	} else if p2 > p1 && code == 2 {
		//	Retain constant value for del (CODE=2)
		rho1 = rho2;
		p1 = p2;
		rho2 = rho1 + del;
		goto it;
	} else if p2 > p1 && code == 0 {
		//	Increase value for del (CODE = 0)
		del = 2 * del;
		rho1 = rho2;
		p1 = p2;
		rho2 = rho1 + del;
		goto it;
	}
	//	Above if/elseif conditions unsatisfied implies p2 < p1
	//	Code = 1 indicates that pressure has a negative density
	//	Derivative, since p2 is less than some previous pressure
	code = 1;
	panic!("Code 1: Derivative of pressure with respect to");
	panic!("Density is negative / System may contain liquid");
	panic!("Default gas density used");
	rho = rho1
	return;
	
}

pub fn ddetail(p: f64, t: f64) -> f64 {
	let imax = 150;
	let epsp = 1.0e-6;
	let epsr = 1.0e-6;
	let epsmin = 1.0e-7;
	let code = 0;
	braket(code, t, p, rho, rhol, rhoh, prhol, prhoh);
	return d;
}

