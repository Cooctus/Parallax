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
mod checks;

mod packages;

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
            let mut package: packages::Package = packages::Package {url: String::new(), build_instructions: String::new(), name : String::from(pkg), file_name: String::new(), version: String::new(), dependncies: String::new()};
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
