// string library test.

mod xstring;

use xstring::XString;

fn main() {
    
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