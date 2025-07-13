// Used modules
use std::{
    fs::{self, File}, io::{Read, Write}, path::Path
};

use crate::FgColor;


pub fn get_all_sub_paths(dir: &Path, paths: &mut Vec<String>){
    // this is the dir as a string
    let dir_string  = match dir.to_str() {
        Some(s) => s.to_string(),
        None => "".to_string(),
    };



    // Open dir for reading
    let par_dir = {
        match fs::read_dir(&dir_string) { // Error checking
            Ok(dir) => dir,
            Err(e) => {
                println!(
                    "{}Unable to read dir {e:?}{}",
                    FgColor!(Red),
                    FgColor!());
                return;
            }
        }
    };


    for dirent in par_dir {
        // Get element
        let elem = match dirent {
            Ok(data) => data,
            Err(e) => {
                println!(
                    "{}Unable to get dirent {e:?}{}",
                    FgColor!(Red),
                    FgColor!());
                continue;
            },
        };

        // Push path to paths
        paths.push(match elem.path().to_str() {
            Some(e) => e.to_string(),
            None => String::new(),
        });


        // If dirent is a directory, call this function
        // recursively on that directory
        if let Ok(typ) = elem.file_type() {
            if typ.is_dir() {
                get_all_sub_paths(elem.path().as_path(), paths);
            }
        }
    }
}


pub fn copy_dir_all(src: &Path, dest: &Path) {
    // Check if src exists
    match fs::exists(src) {
        Ok(true) => {}
        Ok(false) => {
            println!(
                "{}{src:?} does not exist{}",
                FgColor!(Red),
                FgColor!(),
            );
            return;
        }
        Err(e) => {
            println!(
                "{}Unable to determine if source exists {e:?} {}",
                FgColor!(Red),
                FgColor!(),
            );
            return;
        }
    }


    // Check if dest will have new basename
    // or keep the old one
    if dest.is_dir() {
        let mut destbuf = dest.to_path_buf();
        let src_base = {
            match src.file_name() {
                Some(name) => name,
                None => {
                    return ();     // TODO: Figure out how to
                                   // handle this
                }
            }
        };
        destbuf.push(Path::new(src_base));
        let new_dest = destbuf.as_path();
        _copy_dir_all(src, new_dest);
    } else {
        _copy_dir_all(src,dest);
    }
}



// This function assumes that the base name of
// `dest` does not exist yet
fn _copy_dir_all(src: &Path, dest: &Path) {
    // Check if dest exists
    if let Ok(true) = fs::exists(dest) {
        println!("Destination exists, not copying");
        return ();
    }
    if let Err(e) = fs::exists(dest) {
        println!("Cannot determine if destination exists: {e}");
        return ();
    }

    // TODO: create the directory in the destination
    match fs::create_dir_all(dest) {
        Ok(_) => {
            println!("Info: created dir {dest:?}");
        }
        Err(e) => {
            println!("Error: Could not create dir {dest:?}: {e:?}");
            return ();
        }
    }

    // Open src directory
    let srcdir = {
        match fs::read_dir(src) {
            Ok(dir) => dir,
            Err(e) => {
                println!("Could not open source dir: {e:?}");
                return ();
            }
        }
    };

    for dirresult in srcdir {
        // Get DirEntry
        let elem = {
            match dirresult {
                Ok(file) => file,
                Err(e) => {
                    println!("Could not read dir element: {e:?}");
                    continue;
                }
            }
        };

        // Get filetype
        let src_type = {
            match elem.file_type() {
                Ok(typ) => typ,
                Err(e) => {
                    println!("Could not determine file type: {e}");
                    continue;
                }
            }
        };

        // Get Destination path buffer
        let mut dest_buf = dest.to_path_buf();
        dest_buf.push(elem.file_name());

        if src_type.is_dir() {
            _copy_dir_all(&elem.path().as_path(), &dest_buf.as_path());
        } else {
            let _ = fs::copy(&elem.path(), &dest_buf);
            //println!("{:?} to {:?}", elem.path().display(), dest_buf.display());
        }
    }
}





pub fn search_replace(tosearch: &str, path: &Path, replace: &str){
    // Open file
    let mut file = {
        match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("could not open {path:?}: {e:?}");
                return;
            }
        }
    };


    // Read contents of file
    let mut filecontent: String = String::new();
    let _ = {
        match file.read_to_string(&mut filecontent) {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to read {path:?}: {e:?}");
                return;
            }
        }
    };






    // Compile regex
    let re = {
        match regex::Regex::new(tosearch) {
            Ok(regex) => regex,
            Err(_) => {
                println!(
                    "{}Invalid pattern{}",
                    FgColor!(Red),
                    FgColor!());
                return;
            },
        }
    };
    // Search for pattern
    let mat = {
        match re.find(&filecontent) {
            Some(val) => val,
            None => {
                println!("Could not find pattern \'{tosearch}\' in {filecontent}");
                return;
            }
        }
    };

    // Replace
    filecontent.replace_range(mat.start()..mat.end(), replace);

    // Open file for writing
    let mut file_to_write = {
        match File::create(path) {
            Ok(file) => file,
            Err(e) => {
                println!("could not open {path:?}: {e:?}");
                return;
            }
        }
    };



    // write to file
    match file_to_write.write(filecontent.as_bytes()) {
        Ok(_) => {},
        Err(e) => {
            println!("Could not write to {path:?}: {e:?}");
            return;
        }
    }
}
