

fn my_bindings(){
	let x = 1;
	
	println!("Normal binding {}", x);
	
	// Create mutable binding
	
	let mut mut_x = 2;
	
	println!("Mutable binding {}", mut_x);
	
	mut_x = 3;
	
	println!("Mutable binding {}", mut_x);
	
	//Shadow a variable in a block
	{
		println!("Shadow binding {}", mut_x);
		let mut_x = 4;
		println!("Shadow binding {}", mut_x);
	}
	println!("Shadow binding outside block {}", mut_x);
}