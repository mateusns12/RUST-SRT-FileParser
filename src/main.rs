#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]

use std::io::{stdin,stdout,BufWriter,BufReader,prelude::*};
use std::fs::File;
use std::num::ParseIntError;
use std::path::Path;
use std::env;

struct Time{   
    hour:i32,
    mins:i32,
    secs:i32,
    mill:i32
}

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
            println!("\n\x1b[31mFile can't be openned or is invalid\x1b[0m\n");
            println!("{}", usage);
        }
    }
    println!("\n\x1b[35mGoodbye\x1b[0m \u{1f984}");   
}

fn menu(in_file:&str){
    let mut a_input = String::new();
    let mut option:u8;
    let pat: &[_] = &['\r','\n',' '];    
    let mut shift:i32;
    loop{        
        stdout().flush().unwrap();
        print!("Choose an Option :\n\n\t1 - Parse in seconds\n\t2 - Parse in milliseconds\n\t3 - Print File\n\t6 - Exit\n"); 
        stdout().flush().unwrap();    
        stdin().read_line(&mut a_input).unwrap();         
        option = a_input.trim_matches(pat).parse::<u8>().unwrap_or(0);        
        match option {
            1 => {
                stdout().flush().unwrap();
                print!("How many seconds to shift ? ");
                stdout().flush().unwrap();                 
                a_input.clear();
                stdin().read_line(&mut a_input).unwrap(); 
                shift = a_input.trim_matches(pat).parse::<i32>().unwrap_or(0);                
                parse_file(in_file,shift, option);
            },
            2 => {
                stdout().flush().unwrap();
                print!("How many milliseconds to shift ? ");
                stdout().flush().unwrap();
                a_input.clear();
                stdin().read_line(&mut a_input).unwrap(); 
                shift = a_input.trim_matches(pat).parse::<i32>().unwrap_or(0);  
                parse_file(in_file,shift, option);
            }
            3 => {
                print_file(in_file,option);
            },            
            6 => break,
            _ => println!("Invalid"),    
        }
        a_input.clear();
    }    
}

fn print_file(files:&str,option:u8){
    let path = Path::new(files);
    let display = path.display();
    let mut file = match File::open(&path){
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut reader = BufReader::new(file);
    for line in reader.lines(){
        println!("{}", line.unwrap());
    }
}

fn parse_file(in_file:&str,shift:i32,option:u8){
    let out_path = Path::new("outfile.srt");
    let out_display = out_path.display();
    let mut out_file = match File::create(&out_path) {
        Err(error) => panic!("couldn't create {}: {}", out_display, error),
        Ok(out_file) => out_file,
    };

    let in_path = Path::new(in_file);
    let in_display = in_path.display();
    let mut in_file = match File::open(&in_path){
        Err(error) => panic!("couldn't open {}: {}", in_display, error),
        Ok(in_file) => in_file,
    };

    let mut writer = BufWriter::new(out_file);
    let mut reader = BufReader::new(in_file);
    println!("\n\tCreating Outfile.srt...\n");
    for line in reader.lines(){
        let mut buffer = String::from(line.unwrap());
        handle_time(&mut buffer,shift,option);
        write!(writer,"{}\n",buffer).unwrap();
    }
    println!("\x1b[32m\tOutfile.srt created.\n\x1b[0m");
}

fn handle_time(buffer:&mut String,shift:i32,option:u8){
    let mut time_right:Time = Time { mill: 0, secs: 0, mins: 0, hour: 0 }; 
    let mut time_left:Time = Time { mill: 0, secs: 0, mins: 0, hour: 0 };
    
    if get_timestamp(buffer, &mut time_right,&mut time_left){
        change_time(&mut time_right,shift,option);
        change_time(&mut time_left,shift,option);
        new_buffer(buffer, &mut time_right, &mut time_left);
    }
}

fn get_timestamp(buffer:&str,time_right:&mut Time,time_left:&mut Time) -> bool{
    let mut iter:Vec<&str> = buffer.split([':',',',' ']).collect();  
    //println!("{:?}",iter);    
    if iter.len() != 9{ return false }  
    let mut hour = iter[0].parse::<i32>().unwrap_or(-1);
    let mut mins = iter[1].parse::<i32>().unwrap_or(-1);
    let mut secs = iter[2].parse::<i32>().unwrap_or(-1);
    let mut mill = iter[3].parse::<i32>().unwrap_or(-1);    
    if hour == -1{
        return false
    }else{
        time_left.hour = hour; time_left.mins = mins; 
        time_left.secs = secs; time_left.mill = mill;
    }
    hour = iter[5].parse::<i32>().unwrap_or(-1);
    mins = iter[6].parse::<i32>().unwrap_or(-1);
    secs = iter[7].parse::<i32>().unwrap_or(-1);
    mill = iter[8].parse::<i32>().unwrap_or(-1);
    if hour == -1{
        return false
    }else{
        time_right.hour = hour; time_right.mins = mins; 
        time_right.secs = secs; time_right.mill = mill;
    }
    return true;
}

fn new_buffer(buffer:&mut String,time_right:&mut Time,time_left:&mut Time){
    let mut new_buffer = String::new();
    new_buffer = format!("{:02}:{:02}:{:02},{:03} --> {:02}:{:02}:{:02},{:03}"
                    ,time_left.hour,time_left.mins
                    ,time_left.secs,time_left.mill
                    ,time_right.hour,time_right.mins
                    ,time_right.secs,time_right.mill);
    buffer.clear();
    buffer.push_str(&new_buffer);
}

fn change_time(time:&mut Time, shift_opt:i32,option:u8){
    let mut shift:i32 = 0;
    if option == 2 {
        let shift_ms = shift_opt;
		shift = shift_opt/1000;
		time.mill += shift_ms;
		time.secs += time.mill/1000;
        time.mill = time.mill%1000;
    }else{
        time.secs += shift_opt%60;
    }
    //time.secs += shift%60 ;
    time.mins += shift/60 + time.secs/60;
    time.hour += time.mins/60;
    time.secs = time.secs%60;
    time.mins = time.mins%60;

    if time.mill < 0{
        time.mill += 1000;
        time.secs -= 1;
    }
    if time.secs < 0 {
        time.secs += 60;
        time.mins -= 1;
    }
    if time.mins < 0 {
        time.mins += 60;
        time.hour -= 1;
    }
    if time.hour < 0 {
        time.hour += 24;
    }
}