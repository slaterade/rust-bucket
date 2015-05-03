fn main() {
	for i in 2..100 {
		let mut is_prime = true;
		for j in 2..(i/2)+1 {
			if i % j == 0 {	
				is_prime = false;
				break;
			}
		}
		if is_prime {
			println!("{} is prime.", i);
		}
	}
}
