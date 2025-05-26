// Modules declarations
mod packages;
mod types;
mod formatting;
mod utils;
mod def;


// Used modules and types
use types::{customized::create_customized, system::System};
use utils::fileutils::{self as fu, search_replace};
use std::{
     env, fs, os::unix, path::{self, PathBuf}
};


fn move_files(sys: &System, src: &str, dest: &str /*ignored:  &HashMap<bool, Ignored>*/) {
    // Getting home directory
    let home = sys.home.clone();



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

fn make_customized(sys: &System) {
    // Create customized files
    for tup in def::CUSTOMIZED {
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
            search_replace("&& \\(.*;", customizedpath, "startx");
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
                    search_replace("&& \\(.*;", customizedpath, "&& (Hyprland;"),

                types::display::WlComp::River(_) => 
                    search_replace("&& \\(.*;", customizedpath, "&& (river;"),

                types::display::WlComp::Sway(_) => 
                    search_replace("&& \\(.*;", customizedpath, "&& (sway;"),

            }
        }
        types::display::DspServer::Desktop => {
            search_replace("&& \\(.*\\)", customizedpath, "");
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

    // Move dotfiles
    move_files(&sys, def::CFGSRC, def::CFGDEST);

    // Move scripts
    move_files(&sys, def::BINSRC, def::BINDEST);

    // Create the custom files
    make_customized(&sys);



}
