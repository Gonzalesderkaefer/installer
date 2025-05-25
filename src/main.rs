// Modules declarations
mod packages;
mod types;
mod formatting;
mod utils;
mod def;


// Used modules and types
use types::{system::System};
use utils::fileutils as fu;
use std::{
     env, fs, os::unix, path
};




fn move_files(sys: &System, src: &str, dest: &str /*ignored:  &HashMap<bool, Ignored>*/) {
    // Getting home directory
    let home = {
        match env::var("HOME") {
            Ok(var) => var,
            Err(_) => {
                println!(
                    "{}Could not get $HOME{}",
                    FgColor!(Red),
                    FgColor!(),
                );
                return;

            }
        }
    };

    // Build source directory
    let mut srcbuf = path::PathBuf::new();
    srcbuf.push(&home);
    srcbuf.push(src);
    let srcpath = srcbuf.as_path();


    // Build dest directory
    let mut destbuf = path::PathBuf::new();
    destbuf.push(&home);
    destbuf.push(dest);
    let destpath = destbuf.as_path();


    // Open src dir
    let dotdir = {
        match fs::read_dir(srcpath) {
            Ok(v) => v,
            Err(e) => {
                println!(
                    "{}Failed to to open {srcpath:?}: {e:?}{}",
                    FgColor!(Red),
                    FgColor!(),
                );
                return;
            }
        }
    };

    // Check if destination exists
    match fs::exists(destpath) {
        Ok(true) => {}
        Ok(false) => {
            match fs::create_dir_all(destpath) {
                Ok(_) => {}
                Err(e) => {
                    println!(
                        "{}Could not create {destpath:?}: {e:?}{}",
                        FgColor!(Red),
                        FgColor!(),
                    );
                }
            }
        }
        Err(e) => {
            println!(
                "{}Could not determine if dir exists: {e:?}{}",
                FgColor!(Red),
                FgColor!(),
            );
        }
    }

    // Open dest dir
    let cfgdir = fs::read_dir(destpath);








    match sys.transfer {
        types::transfer::Transfer::Link => {
            for elemres in dotdir {
                // Unwrap elem
                let elem =  {
                    match elemres {
                        Err(_) => continue,
                        Ok(val) => val,
                    }
                };


                // Extend dest path
                let mut dest = destpath.to_path_buf();
                match elem.path().file_name() {
                    Some(var) => dest.push(var),
                    None => continue,
                }

                match unix::fs::symlink(elem.path(), dest) {
                    Err(e) => {
                        println!(
                            "{}Could not symlink: {e:?} {}",
                            FgColor!(Red),
                            FgColor!(),
                        );
                        continue;
                    }
                    Ok(_) => {},
                }
            }
        }
        types::transfer::Transfer::Copy => {
            for elemres in dotdir {
                // Unwrap elem
                let elem =  {
                    match elemres {
                        Err(_) => continue,
                        Ok(val) => val,
                    }
                };
                fu::copy_dir_all(elem.path().as_path(), destpath);
            }
        }
        types::transfer::Transfer::None => {
            return;
        }
    }

}

fn main() {
    let sys = {
        match System::get() {
            Ok(a) => a,
            Err(_) => return,
        }
    };
    move_files(&sys, def::CFGSRC, def::CFGDEST);
}
