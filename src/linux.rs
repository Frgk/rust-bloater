// Create the file when compiled on Linux system
fn create_bloatfile(path: &str, name:String, data: Vec<u8>) -> String{	

	let path_file: String = format!("{}.{}.bh",path,name);

	// Create an hidden file (linux only)
	let mut file = File::create(&path_file);
	let _ = file.write_all(&data);

	return path_file

}

// Adding persistence for the linux compilation
fn adding_persistence(filepath: &str, exe_name: &str) -> std::io::Result<()>{
	Ok(())	
}