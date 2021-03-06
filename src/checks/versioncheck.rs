use std::fs::*;
use std::io;
use std::fs;
extern crate semver;
use semver::Version;
use semver::VersionReq;
use std::io::{Read,Write};
pub fn new_version(name : String, version : String) -> bool {
    let mut v = String::new();
    let mut package_checker = OpenOptions::new().append(true).read(true).write(true).open("/etc/parallax/package_list.txt").expect("Failed to read package_list");
    package_checker.read_to_string(&mut v);
    let mut count = 0;
    let mut vec: Vec<&str> = v.split("\n").filter(|&x| x != "").collect();
    if(vec.contains(&(name.clone()).as_str())){
        println!("{}",&(name.clone() + " " + &version.clone()).as_str());
        return true
    }
    for i in vec.clone(){
        let info: Vec<&str> = i.split(" ").collect();
        println!("{} and {}",info[0],name.clone());
        if(name == info[0]){
            println!("test 100 {}",info[1]);

            if(Version::parse(version.as_str()) > Version::parse(info[1])){
                let x = vec.iter().position(|&index| index == i).unwrap();
                println!("{}",x);
                fs::remove_file("/etc/parallax/package_list.txt");
                File::create("/etc/parallax/package_list.txt");
                package_checker = OpenOptions::new().append(true).read(true).write(true).open("/etc/parallax/package_list.txt").expect("Failed to read package_list");
                for line in vec.clone(){
                    println!("{} and {}",line,i);

                    if(line == i) {
                        writeln!(package_checker,"");
                    }
                        else{
                            package_checker.write_all(line.as_bytes());

                        }
                }

                return false
            }


        }
        count+=1;
    }
    println!("{} and len {}",count.clone(),vec.len());
    if(count == vec.len()){
        return false

    }

    true
}