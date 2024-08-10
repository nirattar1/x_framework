// a utility library for interacting with files.

use std::fs::File;
use std::io::Read;

pub struct XFile {
    m_filename: String
}

impl XFile {
    /**
     * read x bytes from file given in filename, into vector of chars.
     * @return bool if succeeded, false otherwise.
     */
    pub fn read(filename: String, bytes: usize, vec: &mut Vec<u8>) -> bool {
        let open_result = File::open(filename);
        if open_result.is_err() {
            return false;
        }

        let mut f = open_result.unwrap();
        // read into a temp vector.
        let mut vec_tmp = vec![0u8; bytes];
        let read_result = f.read_exact(&mut vec_tmp);
        match read_result {
            Ok(..) => {
                *vec = vec_tmp.clone();
                return true;
            },
            Err(..) => return false,            
        };
    }

    pub fn exists(filename: String) -> bool {
        match File::open(filename) {
            Ok(..) => true,
            Err(..) => false
        }
    }

}