
pub fn ddetail(p: f64, t: f64) -> f64 {
	let imax = 150;
	let epsp = 1.0e-6;
	let epsr = 1.0e-6;
	let epsmin = 1.0e-7;
	let code = 0;
	braket(code, t, p, rho, rhol, rhoh, prhol, prhoh);
	return d;
}

