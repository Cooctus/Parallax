use std::fs::*;
use std::io;
use std::fs;
extern crate semver;
use semver::Version;
use semver::VersionReq;
use std::io::{Read,Write};

pub fn file_already_installed(name : String, version: String) -> bool {
    let mut v = String::new();
    let mut package_checker = OpenOptions::new().append(true).read(true).write(true).open("/etc/parallax/package_list.txt").expect("Failed to read package_list");
    package_checker.read_to_string(&mut v);
    let vec: Vec<&str> = v.split("\n").collect();
    for lines in vec{
        println!("{} and {}",lines,name.clone() + " " +  &version);
        if(lines == name.clone() + " " +  &version){
            println!("You have this package");
            return true
        }
    }

    return false
}