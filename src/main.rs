use std::collections::HashMap;
mod aga8;

fn main() {
	let mut composition: HashMap<u8, f64> = HashMap::new();
	composition.insert(1, 89.9403);
	composition.insert(2, 2.3947);
	composition.insert(3, 0.4880);
	composition.insert(4, 5.8860);
	composition.insert(5, 0.7937);
	composition.insert(6, 0.0000);
	composition.insert(7, 0.0000);
	composition.insert(8, 0.0000);
	composition.insert(9, 0.0000);
	composition.insert(10, 0.0000);
	composition.insert(11, 0.1153);
	composition.insert(12, 0.1596);
	composition.insert(13, 0.0502);
	composition.insert(14, 0.0606);
	composition.insert(15, 0.1116);
	composition.insert(16, 0.0000);
	composition.insert(17, 0.0000);
	composition.insert(18, 0.0000);
	composition.insert(19, 0.0000);
	composition.insert(20, 0.0000);
	let tk = 300.0;
	let pmp = 250000.0;
	aga8::detail(composition, tk, pmp);
}
