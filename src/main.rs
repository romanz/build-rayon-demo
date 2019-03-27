use rayon::prelude::*;

fn main() {
	(0..10).into_par_iter().for_each(|i| println!("Hello, {} world!", i));
}
