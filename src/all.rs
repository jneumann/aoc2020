mod day1;
mod day2;

fn main() {
	let mains = [
		day1::main,
		day2::main,
	];

	for main in mains.iter() {
		main();
		println!();
	}
}
