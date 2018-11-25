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
#[macro_use]
extern crate duct;
mod repo;
mod checkers;
struct Package{
    build_instructions:  String,
    url:  String,
    name: String,
    file_name: String,
    version: String
}

impl Package {

    fn set_vars(&mut self, file_contents: &String) {
        let lines: Vec<&str> = file_contents.split('\n').filter(|&s| s != "").collect();
        let url_regex = Regex::new(r"^URL\s=\s\W(\S+)\W*").unwrap();
        let build_regex = Regex::new(r"^BUILD_INSTRUCTIONS\s=\s*\W*(\S+)\W*").unwrap();
        let version_regex = Regex::new(r"^VERSION\s=\s*\W*(\S+)\W*").unwrap();
        let file_name_regex = Regex::new(r"^NAME\s=\s*\W*(\S+)\W*").unwrap();
        for i in lines.clone() {
            if (url_regex.is_match(i))
                {
                    for c in url_regex.captures_iter(i) {
                        &self.url.push_str(&c[1][0..c[1].len()-1]);
                    }
                }
            if (version_regex.is_match(i))
                {
                    for c in version_regex.captures_iter(i) {

                        &self.version.push_str(&c[1][0..c[1].len()-1]);
                    }
                }
            if(file_name_regex.is_match(i)){
                for c in file_name_regex.captures_iter(i) {
                    &self.file_name.push_str(&c[1][0..c[1].len()-1]);
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

    fn install(&self) {
        println!("{}\n{}\n{}\n{}",self.build_instructions,self.file_name,self.version,self.url);
        let mut  v : String = String::new();
        let existing_check = checkers::file_already_installed(self.file_name.clone(),self.version.clone());
        let version_check = checkers::new_version(self.file_name.clone(),self.version.clone());
        if(existing_check){
           return
        }
        if(version_check){
            println!("You already have the newest version");
            return
        }
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
        package_checker.write_all((self.file_name.clone() + " " + &self.version).as_bytes());
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

fn main() {
    let mut Contents: String = String::new();
    let grab = App::new("Parallax").version("1.0").arg(
        Arg::with_name("install").long("install").takes_value(true)
    ).arg(Arg::with_name("package-dir").long("package-dir").short("pd").takes_value(true)).get_matches();
    let mut  d = "";
    let dir = grab.value_of("package-dir");

    match dir {
        Some(dir) => { d = dir.clone() }
        None => {}
    }
    let pkg = grab.value_of("install");

        match pkg{
        Some(pkg) => {
            let mut package: Package = Package {url: String::new(), build_instructions: String::new(), name : String::from(pkg), file_name: String::new(), version: String::new()};
            if(d == ""){
                Contents = repo::load_url(pkg.clone().to_string());
            }
            else {
                let mut file = File::open(d.to_owned() + &pkg.to_owned() + ".pkg").expect("fail");
                file.read_to_string(&mut Contents);
            }
            package.set_vars(&Contents);
            package.install();


        }


        None => println!("None")
    };

}
