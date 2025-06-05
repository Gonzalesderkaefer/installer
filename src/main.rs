// Modules declarations
mod packages;
mod types;
mod formatting;
mod utils;
mod config;


// Used modules and types
use types::{customized::create_customized, ignored::Ignored, system::System};
use utils::fileutils::{self as fu, search_replace};
use std::{collections::HashMap, ffi::OsString, fs::{self, read_dir}, os::unix::fs::symlink, path::PathBuf};


/// This function copies everything from from inside `src` into `dest`
/// This means `src` has to exist and `dest` can exist. `src` and `dest`
/// have to be paths relative to $HOME, `ignored` can be used to define
/// targets that should not be moved, the key should be the basename of
/// the file or directory. If `hidden` is set to true, the file at the
/// destination will be hidden
fn move_files(
    sys: &System,
    src: &str,
    dest: &str,
    ignored:  &HashMap<OsString, Ignored>,
    hidden: bool,
) {
    // Build src buf
    let mut srcbuf = PathBuf::new();
    srcbuf.push(&sys.home);// Home is from the sys struct
    srcbuf.push(src);

    // Check if src exists. Return if not
    match fs::exists(&srcbuf) {
        Ok(true) => {}
        Ok(false) => {
            println!("{}Source does not exist.{}", 
                FgColor!(Red),
                FgColor!(),
            );
            return;
        }
        Err(e) => {
            println!("{}Unable to determine if source exists: {e:?}{}", 
                FgColor!(Red),
                FgColor!(),
            );
            return;
        }
    }

    // Build dest buf
    let mut destbuf = PathBuf::new();
    destbuf.push(&sys.home);// Home is from the sys struct
    destbuf.push(dest);

    // Check if dest exists. Create it if not
    match fs::exists(&destbuf) {
        Ok(true) => {}
        Ok(false) => {
            match fs::create_dir_all(&destbuf) {
                Ok(_) => {}
                Err(e) => {
                    println!(
                        "{}Unable to create dir {e:?}{}",
                        FgColor!(Red),
                        FgColor!(),
                    );
                    return;
                }
            }
        }
        Err(e) => {
            println!("{}Unable to determine if destination exists: {e:?}{}", 
                FgColor!(Red),
                FgColor!(),
            );
            return;
        }
    }

    // Open source directory, on failure, terminate function
    let source_dir = {
        match read_dir(&srcbuf) { // Error checking
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

    for direntres in source_dir {
        // Error check on direntres, if its error skip it
        let dirent = {
            match direntres {
                Ok(ent) => ent,
                Err(_) => continue, // Skip on error
            }
        };

        // If the file should be ignored contains the the file skipt
        if ignored.contains_key(&dirent.file_name()) {
            match ignored.get(&dirent.file_name()) {
                Some(a) => a.apply(&sys.transfer),
                None => {}
            }
            continue;
        }

        // Error check on metadata, if its error skip it
        let metadata = {
            match dirent.metadata() {
                Ok(ent) => ent,
                Err(_) => continue, // Skip on error
            }
        };


        // Build full src path
        let mut complete_src_buf = srcbuf.clone();

        // Build full dest path
        let mut complete_dest_buf = destbuf.clone();

        // Add dot if the dest should be hidden
        if hidden {
            complete_dest_buf.push(".");
        }

        // Append basenames
        complete_src_buf.push(dirent.file_name());
        complete_dest_buf.push(dirent.file_name());


        // Check for transfer method
        match &sys.transfer {
            types::transfer::Transfer::Link => { // Symlink
                match symlink(complete_src_buf, complete_dest_buf) { // Error checking
                    Ok(_) => {}
                    Err(e) => { // Print message and skip to next
                        println!(
                            "{}Symlink failed: {e:?}{}",
                            FgColor!(Red),
                            FgColor!()
                        );
                        continue;
                    }
                }
            }


            types::transfer::Transfer::Copy => {
                if metadata.is_dir() { // Copy recursively
                    fu::copy_dir_all(
                        complete_src_buf.as_path(),
                        complete_dest_buf.as_path()
                    );
                } else { // Just copy, on error skip
                    let _ = fs::copy(complete_src_buf, complete_dest_buf);
                }
            }
            types::transfer::Transfer::None => {}
        }
    }
}

fn make_customized(sys: &System) {
    // Create customized files
    for tup in config::CUSTOMIZED {
        create_customized(tup.0, tup.1, tup.2);
    }

    // Getting home
    let home = sys.home.clone();

    // Build customized.sh path
    let mut customizedbuf = PathBuf::new();
    customizedbuf.push(&home);
    customizedbuf.push(".customized.sh");
    let customizedpath = customizedbuf.as_path();

    match &sys.display {
        types::display::DspServer::Xorg(xorg_wm, _) => {
            search_replace("\\&\\& \\(.*;", customizedpath, "startx");
            // Build customized.sh path
            let mut xinitbuf = PathBuf::new();
            xinitbuf.push(&home);
            xinitbuf.push(".xinitrc");
            let xinitpath = xinitbuf.as_path();

            match xorg_wm {
                types::display::XorgWM::Awesome(_) =>
                    search_replace("exec .*", xinitpath, "exec awesome"),
                types::display::XorgWM::Bspwm(_) =>
                    search_replace("exec .*", xinitpath, "exec bspwm"),
                types::display::XorgWM::I3(_) =>
                    search_replace("exec .*", xinitpath, "exec i3"),

            }
        }
        types::display::DspServer::Wayland(wl_comp, _) => {
            match wl_comp {
                types::display::WlComp::Hyprland(_) => 
                    search_replace("\\&\\& \\(.*;", customizedpath, "&& (Hyprland;"),

                types::display::WlComp::River(_) => 
                    search_replace("\\&\\& \\(.*;", customizedpath, "&& (river;"),

                types::display::WlComp::Sway(_) => 
                    search_replace("\\&\\& \\(.*;", customizedpath, "&& (sway;"),

            }
        }
        types::display::DspServer::Desktop => {
            search_replace("\\&\\& \\(.*\\)", customizedpath, "");
        },
    }
}


fn main() {
    // Get system from user
    let sys = {
        match System::get() {
            Ok(a) => a,
            Err(_) => return,
        }
    };

    // Update system
    sys.update();

    // Install packages
    sys.install();

    // HashMap for ignored configs
    let mut ignored: HashMap<OsString, Ignored> = HashMap::new();

    // Insert ignored configs into HashMap
    ignored.insert(OsString::from("shell"), Ignored::new("",""));

    // Move config files
    move_files(&sys, config::CFGSRC, config::CFGDEST, &ignored, false);

    // Move scripts
    move_files(&sys, config::BINSRC, config::BINDEST, &ignored, false);

    // Move shell files
    move_files(&sys, "Jazzian/dotfiles/config/shell", "", &ignored, true);


    // Create customized files
    make_customized(&sys);
}
