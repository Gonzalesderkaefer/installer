// Used modules
use std::fs;
use crate::{packages as pacs, FgColor};





pub enum Distro<'a> {
    Debian (
        &'a[&'a str], // install cmd
        &'a[&'a str], // update cmd
        &'a[&'a str], // upgrade cmd
    ),
    Fedora (
        &'a[&'a str], // install cmd
        &'a[&'a str], // update cmd
    ),
    Arch (
        &'a[&'a str], // install cmd
        &'a[&'a str], // upgrade cmd
        &'a[&'a str], // suffix
    ),
    Unknown,
}




impl<'a> Distro<'a> {
    /// This method gets the [Distro] that is running
    /// on the system
    pub fn get_distro() -> Distro<'a> {
        // Read /etc/os-release
        let contents = { 
            match fs::read_to_string(String::from("/etc/os-release")) {
                Ok(cont) => cont,
                Err(e) => {
                    println!(
                        "{}Failed to read \"/etc/os-release\" {e:?}{}",
                        FgColor!(Red),
                        FgColor!()
                    );
                    return Self::Unknown;
                },
            }
        };

        // Search for distros
        if let Some(_) = contents.find("Debian") {
            return Self::Debian(
                &["apt", "install", "-y"],
                &["apt", "update", "-y"],
                &["apt", "upgrade", "-y"],
            );
        }
        if let Some(_) = contents.find("Fedora") {
            return Self::Fedora(
                &["dnf", "install", "-y"], 
                &["dnf", "update", "-y"]
            );
        }
        if let Some(_) = contents.find("Arch Linux") {
            return Self::Arch(
                &["pacman", "-S"],
                &["pacman", "-Syu", "--noconfirm", "--needed"],
                &["--noconfirm", "--needed"],
            );
        }
        return Self::Unknown;
    }

    pub fn packages(&self) -> &'static[&'static str] {
        match self {
            Self::Debian(_,_,_) => return pacs::DEB_BASE,
            Self::Fedora(_,_) => return pacs::FED_BASE,
            Self::Arch(_,_,_) => return pacs::ARCH_BASE,
            Self::Unknown => &[""],
        }
    }
}
