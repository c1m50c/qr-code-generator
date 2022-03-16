use std::collections::HashMap;
use std::vec::Vec;

pub mod render;
pub mod encode;


/// Retrieves the flags and their parameters from the environment's arguments, storing them in a [`HashMap`].
/// 
/// # Example
/// ```rust
/// // TERMINAL:$ "./qrgen -m Hello World! -o ./output-path.bmp"
/// let flags = get_flags(std::env::args().skip(1).collect::<Vec<_>>());
/// let flags_as_map = HashMap::new();
/// 
/// flags_as_map.insert("-m".to_string(), "Hello World!".to_string());
/// flags_as_map.insert("-o".to_string(), "./output-path.bmp".to_string());
/// 
/// assert_eq!(flags, flags_as_map);
/// ```
fn get_flags(args: Vec<String>) -> HashMap<String, String> {
    let mut map = HashMap::new();

    // If there is no explicit flag passed into the application treat all arguments as part of the message.
    if args.iter().filter(|&s| matches!(s.as_str(), "-m" | "-o" | "-v")).collect::<Vec<_>>().len() == 0 {
        let mut parameters = String::new();

        args.iter().for_each(|s| {
            parameters.push_str((s.clone() + " ").as_str());
        });

        map.insert(
            "-m".into(), parameters.trim_end().to_string()
        );

        return map;
    }

    // TODO: This section could use some cleanup

    let mut flags = args.iter()
        .filter(|&s| s.chars().nth(0).unwrap_or(' ') == '-').cloned()
        .collect::<Vec<_>>();
    
    let split_parameters = args.split(|s| s.chars().nth(0).unwrap_or(' ') == '-')
        .skip(1)
        .collect::<Vec<_>>();
    

    let mut parameters = Vec::with_capacity(flags.len());

    split_parameters.iter().for_each(|&slice| {
        let mut new_parameter = String::new();
        
        slice.iter().cloned().for_each(|s| {
            new_parameter.push_str((s + " ").as_str());
        });

        parameters.push(new_parameter.trim_end().to_string());
    });

    assert_eq!(flags.len(), parameters.len(), "Error in get_flag(), non-equal amount of flags compared to parameters.");

    while let (Some(flag), Some(parameter)) = (flags.pop(), parameters.pop()) {
        map.insert(flag, parameter);
    }

    return map;
}


fn main() {
    let env_args = std::env::args().skip(1)
        .collect::<Vec<_>>();
    
    assert!(env_args.len() != 0, "Application requires atleast one argument.");
    
    let flags = get_flags(env_args.clone());

    let mut message: String = String::new();
    let mut output: String = String::from("./output-qrcode.bmp");
    let mut version: String = String::from("5");

    if cfg!(debug_assertions) {
        println!("Args: {:?}", env_args);
        println!("Flags: {:?}", flags);
    }

    for (key, val) in flags {
        match key.to_lowercase().as_str() {
            "-m" => message = val,
            "-o" => output = val,
            "-v" => version = val,
            f => panic!("Invalid flag {:?} given.", f),
        }
    }

    if message.is_empty() {
        panic!("No message given, prefix a \"-m\" to a String to create a message.");
    }

    if output.is_empty() {
        panic!("No output path given, prefix a \"-o\" to a String to create an output path.");
    }

    if version.is_empty() {
        panic!("No version given, prefix a \"-v\" to a Number to create a version.");
    }

    let version = version.parse::<u32>()
        .expect(format!("Cannot parse {:?} into a 32-bit unsigned integer.", version).as_str());
    
    let qr_code = render::as_image(message.as_bytes(), version);
    qr_code.save(output.clone())
        .expect(format!("Failed to save QRCode to path {:?}.", output).as_str());
}
