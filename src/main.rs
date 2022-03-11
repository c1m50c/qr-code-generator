use std::vec::Vec;
mod encode;


fn main() {
    // Message to encode within the QR Code.
    let message = std::env::args().nth(1)
        .expect("Expected a message argument.");
    
    // Where the QR Code will be saved, including the file extension.
    let output_path = std::env::args().nth(2)
        .expect("Expected an output path argument");
    
    let flags: Vec<String> = std::env::args().skip(3)
        .filter(|s| s.find('-') == Some(0))
        .collect();
    
    // Retrieve Version Flags, then isolate the first one into a `u32`.
    let version = flags.iter()
        .filter(|&s| s.find('v') == Some(1)) // Find "-v" flags.
        .map(|s| s.clone()) // Make Strings owned.
        .collect::<Vec<String>>().get(0) // Collect all version flags into a Vec, then grab the first item.
        .unwrap_or(&String::from("1")) // Retrieve the Some value or "1".
        .trim_start_matches("-v").to_string() // Remove the "-v" prefix of the flag.
        .parse::<u32>()
        .expect("Cannot parse the version flag into an unsigned integer.");
    
    assert!(version >= 1 && version <= 40, "Invalid Version.");
    
    if cfg!(debug_assertions) {
        // Debug Info
        println!("Message: {:?}", message);
        println!("Output Path: {:?}", output_path);
        println!("Flags: {:?}", flags);
        println!("Version: {:?}", version);
    }

    let qr_code = encode::generate(message, version);
    qr_code.save(output_path.clone())
        .expect(format!("Failed to save QR Code to {:?}", output_path).as_str());
}
