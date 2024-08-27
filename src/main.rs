// string library test.
#![allow(dead_code)]
#![allow(unused_variables)]

mod xstring;
mod xfile;
mod xscheduler;

use xstring::XString;
use xfile::XFile;
use xscheduler::{ JobParams, XScheduler };
use crate::xscheduler::start_scheduler_event_loop;
use std::thread;
use std::time::Duration;

fn main() {
    test_xscheduler();
}

fn mytest_add_one(n: i32) -> i32 {
    n + 1
}

fn mytest_multiply_by_2(n: i32) -> i32 {
    n * 2
}

fn test_xscheduler() {
    // s.queue_job(JobParams::new(5, String::from("/bin/ls"), vec![String::from("/")]));

    // s.queue_job(JobParams::new(1, String::from("/bin/ps"), vec![]));

    // s.queue_job(JobParams::new(1, String::from("/bin/sleep"), vec![String::from("10")]));

    let (handle, scheduler_ptr) = start_scheduler_event_loop();

    scheduler_ptr.queue_job(JobParams::new(5, String::from("/bin/ls"), vec![String::from("/")]));

    scheduler_ptr.queue_job(JobParams::new(1, String::from("/bin/ps"), vec![]));

    scheduler_ptr.queue_job(
        JobParams::new(1, String::from("/bin/sleep"), vec![String::from("10")])
    );

    thread::sleep(Duration::from_millis(2000));

    scheduler_ptr.queue_job(
        JobParams::new(0, String::from("/bin/ls"), vec![String::from("/home")])
    );

    thread::sleep(Duration::from_millis(1000));

    handle.join().unwrap();
}

#[test]
fn test_xfile() {
    assert_eq!(true, XFile::exists(String::from("test_files/1.txt")));

    assert_eq!(false, XFile::exists(String::from("test_files/nonexist.txt")));

    let test_data: Vec<u8> = vec![5, 2, 3, 4, 5, 100, 99, 98, 101];
    assert!(!XFile::write(String::from("test_files/tmp.txt"), &test_data).is_err());

    assert!(XFile::write(String::from("/dev/i"), &test_data).is_err());

    let mut read_output: Vec<u8> = Vec::new();
    assert!(
        !XFile::read(String::from("test_files/tmp.txt"), test_data.len(), &mut read_output).is_err()
    );

    // compare vector to and from.
    assert_eq!(
        test_data.len(),
        read_output
            .iter()
            .zip(test_data.iter())
            .filter(|&(a, b)| { a == b })
            .count()
    );
}

#[test]
fn test_string() {
    let s1: &str = "dfndnfdf";
    let xs2 = XString::from(s1);
    println!("string: {}, string length: {}", xs2.to_string(), xs2.len());
    assert_eq!(xs2.to_string(), String::from("dfndnfdf"));
    assert_eq!(xs2.len(), s1.len());

    let xs3 = XString::new();
    println!("string: {}, string length: {}", xs3.to_string(), xs3.len());

    let mut xs4 = XString::from("my name is ");
    println!("string: {}, string length: {}", xs4.to_string(), xs4.len());

    xs4.concat(xs2);
    println!("string after concat: {}, string length: {}", xs4.to_string(), xs4.len());
}
