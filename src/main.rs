// string library test.
#![allow(dead_code)]
#![allow(unused_variables)]


mod xstring;
mod xfile;

use xstring::XString;
use xfile::XFile;

fn main () {
    test_xfile();
}

fn test_xfile () {
    println!("{}", XFile::exists(String::from("1.txt")));

    let mut test_vec1: Vec<u8> = Vec::new();
    let read_result = XFile::read(String::from("1.txt"), 10, &mut test_vec1);
    println!("read result: {} buffer: {:?}", read_result, &test_vec1);

}


fn test_string() {    
    let s1: &str = "dfndnfdf";
    let xs2 = XString::from(s1);
    println!("string: {}, string length: {}", 
        xs2.to_string(),
        xs2.len());

    let xs3 = XString::new();
    println!("string: {}, string length: {}", 
        xs3.to_string(),
        xs3.len());

    let mut xs4 = XString::from("my name is ");
    println!("string: {}, string length: {}", 
        xs4.to_string(),
        xs4.len());

    xs4.concat(xs2);
    println!("string after concat: {}, string length: {}", 
        xs4.to_string(),
        xs4.len());

}