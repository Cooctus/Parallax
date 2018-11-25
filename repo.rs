use std::fs;
use std::io::Read;
use std::fs::File;
use std::io::*;
use std::str;

extern crate reqwest;
extern crate ftp;
use ftp::FtpStream;

pub fn load_url(name: String) -> String{
let mut Contents = String::new();
    let mut file_list = String::new();
    File::open("/etc/parallax/repolist").unwrap().read_to_string(&mut file_list);
    let file_vec : Vec<&str> = file_list.split('\n').collect();
    for repo in file_vec{
        if(&repo.clone()[0..3] == "ftp"){
            let mut ftp_stream = FtpStream::connect((repo,21)).unwrap();
            ftp_stream.login("anonymous", "").unwrap();
            let remote_file = ftp_stream.simple_retr(&(name.clone() + &".pkg".to_string())).unwrap();
            Contents = String::from(str::from_utf8(&remote_file.into_inner()).unwrap());
            break;
        }

    }
    Contents
    }


