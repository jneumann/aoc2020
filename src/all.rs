mod day1;

fn main() {
	let mains = [
		day1::main,
	];

	for (day, main) in mains.iter().enumerate() {
		main();
		println!();
	}
}
