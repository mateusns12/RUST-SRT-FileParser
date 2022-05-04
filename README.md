# SRT file parser in RUST

![Language](https://img.shields.io/badge/Language-RUST-critical?style=for-the-badge&logo=rust)
![System](https://img.shields.io/badge/System-Arch_WSL2-A100FF?style=for-the-badge&logo=windows)

<!--
![Status](https://img.shields.io/badge/status-concluded-87D935?style=for-the-badge)
-->

> **This program is a RUST re-implementation of the [SRT File Parser in C](https://github.com/mateusns12/C-SRT-FileParser)**

SRT File parser in RUST. This program gets a srt file as a command line argument. The main purpose is to be able to change the time of the subtitles, entering a new value in seconds or in milliseconds.

Currently, the option 1 - "Parse in seconds" shifts the time in seconds, while creating a new file "outfile.srt", with the updated time. The option 2 - "Parse in milliseconds" shifts the time in milliseconds. Option 3 - "Print File" just prints the source file, and 6 - "Exit" leaves the program.

# Usage

After building the executable "rust-files" with cargo build, the target file is inserted by passing as a reference in the command line:

````
// On Linux, passing the sample file in the src folder

[user@DESKTOP RUST-SRT-FileParser]$ cargo run src/IronMan.srt
````
## Menu

````
Openning File : src/IronMan.srt

Choose an Option:
        1 - Parse in seconds
        2 - Parse in milliseconds
        3 - Print File
        6 - Exit
````
## Parse

````
1  // Chosen option

How many seconds to shift ? : 59

        Creating Outfile.srt...

        Outfile.srt created.

====================================================
2  // Chosen option

How many milliseconds to shift ? : 59000

        Creating Outfile.srt...

        Outfile.srt created.
````
## Comparison IN and OUT files

````
// Input File

897
00:59:10,365 --> 00:59:12,591
No. No, absolutely not.
It'll give me a bone to throw the boys

====================================================
// Output File

897
01:00:09,365 --> 01:00:11,591
No. No, absolutely not.
It'll give me a bone to throw the boys
````
# Notes 

- Direct reimplementation of the C version, changing just what is needed.

# To-do
- [ ] Change C idiomatics to RUST
- [ ] Implement generator