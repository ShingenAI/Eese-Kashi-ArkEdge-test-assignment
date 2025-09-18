use std::env;
use std::fs::File;
use std::io::{self, Read};

pub fn packet_parser(path: &str) -> io::Result<()> {
// Set the path to the sample binary file manually or via CLI args
let args: Vec<String> = env::args().collect();
if args.len() != 2 {
    eprintln!("Usage: {} <binary_file_path>", args[0]);
    std::process::exit(1);
}

// Open and read the entire binary file into a buffer
let mut file = File::open(path)?;
let mut buffer = Vec::new();
file.read_to_end(&mut buffer)?;


// Define protocol constants
const STX: u16 = 0xEB90;
const ETX: u16 = 0xC579;
const MIN_PACKET_SIZE: usize = 6; // STX (2) + LENGTH (2) + ETX (2)


let mut i = 0;
while i + MIN_PACKET_SIZE <= buffer.len() {
// Try reading STX (big-endian)
let stx = u16::from_be_bytes([buffer[i], buffer[i + 1]]);
if stx != STX {
i += 1;
continue;
}


// Read LENGTH (big-endian)
let length = u16::from_be_bytes([buffer[i + 2], buffer[i + 3]]) as usize;


// Check if enough bytes remain
let total_packet_size = 2 + 2 + length + 2;
if i + total_packet_size > buffer.len() {
break; // Not enough data left for full packet
}


// Read ETX
let etx_index = i + 4 + length;
let etx = u16::from_be_bytes([buffer[etx_index], buffer[etx_index + 1]]);
if etx != ETX {
i += 1;
continue;
}


// Valid packet found, extract BODY!
let body = &buffer[i + 4..i + 4 + length];
for byte in body {
print!("0x{:02x} ", byte);
}
println!();


// Move to the next potential packet
i += total_packet_size;
}


Ok(())
}