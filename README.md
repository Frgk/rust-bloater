# rust-bloater #
This project is a bloatware programmed in Rust. It can be compiled to work on Windows.

## Disclaimer ##
I aimed to create this malware to improve my skills in Rust and learn about malawares behaviour. 
It's not designed to be harmful, but just a POC project to how it does work.

## Description ##
The malware check for specific folders to write to : **/temp**, **/tmp** or **/**.
Once a folder is found, it creates one bloat file inside (extension in **.bh**), either with a constant size, or a size proportional to the remaining capacity of the disk.
The file created is considered as hidden (hidden attribute in Windows).

By default, the malware is just an executable that create a file, that's all ! Depending on the features enabled when compiling the code, the malware can rename the created file by a random string, or add persistence to execute itself at startup.


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