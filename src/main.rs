extern crate rand;
extern crate num_bigint;
extern crate num_traits;

use rand::Rng;
use num_bigint::{BigInt, ToBigInt};
use num_traits::{One, Zero};
use std::io;

// Function to compute the greatest common divisor
fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
	if *b == BigInt::zero() {
		a.clone()
	} else {
		gcd(b, &(a % b))
	}
}


fn main() {
	let mut rng = rand::thread_rng();
	
	// Set p and q to be two large primes such that p \u{2261} 3 (mod 4) and q \u{2261} 3 (mod 4)
	let p = BigInt::parse_bytes(b"32212254719", 10).unwrap(); // Replace with actual large prime
	let q = BigInt::parse_bytes(b"21474836483", 10).unwrap(); // Replace with actual large prime
	let n = &p * &q;
	
	// Ensure p \u{2261} 3 (mod 4) and q \u{2261} 3 (mod 4)
	assert!((&p % 4.to_bigint().unwrap()) == 3_i32.to_bigint().unwrap());
	assert!((&q % 4.to_bigint().unwrap()) == 3_i32.to_bigint().unwrap());
	
	// Assume x is the square root of PIN modulo n
	let x = BigInt::parse_bytes(b"17", 10).unwrap(); // Replace with actual secret square root
	let pin = (&x * &x) % &n;
	
	println!("TTP setup:");
	println!("n = {}", n);
	println!("PIN = {}", pin);
	
	println!("Card reader has n = {}", n);
	
	// Smart Card or the card holder Cyprian enters the PIN number into the card reader
	println!("Enter PIN: ");
	let mut pin_input = String::new();
	io::stdin().read_line(&mut pin_input).expect("Failed to read line");
	let pin_input = pin_input.trim().parse::<BigInt>().expect("Invalid input");
	
	if pin_input != pin {
		println!("Incorrect PIN");
		return;
	}
	
	// Steps 4-7 repeated for demonstration
	for _ in 0..5 {
		// Step 4: Card/Cyprian creates a random r, calculates t \u{2261} r^2 mod n, and transmits it to Alex
		let mut r;
		loop {
			r = BigInt::from(rng.gen::<u64>()) % &n;
			if gcd(&r, &n) == BigInt::one() {
				break;
			}
		}
		let t = (&r * &r) % &n;
		println!("Card/Cyprian sends t = {}", t);
		
		// Step 5: The Card Reader randomly selects e \u{2208} {0, 1} and sends it to Cyprian
		let e: u32 = rng.gen_range(0..=1);
		println!("Card reader selects e = {}", e);
		
		// Step 6: Card/Cyprian calculates u  \u{2261} r\u{00B7}x^e mod n and sends it to the card reader
		let x_e = x.modpow(&BigInt::from(e), &n);
		let u = (&r * &x_e) % &n;
		println!("Card/Cyprian sends u = {}", u);
		
		// Step 7: The Card Reader verifies u^2  \u{2261} t\u{00B7}PIN^e mod n
		let u_squared = (&u * &u) % &n;
		let pin_e = pin.modpow(&BigInt::from(e), &n);
		let verification = (&t * &pin_e) % &n;
		println!("Verification: {}^2 mod {} = {}\u{00B7} t\u{00B7}PIN^e mod {} = {}", u, n, u_squared, n, verification);
		
		if u_squared == verification {
			println!("Verification successful.");
		} else {
			println!("Verification failed.");
			return;
		}
	}
	
	println!("Card reader is convinced that the card holds the correct square root of the PIN.");
}