// Used modules
use std::{env::{self, VarError}, path::Path};


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
    /// ```{.rs}
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


    pub fn apply(&self, /* Transfer */) {
        let srcpath = Path::new(&self.src);
        let destpath = Path::new(&self.dest);
    }
}
