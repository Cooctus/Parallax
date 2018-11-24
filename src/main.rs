use std::io::*;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
extern crate clap;
use clap::*;
use std::path::Path;
use std::io;
extern crate regex;
use std::process::*;
use regex::Regex;
#[macro_use]
use std::env::*;
extern crate reqwest;
#[macro_use]
extern crate duct;
struct Package{
    build_instructions:  String,
    url:  String,
    name: String,
    file_name: String
}

impl Package {

    fn set_vars(&mut self, file_contents: &String) {
        let lines: Vec<&str> = file_contents.split('\n').filter(|&s| s != "").collect();
        let url_regex = Regex::new(r"^URL\s=\s\W(\S+)\W").unwrap();
        let build_regex = Regex::new(r"^BUILD_INSTRUCTIONS\s=\s*(.+)").unwrap();
        let file_name_regex = Regex::new(r"^NAME\s=\s*(.+)").unwrap();
        for i in lines.clone() {
            if (url_regex.is_match(i))
                {
                    for c in url_regex.captures_iter(i) {
                        &self.url.push_str(&c[1]);
                    }
                }
            if(file_name_regex.is_match(i)){
                for c in file_name_regex.captures_iter(i) {
                    &self.file_name.push_str(&c[1]);
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
        let mut current_dir =String::from(std::env::current_dir().unwrap().to_str().unwrap()) + &String::from("/");
        let mut res = reqwest::get(&self.url).expect("Package not found");
        let mut file = File::create(&mut self.file_name.clone()).expect("Error creating file");
        io::copy(&mut res, &mut file).expect("Error creating file");
        let lines: &Vec<&str> = &self.build_instructions.split("&&").collect();
        println!("{:?}",lines);
        for x in 0..lines.len() {

            if(&lines[x][1..3] == "cd"){

                current_dir.push_str(&lines[x][4..lines[x].len()-1]);
                current_dir.push_str("/");
                continue;


            }
            if(current_dir != ""){
                let file_name_regex = Regex::new(r"^(\w*).+").unwrap();
                let mut file_name=  String::new();
                if(file_name_regex.is_match(&lines[x][1..lines[x].len()])){
                    for c in file_name_regex.captures_iter(&lines[x][1..lines[x].len()]){
                        file_name.push_str(&c[1]);
                    }

                }
                let file_path = String::from(current_dir.clone() + &file_name);
                println!("{}",file_path);
                match  fs::metadata(file_path.clone()){
                    Ok(..) => {
                        println!("FOUND IT");

                        let o = current_dir.clone();

                        let mut lo =  Command::new("sh").arg("-c").arg( "su -c \"".to_owned() + &current_dir.clone() + &lines[x][1..lines[x].len()-1] + &"\" - paradox".to_owned()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
                        let s = lo.stdout.as_mut().unwrap();
                        let sr = BufReader::new(s);
                        let sl = sr.lines();
                        for li in sl{
                            println!("{:?}",li);
                        }
                        continue;
                    },
                    Err(E) => {
                    }

                }



            }
            let mut nyan = Command::new("sh").arg("-c").arg(lines[x]).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
            let s = nyan.stdout.as_mut().unwrap();
            let sr = BufReader::new(s);
            let sl = sr.lines();
            for li in sl{
                println!("{:?}",li);
            }
        }
    }
}

fn main() {
    let mut Contents: String = String::new();
    let grab = App::new("Parallax").version("1.0").arg(
        Arg::with_name("install").long("install").takes_value(true)
    ).get_matches();


    let pkg = grab.value_of("install");
    match pkg{
        Some(pkg) => {

            let mut file = File::open(pkg.to_owned() + ".pkg");
            match file{
                Ok(mut file) =>
                    {
                        file.read_to_string(&mut Contents);
                        let mut package: Package = Package {url: String::new(), build_instructions: String::new(), name : String::from(pkg), file_name: String::new()};
                        package.set_vars(&Contents);
                        println!("a");
                        package.install();
                    },
                Err(file) => {
                    println!("Package not found!");
                    return
                }
            };

        }
        None => println!("None")
    };


}
