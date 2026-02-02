# Installation

## Prerequisites
- Install **Rust** (https://www.rust-lang.org/tools/install)

## Install

```bash
git clone <this-repo-url>
cd <repo-name>
cargo install --path .
```
This will build the project in release mode and install the binary into your Cargo bin directory (usually ~/.cargo/bin).

Make sure that directory is in your $PATH, most of the time it is.


## Usage
```
fq ${FASTQ/FASTQ.GZ}
```
```
fq  ~/Downloads/test.fastq
Total reads : 25
Total bases : 28421
Mean length : 1136
Median lens : 813
GC%         : 44.00%
```




