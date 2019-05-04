use std::io;
use std::io::prelude::*;
use std::fs::File;
//use std::str;

fn main() -> io::Result<()>{

    let mut file = File::open("test.txt")?;

    // Create a Vec with just enough capacity to store file data
    let mut bytes: Vec<u8> = Vec::with_capacity(file.metadata().unwrap().len() as usize);

    file.read_to_end(&mut bytes).unwrap();
    
    let copy_bytes = bytes.clone();

    println!("Raw data:");
    for byte in bytes.iter() {
        println!("{}", byte);
    }

    println!("Delta endcoded data:");
    let delta_vec = delta_encode(&copy_bytes);

    println!("Delta decoded data:");
    let _decoded_vec = delta_decode(&delta_vec);

    //println!("Data:\n{}", str::from_utf8(&copy_bytes).unwrap());

    println!("Finished");
    Ok(())
}

// Delta-encoding a vector of bytes
//  Loops over vector of bytes and return new vector with deltas
//   49 50 51 97 98 99 -> 49 1 1 46 1 1
fn delta_encode(data: &Vec<u8>) -> Vec<i8> {

    let mut delta_vec: Vec<i8> = Vec::with_capacity(data.len());
    let mut last_byte = 0;

    for byte in data.iter() {
        let current_byte = *byte;
        let delta = (current_byte as i16 - last_byte as i16) as i8;
        delta_vec.push(delta);
        println!("{}", delta);
        last_byte = current_byte;
    }

    delta_vec
}

// Decoding a vector of deltas
//  Loops over delta vector and return new vector with original data
//   49 1 1 46 1 1 -> 49 50 51 97 98 99
fn delta_decode(delta_vec: &Vec<i8>) -> Vec<u8> {
    
    let mut decoded_vec: Vec<u8> = Vec::with_capacity(delta_vec.len());
    let mut last_delta = 0;

    for delta in delta_vec.iter() {
        let current_delta = *delta;
        let byte = (current_delta as i16 + last_delta as i16) as u8;
        decoded_vec.push(byte);
        println!("{}", byte);
        last_delta = byte;
    }

    decoded_vec
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_delta_encode() {
        assert_eq!(delta_encode(&vec![49, 50, 51, 97, 98, 99]), vec![49, 1, 1, 46, 1, 1]);
    }

    #[test]
    fn test_delta_decode() {
        assert_eq!(delta_decode(&vec![49, 1, 1, 46, 1, 1]), vec![49, 50, 51, 97, 98, 99]);
    }
}