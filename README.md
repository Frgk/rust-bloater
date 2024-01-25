# rust-bloater #
This project is a bloatware programmed in Rust. It can be compiled to work either on Windows or Linux.

## Description ##
The malware check for specific folders to write to : /temp, /tmp or /tmp
Once a folder is found, it creates one file inside, either with a constant size, or a size proportional to the remaining capacity of the disk.
The file created is considered as hidden (hidden attribute in Windows, '.' before the name of the bloat file in Linux).
Depending on the features enabled when compiling the code, the malware can rename the created file by a random string, or add persistence to execute itself at startup.


## Features ##
| Features | Description | 
| :---: | :---: |
| persistent | Add persistence to the bloatware, only working on Windows for now. It manipulates the registry to add a key inside HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run | 
| randomized | The bloat file has a random name.|

## Examples ##
You can compile the rust project to have the bloatware with the features you want
````
# Build the bloatware normally
cargo build --release

# Build with the features you want
cargo build --release -F feature1,feature2
cargo build --release --features feature1,feature2

# Examples: 
# Build with persistence function
cargo build --release -F persistent