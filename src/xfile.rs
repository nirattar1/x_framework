// a utility library for interacting with files.

use std::fs::File;
use std::io::prelude::*;
use std::io::{ Read, Error };

pub struct XFile {
    m_filename: String,
}

impl XFile {
    /**
     * read x bytes from file given in filename, into vector of chars.
     * @return bool if succeeded, false otherwise.
     */
    pub fn read(filename: String, bytes: usize, vec: &mut Vec<u8>) -> Result<usize, Error> {
        let open_result = File::open(filename);
        let mut f = match open_result {
            Ok(file) => { file }
            Err(e) => {
                return Err(e);
            }
        };

        // read into a temp vector.
        let mut vec_tmp = vec![0u8; bytes];
        let read_result = f.read_exact(&mut vec_tmp);
        match read_result {
            Ok(..) => {
                *vec = vec_tmp.clone();
                Ok(bytes)
            }
            Err(e) => { Err(e) }
        }
    }

    pub fn write(filename: String, buffer: &Vec<u8>) -> Result<usize, Error> {
        let mut file: File = match File::create(&filename) {
            Ok(f) => { f }
            Err(e) => {
                println!("create failed for file {}: {e}", &filename);
                return Err(e);
            }
        };

        match file.write_all(buffer) {
            Ok(_) => { Ok(buffer.len()) }
            Err(e) => { Err(e) }
        }
    }

    pub fn exists(filename: String) -> bool {
        match File::open(filename) {
            Ok(..) => true,
            Err(..) => false,
        }
    }
}
