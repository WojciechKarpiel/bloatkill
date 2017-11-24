#![feature(libc)]
extern crate libc;
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::io::prelude::*;
use libc::{kill, SIGTERM}; //todo może SIGKILL xD

const MEMORY_CONSUMPTION_THRESHOLD_BYTES: u64 = 1024 * 1024;

fn main() {
    //todo proper getOpts
    let memory_consumption_threshold_bytes: u64 = match env::args().next() {
        Some(arg) => MEMORY_CONSUMPTION_THRESHOLD_BYTES,
        None => MEMORY_CONSUMPTION_THRESHOLD_BYTES,
    };
    println!("Hello, world!");
    let files = fs::read_dir("/proc/").unwrap();

    for file in files {
        let file: DirEntry = file.unwrap();
        if file.metadata().unwrap().is_dir() {
            match file.path()
                      .file_name()
                      .unwrap()
                      .to_str()
                      .unwrap()
                      .parse::<i32>() {//  .to_str().unwrap().parse::<u64>() {
                Ok(pid) => {
                    let mut statmFile: std::fs::File =
                        std::fs::File::open(file.path().join(Path::new("statm"))).unwrap();

                    let mut stri = String::new();
                    statmFile.read_to_string(&mut stri).unwrap();

                    let a: u64 = stri.split(' ').next().unwrap().parse().unwrap();

                    //    std::io::stdout().write(&buf);
                    println!("Process {} takes {}", pid, a);
                    if (a > memory_consumption_threshold_bytes) {
                        println!("KILLING THE {}", pid);
                        unsafe {
                            kill(pid, SIGTERM);
                        }
                    }
                }
                Err(r) => println!("NOT PID {}", file.path().display()),
            }
        }
    }


}
