use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn test() -> std::io::Result<()> {
    // Connect to the Minecraft server
    let mut stream = TcpStream::connect("127.0.0.1:9191")?;

    // Send the 0xFE byte to query the server
    stream.write(&[0xFE])?;

    // Read the response
    let mut buffer = [0; 512];
    let mut data = Vec::new();

    while let Ok(size) = stream.read(&mut buffer) {
        if size == 0 {
            break;
        }
        for &byte in &buffer[..size] {
            // Filter out unwanted characters (as per original logic)
            if byte != 0 && byte > 16 && byte != 255 && byte != 23 && byte != 24 {
                data.push(byte);
                // println!("{}:{}", byte, byte as char);
            }
        }
    }

    if data.len() == 0 {
        println!("Server not running");
        return Ok(());
    }
    // Split the data into three parts based on ASCII 167 (which is '§')
    let parts: Vec<Vec<u8>> = data.split(|&x| x == 167).map(|x| x.to_vec()).collect();

    // Check if we have at least 3 parts and convert them to strings

    if parts.len() >= 3 {
        let part1 = str::from_utf8(&parts[0]).unwrap_or_default();
        let part2 = str::from_utf8(&parts[1]).unwrap_or_default();
        let part3 = str::from_utf8(&parts[2]).unwrap_or_default();

        // Print the three strings
        println!("MOTD: {}", part1);
        println!("Active Players: {}", part2);
        println!("Max Players: {}", part3);
    } else {
        println!("Failed to parse server response into three parts.");

    }

    Ok(())
}
