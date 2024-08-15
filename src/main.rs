// string library test.
#![allow(dead_code)]
#![allow(unused_variables)]

mod xstring;
mod xfile;
mod xscheduler;

use xstring::XString;
use xfile::XFile;
use xscheduler::{ JobParams, XScheduler };
use crate::xscheduler::run_event_loop;
use std::thread;

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

    run_event_loop();
}

fn test_xfile() {
    println!("{}", XFile::exists(String::from("1.txt")));

    let mut test_vec1: Vec<u8> = Vec::new();
    let read_result = XFile::read(String::from("1.txt"), 10, &mut test_vec1);
    println!("read result: {} buffer: {:?}", read_result, &test_vec1);
}

fn test_string() {
    let s1: &str = "dfndnfdf";
    let xs2 = XString::from(s1);
    println!("string: {}, string length: {}", xs2.to_string(), xs2.len());

    let xs3 = XString::new();
    println!("string: {}, string length: {}", xs3.to_string(), xs3.len());

    let mut xs4 = XString::from("my name is ");
    println!("string: {}, string length: {}", xs4.to_string(), xs4.len());

    xs4.concat(xs2);
    println!("string after concat: {}, string length: {}", xs4.to_string(), xs4.len());
}
