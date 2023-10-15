use std::collections::HashMap;
mod aga8;

fn main() {
	//	Using High N2 from table A-51 - AGA 8 - Page 83
	let mut composition: HashMap<u8, f64> = HashMap::new();
	composition.insert(1, 81.4410);
	composition.insert(2, 13.4650);
	composition.insert(3, 0.9850);
	composition.insert(4, 3.3000);
	composition.insert(5, 0.6050);
	composition.insert(6, 0.1000);
	composition.insert(7, 0.1040);
	composition.insert(8, 0.0000);
	composition.insert(9, 0.0000);
	composition.insert(10, 0.0000);
	composition.insert(11, 0.0000);
	composition.insert(12, 0.0000);
	composition.insert(13, 0.0000);
	composition.insert(14, 0.0000);
	composition.insert(15, 0.0000);
	composition.insert(16, 0.0000);
	composition.insert(17, 0.0000);
	composition.insert(18, 0.0000);
	composition.insert(19, 0.0000);
	composition.insert(20, 0.0000);
	let tk = 300.0;
	let pmp = 250000.0;
	aga8::detail(composition, tk, pmp);
}
