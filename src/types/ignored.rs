// Used modules
use std::{
    fs,
    os::unix::fs::symlink,
    path::Path,
};
use super::transfer::Transfer;
use crate::{utils::fileutils as fu, FgColor};


pub struct Ignored {
    src: String,
    dest: String,
}

impl Ignored {
    /// This function returns a new [Ignored] struct.
    /// Which contains the absolute path of the source and destination
    ///
    /// # Example
    /// ```
    /// // paths have to be absolute
    /// let ign = Ignored::new(
    ///     "/home/urname/Jazzian/dotfiles/vim",
    ///     "/home/urname/.config/vim",
    ///     "vim"
    /// );
    /// ```
    ///
    ///
    pub fn new(src: &str, dest: &str) -> Self {
        // Build source directory
        let mut srcbuf = String::new();
        srcbuf.push_str(src);

        // Build dest directory
        let mut destbuf = String::new();
        destbuf.push_str(dest);



        let result = Ignored {
            src: srcbuf,
            dest: destbuf,
        };
        return result;
    }


    /// This function applies the Operations defined
    /// in the [Ignored], provided. Additionally you have
    /// to supply a method of [Transfer] 
    ///
    /// # Example
    /// ```
    /// // paths have to be absolute
    /// let ign = Ignored::new(
    ///     "/home/urname/Jazzian/dotfiles/vim",
    ///     "/home/urname/.config/vim",
    ///     "vim"
    /// );
    /// 
    /// // This creates a symlink with the name of dest and the value of src
    /// ign.apply(Transfer::Link);
    /// ```
    pub fn apply(&self, method: &Transfer) {
        // If src and dest are empty, nothing needs to be done
        if self.src.len() == 0 || self.dest.len() == 0 {
            return;
        }

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


        // Check that destination does not exist
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
            Transfer::None => {}
        }
    }
}
