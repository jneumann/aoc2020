mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
	let mains = [
		day1::main,
		day2::main,
		day3::main,
		day4::main,
		day5::main,
	];

	for main in mains.iter() {
		main();
		println!();
	}
}
