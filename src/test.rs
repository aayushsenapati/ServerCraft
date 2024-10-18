use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn test() -> std::io::Result<()> {
    // Connect to the Minecraft server
    let mut stream = match TcpStream::connect("127.0.0.1:9191") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to connect to the server: {}", e);
            return Err(e);
        }
    };

    // Send the 0xFE byte to query the server
    if let Err(e) = stream.write(&[0xFE]) {
        eprintln!("Failed to send query to the server: {}", e);
        return Err(e);
    }

    // Read the response
    let mut buffer = [0; 512];
    let mut data = Vec::new();

    while let Ok(size) = stream.read(&mut buffer) {
        if size == 0 {
            break; // Connection closed
        }
        for &byte in &buffer[..size] {
            // Filter out unwanted characters
            if byte != 0 && byte > 16 && byte != 255 && byte != 23 && byte != 24 {
                data.push(byte);
                // println!("{}:{}", byte, byte as char);
            }
        }
    }

    if data.is_empty() {
        eprintln!("No data received from the server or server not running.");
        return Ok(());
    }

    // Split the data into three parts based on ASCII 167 (which is '§')
    let parts: Vec<Vec<u8>> = data.split(|&x| x == 167).map(|x| x.to_vec()).collect();

    // Check if we have at least 3 parts and convert them to strings
    if parts.len() >= 3 {
        let part1 = str::from_utf8(&parts[0]).unwrap_or_else(|e| {
            eprintln!("Failed to convert part 1 to UTF-8: {}", e);
            ""
        });
        let part2 = str::from_utf8(&parts[1]).unwrap_or_else(|e| {
            eprintln!("Failed to convert part 2 to UTF-8: {}", e);
            ""
        });
        let part3 = str::from_utf8(&parts[2]).unwrap_or_else(|e| {
            eprintln!("Failed to convert part 3 to UTF-8: {}", e);
            ""
        });

        // Print the three strings
        println!("MOTD: {}", part1);
        println!("Active Players: {}", part2);
        println!("Max Players: {}", part3);
    } else {
        eprintln!("Failed to parse server response into three parts.");
    }

    Ok(())
}
