// Used modules
use std::fs;
use crate::{packages as pacs, FgColor};





pub enum Distro<'a> {
    Debian (_Distro<'a>),
    Fedora (_Distro<'a>),
    Arch (_Distro<'a>),
    Unknown,
}


pub struct _Distro<'a> {
    pub install:  &'a[&'a str],
    pub update:  &'a[&'a str],
    pub upgrade:  Option<&'a[&'a str]>,
    pub suffix: Option<&'a[&'a str]>,
    pub basepkg: &'static[&'static str],
    pub desktop: &'static[&'static str],
}


impl<'a> Distro<'a> {
    pub fn get_distro() -> Distro<'a> {
        // Read /etc/os-release
        let contents =
            fs::read_to_string(String::from("/etc/os-release"))
            .expect("File does not exist");

        // Search for distros
        if let Some(_) = contents.find("Debian") {
            return Self::Debian(
                _Distro {
                    install: &["apt","install", "-y"],
                    update: &["apt", "update", "-y"],
                    upgrade: Some(&["apt", "upgrade", "-y"]),
                    suffix: None,
                    basepkg: pacs::DEB_BASE,
                    desktop: pacs::DEB_DESKTOP,
                }

            );
        }
        if let Some(_) = contents.find("Fedora") {
            return Self::Fedora(
                _Distro {
                    install: &["dnf","install", "-y"],
                    update: &["dnf", "update", "-y"],
                    upgrade: None,
                    suffix: None,
                    basepkg: pacs::FED_BASE,
                    desktop: pacs::FED_DESKTOP,
                }

            );
        }
        if let Some(_) = contents.find("Arch Linux") {
            return Self::Arch(
                _Distro {
                    install: &["pacman","-S"],
                    update: &["pacman", "-Syu"],
                    upgrade: None,
                    suffix: Some(&["--noconfirm", "--needed"]),
                    basepkg: pacs::ARCH_BASE,
                    desktop: pacs::ARCH_DESKTOP,
                }
            )
        }
        return Self::Unknown;
    }
}
