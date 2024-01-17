use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rand::Rng;
use sysinfo::{
    Components, Disks, Networks, System,
};

fn main() -> std::io::Result<()> {

	// Search for a valid path to write to
	let mut valid_path:&str = "";
	let path_array = [r"C:/temp/", r"C:/tmp/", r"/tmp/"];

	for path in path_array {
		if Path::new(path).is_dir() {
			valid_path = path;
			println!("Valid path : {}", path);
			break;
		}
	}
   
	let mut rng = rand::thread_rng();
	/*
	// Create the files
	for _ in 1..2{

		// Generate a random numbers
		let n2:u64 = rng.gen();
		// print!("Random : {}",n2);

		// 1Mo =  1_048_576
		// 16Mo = 16_777_216
		// 128Mo = 134_217_728

		// Vec with specified size (number of elements in the vector)
		let encoded: Vec<u8> = vec![0;1_073_741_824];

		// Create the file and write into it
		let mut file = File::create(format!("{}{}.bh",valid_path,n2))?;
		file.write_all(&encoded);

	}

	*/
let disks = Disks::new_with_refreshed_list();
for disk in &disks {
    println!("Total space : {}, Available space : {}", disk.total_space(), disk.available_space());
}

	Ok(())
}
