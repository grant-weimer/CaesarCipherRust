use std::env::args;
use std::fs;
use clap::Parser; //Library for parsing CLI arguments

//CLI Struct (objectlike), implementing constrains on inputs
#[derive(Parser)] //Kinda like inheritance
struct CLI {
    shift: u8, //unsigned 8 bit integer

    #[clap(parse(from_os_str))]
    
    path: std::path::PathBuf, //has the path as a PathBuf type
    //PathBuf is like string but for file system-paths that work cross platform
}

fn main() {
    let args = CLI::parse();

    // open file and read it's content
    let content = fs::read_to_string(&args.path).expect("can't read file");

    let mut buffer = Vec::new();
    for byte in content.bytes(){
        if(byte.is_ascii_whitespace()){
            buffer.push(byte);
        }
        else{
            buffer.push(byte + args.shift);
        }
    }

    //Convert charArr back to string
    let convertedString = String::from_utf8(buffer).expect("String conversion error");
    
    fs::write("output.txt", convertedString).expect("Write Error");
}
