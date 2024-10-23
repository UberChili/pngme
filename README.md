# pngme
Encode and decode hidden messages in PNG files.

## Introduction
As this is a personal project for learning purposes, it is not available to install as a crate on crates.io but you can clone and install it locally:
```bash
git clone https://github.com/UberChili/pngme.git
cd pngme
cargo build --release
```
Then you can run the executable via:
```bash
./target/release/pngme
```
Or you can install globally so you can run it from anywhere:
```bash
cargo install --path .
```
## Usage
Run without any arguments to get a list of possible commands:
```bash
pngme 
```
- Commands:
    - encode
    - decode
    - remove
    - print
    - help

Run a **command** without additional arguments to get a list of the needed options:
```bash
pngme <COMMAND>
# For example
pngme encode
```
### Encoding a message in a PNG file:
```bash
pngme encode --filepath [filename.png] --chunk-type rUsT --message "Hello, this is a very secret message!" --out-file [out_name.png]
```
### Decoding a message in a PNG file:
```bash
pngme decode --filepath [filename.png] --chunk-type rUsT
```
