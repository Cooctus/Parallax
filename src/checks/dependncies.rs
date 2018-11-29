
use std::env::*;
extern crate reqwest;
use crate::repo::load_url;
use crate::packages::Package;
pub fn install_dependncies(dependcies: String) -> bool{
    println!(" T100 {}",dependcies.clone());
let deps :Vec<&str> = dependcies.split("&&").filter(|&dep| dep != "").collect();
println!(" gtsfcsdf {:?}",deps);
    for x in deps{
    let mut pkg = load_url(x.clone().to_string());
        if(pkg.len() > 1){

            println!("{}",pkg);
            let mut package: Package = Package{url: String::new(), build_instructions: String::new(), name : String::from(x.clone().to_string()), file_name: String::new(), version: String::new(), dependncies: String::new()};
            package.set_vars(&pkg);
            println!("url {}\n{}",package.build_instructions.clone(),package.file_name);
            package.install();

        }

    }
    true
    }