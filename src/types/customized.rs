// Std and other libs
use std::env;
use std::fs;
use std::fs::set_permissions;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;

pub fn create_customized(path: &str, contents: &str, mode: u32) {
    // Check if path starts with /
    if path.as_bytes()[0] != b'/' {
        println!("Error: path has to start with '/'");
        return;
    }


    // Build full path and parent
    let mut homestr: String;
    match env::var("HOME") {
        Ok(val) => {homestr = val;}
        Err(e) => {
            println!("Error: could'nt get env var: {e}");
            return;
        }
    }
    homestr.push_str(path);
    let homebuf = PathBuf::from(&homestr);
    //homebuf.push(path);
    let fullpath = homebuf.as_path();
    let parent = {
        match fullpath.parent() {
            Some(val) => val,
            None => {
                println!("Error: No permissions to write to /");
                return();
            }
        }
    };

    // If fullpath exists terminate
    if let Ok(true) = fs::exists(fullpath) {
        return;
    }



    // Check if path is dir
    match path.as_bytes()[path.len() - 1] {
        b'/' => {
            // Create dir
            let _ = fs::create_dir_all(fullpath);
        }
        _ => {
            // Create parent dir if does not exist
            if let Ok(false) = fs::exists(parent) {
                let _ = fs::create_dir_all(parent);
            }

            // Write to the file
            match fs::write(fullpath, contents.as_bytes()) {
                Ok(_) => {},
                Err(err) => {
                    println!("Error: Failed to write to: {:?}: {err}", fullpath);
                    return;
                }
            }
        }
    }

    // Set permissions
    let _ = set_permissions(
        Path::new(fullpath),
        PermissionsExt::from_mode(mode)
    );
}
