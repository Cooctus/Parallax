use crate::checks::dependncies;
use crate::checks::have_pkg;
use crate::checks::versioncheck;

use std::io::*;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
extern crate clap;
use clap::*;
use std::path::Path;
use std::ptr::null;
use std::collections::HashMap;
use std::io;
use std::fs::OpenOptions;
extern crate regex;
use std::process::*;
use regex::Regex;
#[macro_use]
use std::env::*;
extern crate reqwest;

pub struct Package{
    pub build_instructions:  String,
    pub url:  String,
    pub name: String,
    pub file_name: String,
    pub version: String,
    pub dependncies: String
}

impl Package {

    pub fn set_vars(&mut self, file_contents: &String) {
        let lines: Vec<&str> = file_contents.split('\n').filter(|&s| s != "").collect();
        let url_regex = Regex::new(r"^URL\s=\s\W(\S+)\W*").unwrap();
        let build_regex = Regex::new(r"^BUILD_INSTRUCTIONS\s=\s*\W*(\S+)\W*").unwrap();
        let version_regex = Regex::new(r#"^VERSION\s=\s*\W*(\S+[^"])\W*"#).unwrap();
        let file_name_regex = Regex::new(r#"^NAME\s=\s*\W*(\S+[^"])\W*"#).unwrap();
        let dependencies_regex = Regex::new(r"^DEPENDENCIES\s*=\s*\W*(.+)\W*").unwrap();
        for i in lines.clone() {
            if (url_regex.is_match(i))
                {
                    for c in url_regex.captures_iter(i) {
                        &self.url.push_str(&c[1][0..c[1].len()-1]);
                    }
                }
            if (dependencies_regex.is_match(i))
                {
            println!("{}",i.clone());
                    for c in dependencies_regex.captures_iter(i) {

                            let mut temp = String::new();
                            let mut a = 0;
                            for x in lines.iter().position(|&r| r == i).unwrap() + 1..lines.len() {
                                if (lines[x] == ")") {
                                    a += 1;
                                    break;
                                }
                                temp.push_str(&lines[x]);
                                temp.push_str("&&");
                            }
                            println!("{} temp",temp.clone());
                            if (a == 1) {
                                &self.dependncies.push_str(&temp[0..temp.len() - 2]);
                            } else if (a != 1 && i.contains(")")) {
                                &self.dependncies.push_str(&c[1]);
                            } else {
                                panic!("Build Instructions are not in the right format");
                            }
                    }
                }
            if (version_regex.is_match(i))
                {
                    for c in version_regex.captures_iter(i) {
                        println!("{} + {}",self.name.clone(),&c[1][0..c[1].len()]);
                        &self.version.push_str(&c[1][0..c[1].len()]);
                    }
                }
            if(file_name_regex.is_match(i)){
                for c in file_name_regex.captures_iter(i) {
                    &self.file_name.push_str(&c[1][0..c[1].len()]);
                }
            }
            if (build_regex.is_match(i)) {
                for c in build_regex.captures_iter(i) {
                    let mut temp = String::new();
                    let mut a = 0;
                    for x in lines.iter().position(|&r| r == i).unwrap() + 1..lines.len() {
                        if (lines[x] == ")") {
                            a += 1;
                            break;
                        }
                        temp.push_str(&lines[x]);
                        temp.push_str(" && ");
                    }

                    if (a == 1) {
                        &self.build_instructions.push_str(&temp[0..temp.len() - 4]);
                    } else if (a != 1 && i.contains(")")) {
                        &self.build_instructions.push_str(&c[1]);
                    } else {
                        panic!("Build Instructions are not in the right format");
                    }
                }
            }
        }
    }

    pub fn install(&self) {
        println!("{}\n{}\n{}\n{}",self.build_instructions,self.file_name,self.version,self.url);
        let mut  v : String = String::new();
        let existing_check = crate::checks::have_pkg::file_already_installed(self.file_name.clone(),self.version.clone());
        let version_check = crate::checks::versioncheck::new_version(self.file_name.clone(),self.version.clone());
        crate::checks::dependncies::install_dependncies(self.dependncies.clone());
        println!("{}",existing_check);
        println!("{}",version_check);
        if(existing_check){
            println!("test");
            return
        }
        if(version_check){
            println!("You already have the newest version");
            return
        }
        println!("Test 12389 {}",self.url.clone());
        let mut res = reqwest::get(&self.url).expect("Package not found");
        let cl = &self.url.clone();
        let x = cl.rfind("/").unwrap();
        let mut file_name = &cl[x+1..cl.len()];
        println!("{}",file_name);
        let mut file = File::create(&mut file_name).expect("Error creating file");
        io::copy(&mut res, &mut file).expect("Error creating file");
        let mut lines: &Vec<&str> = &self.build_instructions.split("&&").collect();
        for x in 0..lines.len() {
            let mut cmd = String::from(lines[x]);
            cmd = self.set_env_vars(&mut cmd);
            if(&lines[x][1..3] == "cd"){
                let mut current_dir =String::from(std::env::current_dir().unwrap().to_str().unwrap()) + &String::from("/");
                current_dir.push_str(&cmd[4..cmd.len()-1]);
                let path = Path::new(&current_dir);
                std::env::set_current_dir(path);

            }

            println!("{}",cmd);
            let mut nyan = Command::new("sh").arg("-c").arg(cmd).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
            let s = nyan.stdout.as_mut().unwrap();
            let sr = BufReader::new(s);
            let sl = sr.lines();
            for li in sl{
                println!("{:?}",li);
            }
        }
        let mut package_checker = OpenOptions::new().append(true).read(true).write(true).open("/etc/parallax/package_list.txt").expect("Failed to read package_list");
        writeln!(package_checker,"{}",self.file_name.clone() + " " + &self.version);
    }

    fn set_env_vars(&self, line :&mut String) -> String{
        let mut options = HashMap::new();
        options.insert("$NAME",&self.file_name);
        options.insert("$VERSION",&self.version);
        let mut x = line.clone();
        for (k,v) in options{
            if(line.contains(k)){

                x = x.replace(k,v);

            }
        }
        x
    }
}