# rust-bloater #
This project is a bloatware programmed in Rust. 

## Description ##
Its aim is to fill the principal disk of a computer by creating a file in the temp folder.
This file has the size of the remaining available space.

I added options to the compilation of the executable to make the bloatware more or less personnalizable (by adding features). However, an executable should be available in the releases.

## Features ##
| persistent | Add persistence to the bloatware, only working on Windows for now. | 
| :---: | :---: |
| randomized | The bloat file has random name. If the feature 'persistent' is enabled, also generate a random name for the new executable. | 
| :---: | :---: |

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