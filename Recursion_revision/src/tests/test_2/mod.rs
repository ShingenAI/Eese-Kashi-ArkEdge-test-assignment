use std::fs::File;
use std::io::{self, Read};

pub fn packet_parser(path: &str) -> io::Result<()> {
    println!("\n::::::: Packet parser started :::::::\n");
    println!("Path: {}\n", path);
    // Open and read the binary file into a buffer
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Define protocol constants
    const STX: u16 = 0xEB90;
    const ETX: u16 = 0xC579;
    const MIN_PACKET_SIZE: usize = 6; // STX (2) + LENGTH (2) + ETX (2)

    let mut i = 0;
    while i + MIN_PACKET_SIZE <= buffer.len() {
        // Try reading STX (start-of-packet) in big-endian
        let stx = u16::from_be_bytes([buffer[i], buffer[i + 1]]);
        if stx != STX {
            i += 1;
            continue;
        }
        println!("Found STX at byte offset {}", i);

        // Read LENGTH field (big-endian)
        let length = u16::from_be_bytes([buffer[i + 2], buffer[i + 3]]) as usize;
        println!("Packet length = {} bytes", length);

        // Check if enough data remains for full packet
        let total_packet_size = 2 + 2 + length + 2;
        if i + total_packet_size > buffer.len() {
            println!("  ⚠️ Incomplete packet: not enough bytes remaining. Aborting parse.");
            break;
        }

        // 🔚 Read and validate ETX (end-of-packet)
        let etx_index = i + 4 + length;
        let etx = u16::from_be_bytes([buffer[etx_index], buffer[etx_index + 1]]);
        if etx != ETX {
            println!("  ❌ Invalid ETX at byte offset {}. Skipping this region.", etx_index);
            i += 1;
            continue;
        }

        // Valid packet found — extract and print BODY
        println!("  ✅ Valid packet! BODY contents:");
        let body = &buffer[i + 4..i + 4 + length];
        for byte in body {
            print!("0x{:02x} ", byte);
        }
        println!();

        // Move to the next potential packet
        i += total_packet_size;
    }
    println!("\n::::::: Packet parser ended :::::::\n");
    Ok(())
}