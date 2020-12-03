mod day1;
mod day2;
mod day3;

fn main() {
	let mains = [
		day1::main,
		day2::main,
		day3::main,
	];

	for main in mains.iter() {
		main();
		println!();
	}
}
