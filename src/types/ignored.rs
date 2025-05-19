// Used modules
use std::{
    env::{self, VarError},
    fs,
    os::unix::fs::symlink,
    path::Path,
};
use super::transfer::Transfer;
use crate::{utils::fileutils as fu, FgColor};


pub struct Ignored {
    src: String,
    dest: String,
    name: String,
}

impl Ignored {
    /// This function returns a new [Ignored] struct.
    /// Which contains the absolute path of the source and destination
    ///
    /// # Example
    /// ```
    /// // paths have to be relative to $HOME
    /// let ign = Ignored::new("/Jazzian/dotfiles/vim", ".config/vim", "vim")
    ///         .expect("Failed to get Environment variable");
    /// ```
    ///
    /// Returns an Err if determining $HOME fails.
    pub fn new( src: &str, dest: &str, name: &str) -> Result<Self, VarError> {
        // Get $HOME
        let homedir = {
            match env::var("HOME") {
                Ok(dir) => dir,
                Err(e) => {
                    return Err(e);
                }
            }
        };


        // Build source directory
        let mut srcbuf = String::new();
        srcbuf.push_str(&homedir);
        srcbuf.push_str(src);

        // Build dest directory
        let mut destbuf = String::new();
        destbuf.push_str(&homedir);
        destbuf.push_str(dest);



        let result = Ignored {
            src: srcbuf,
            dest: destbuf,
            name: String::from(name),
        };
        return Ok(result);
    }


    // TODO: Write proper docs!!!
    pub fn apply(&self, method: Transfer) {
        // Create paths
        let srcpath = Path::new(&self.src);
        let destpath = Path::new(&self.dest);


        // Check if source exists
        match fs::exists(srcpath) {
            Ok(true) => {},
            Ok(false) => {
                println!("Source does not exist");
                return;
            },
            Err(e) => {
                println!("An error occured checking for existence: {e:?}");
                return;
            }
        }


        // Check if destination does not exists
        match fs::exists(destpath) {
            Ok(false) => {},
            Ok(true) => {
                println!("Destination does exist not moving");
                return;
            },
            Err(e) => {
                println!("An error occured checking for existence: {e:?}");
                return;
            }
        }


        let parent = {
            match destpath.parent() {
                Some(val) => val,
                None => {
                    println!("No permissison to write to '/'");
                    return;
                }
            }
        };

        // TODO: Proper error handling
        //
        // Create parent directory if it does not exist
        match fs::exists(parent) {
            Ok(true) => {},
            Ok(false) => {let _ = fs::create_dir_all(parent);},
            Err(e) => {
                println!(
                    "{}An error occured checking for existence{}: {e:?}",
                    FgColor!(Red),
                    FgColor!(),
                );
                return;
            }
        }

        // Transfer the file(s)
        match method {
            Transfer::Link => {
                let _ = symlink(srcpath, destpath);
            }
            Transfer::Copy => {
                if srcpath.is_dir() {
                    fu::copy_dir_all(srcpath, destpath);
                } else if srcpath.is_file() {
                    let _ = fs::copy(srcpath, destpath);
                }
            }
            _ => {}
        }
    }
}
