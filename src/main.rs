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
    
    if cfg!(debug_assertions) {
        // Debug Info
        println!("Message: {:?}", message);
        println!("Output Path: {:?}", output_path);
        println!("Flags: {:?}", flags);
    }

    let qr_code = encode::generate(message);
    qr_code.save(output_path.clone())
        .expect(format!("Failed to save QR Code to {:?}", output_path).as_str());
}
