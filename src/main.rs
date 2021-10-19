extern crate rand;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::convert::TryInto;
use std::io;


fn read_file(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Wrong path dumbass!");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Woops! I got and error while trying to read the the file :( try again later!");
    return content;
}

fn encrypt(file_path: &str, output_file_path: &str) -> String {

    let clamp = |val: u16, min: u16, max:u16| -> u16 { 
                                            if val < min { min } else if val > max { max } else { val }
    }; 

    let content = read_file(file_path);
    let mut output = String::new();
    for chr in content.chars() {
        let mut char_code: u16 = chr as u16;
        char_code += clamp(rand::thread_rng().gen_range(0, 255), 0, 255);
        output.push((char_code as u8) as char);
    }

    let mut output_file = File::create(output_file_path).expect("Wrong path idiot!");
    output_file.write_all(output.as_bytes());

    return output;
}

fn main() {
    println!("Please enter the input file path: ");
    let mut input_file_path = String::new();
    io::stdin().read_line(&mut input_file_path).unwrap();
    if input_file_path.ends_with('\n') { 
        input_file_path = input_file_path[0..input_file_path.len() - 2].to_string()
    }

    println!("Now, enter the output file path: ");
    let mut output_file_path = String::new();
    io::stdin().read_line(&mut output_file_path).unwrap();
    if output_file_path.ends_with('\n') { 
        output_file_path = output_file_path[0..output_file_path.len() - 2].to_string()
    }

    encrypt(&input_file_path as &str, &output_file_path as &str);
    print!("Encrypted with success!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file_test() {
        let content = read_file("dummy.txt");
        println!("{}", content);    
    }

    #[test]
    fn encrypt_test() {
        encrypt("dummy.txt", "dummy-encrypted.txt");
    }
}