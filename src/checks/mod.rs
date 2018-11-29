pub mod versioncheck;
pub mod  have_pkg;
pub mod dependncies;
fn check_version(name : String, version : String){
    versioncheck::new_version(name,version);
}

fn check_pkg(name : String, version : String){
    have_pkg::file_already_installed(name,version);
}