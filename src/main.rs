#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]

use std::io::{stdin,stdout,BufWriter,BufReader,prelude::*};
use std::fs::File;
use std::num::ParseIntError;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = "Insert path to SRT file in the command line :\x1b[36m cargo run <Path to file>\x1b[0m\n\nExample:\n\n[user@DESKTOP C-SRT-FileParser]$\x1b[36m cargo run src/file.srt\x1b[0m";
    if args.len() < 2{
        println!("\n\x1b[31mFile not especified.\x1b[0m\n");
        
        println!("{}", usage);
    }else{
        let path = Path::new(&args[1]);
        if path.exists(){
            println!("\nOpenning File : {}\n",&args[1]);
            menu(&args[1]);
            //println!("Exists");
        }else{
            println!("{}", usage);
        }

        /*
        FILE * fp;
		if((fp = fopen(argv[1],"r")) == NULL){
			printf(ANSI_COLOR_RED "\nFile can't be openned or is invalid\n" ANSI_COLOR_RESET);
			printf("%s",usage);
		}else{
		fclose(fp);
		printf("\nOpenning File : %s\n",argv[1]);
		Menu(argv[1]);
		}
         */
        //println!("{:?}",&args[1]);
    }
    println!("\n\x1b[35mGoodbye\x1b[0m \u{1f984}");   
}

fn menu(IN_file:&str){
    let mut a_input = String::new();
    let mut option:i32;
    let pat: &[_] = &['\r','\n',' '];    
    loop{        
        stdout().flush().unwrap();
        print!("Choose an option: "); 
        stdout().flush().unwrap();    
        stdin().read_line(&mut a_input).unwrap();         
        option = a_input.trim_matches(pat).parse::<i32>().unwrap_or(0);        
        match option {
            1 => parse_file(IN_file),
            2 => print_file(IN_file),
            6 => break,
            _ => println!("Invalid"),    
        }
        a_input.clear();
    }    
}

fn parse_file(IN_file:&str){
    println!("opt parse");
}

fn print_file(files:&str){
    //println!("{:?}", std::env::current_exe());
    let path = Path::new(files);
    let display = path.display();
    //let mut file = File::open(&path).expect("Cant open file");
    let mut file = match File::open(&path){
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut reader = BufReader::new(file);
    for line in reader.lines(){
        println!("{}", line.unwrap());
    }
}
