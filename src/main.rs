fn main() {
	let multiples:Vec<(u64, &str)> = vec!((3, "Fizz"), (5, "Buzz"));
	let mut output;
	let mut count = 0;

	for multiple in &multiples {
		count += multiple.1.len();
	}

	output = String::with_capacity(count);

	for i in 1..101 {
		for (num, out) in &multiples {
			if i%num == 0 {
				output.push_str(out);
			}
		}
		if output.eq("") {
			println!("{}", i);
		} else {
			println!("{}", output);
		}
		output.clear();
	}
}

