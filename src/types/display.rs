// Used modules
use crate::{utils::{fileutils::get_all_sub_paths, menu::print_menu}, FgColor};
use super::distro::Distro;
use crate::packages as pacs;
use std::{fs::{self, read_dir, OpenOptions}, io::{self, Write}, path::PathBuf};

/// This enum reprresents a Wayland compositor
/// this compositor has an array of packages that are associated
/// with it.
pub enum WlComp {
    Hyprland(&'static[&'static str]),
    River(&'static[&'static str]),
    Sway(&'static[&'static str]),
    Niri(&'static[&'static str]),
}
impl WlComp {
    /// This function returns A [WlComp] which contains an array
    /// of packages as [&str]. A distro has to be provided.
    pub fn get(distro: &Distro<'_>) -> io::Result<Self> {
        let chares = print_menu(
            "Please choose a compositor",
            &[
                "[N]iri",
                "[H]yprland",
                "[R]iver",
                "[S]way (Default)",
            ],
        );
        match chares {
            Ok('n') | Ok('N') =>
                match distro {
                    Distro::Debian(_distro) => Ok(Self::Sway(pacs::DEB_SWAY)),
                    Distro::Fedora(_distro) => Ok(Self::Niri(pacs::FED_NIR)),
                    Distro::Arch(_distro) => Ok(Self::Niri(pacs::ARCH_NIR)),
                    Distro::Unknown => Ok(Self::Niri(&[""])),
                }

            Ok('h') | Ok('H') =>
                match distro {
                    Distro::Debian(_distro) => Ok(Self::Sway(pacs::DEB_SWAY)),
                    Distro::Fedora(_distro) => Ok(Self::Hyprland(pacs::FED_HYPR)),
                    Distro::Arch(_distro) => Ok(Self::Hyprland(pacs::ARCH_HYPR)),
                    Distro::Unknown => Ok(Self::Hyprland(&[""])),
                }
            Ok('r') | Ok('R') =>
                match distro {
                    Distro::Debian(_distro) => Ok(Self::Sway(pacs::DEB_SWAY)),
                    Distro::Fedora(_distro) => Ok(Self::River(pacs::FED_RIV)),
                    Distro::Arch(_distro) => Ok(Self::River(pacs::ARCH_RIV)),
                    Distro::Unknown => Ok(Self::River(&[""])),
                },
            Ok(_) => 
                match distro {
                    Distro::Debian(_distro) => Ok(Self::Sway(pacs::DEB_SWAY)),
                    Distro::Fedora(_distro) => Ok(Self::Sway(pacs::FED_SWAY)),
                    Distro::Arch(_distro) => Ok(Self::Sway(pacs::ARCH_SWAY)),
                    Distro::Unknown => Ok(Self::Sway(&[""])),
                },
            Err(e) => Err(e),
        }
    }

    /// This function runs any code for the specified window manager
    /// It is used to 'initialize' that Window manager
    fn init(&self) {
        match self {
            WlComp::Hyprland(_) => {}
            WlComp::River(_) => {},
            WlComp::Sway(_) => {},
            WlComp::Niri(_) => {
                // Get home
                let hme = {
                    match std::env::var("HOME") {
                        Ok(h) => h,
                        Err(_) => {
                            return;
                        }
                    }
                };

                // Build config dir for niri
                let mut config_dir = PathBuf::new();
                config_dir.push(&hme);
                config_dir.push(".config/niri");

                // Build default config file
                let mut config_file = PathBuf::new();
                config_file.push(&hme);
                config_file.push(".config/niri/config.kdl");
                // Delete file if it exists
                if let Ok(true) = fs::exists(&config_file) {
                    match fs::remove_file(&config_file) {
                        Ok(_) => {}
                        Err(e) => {
                            println!(
                                "{}Unable to delete file {e:?}{}",
                                FgColor!(Red),
                                FgColor!());
                            return;

                        },
                    }
                }

                // Get all sub paths to the config directory
                let mut config_files: Vec<String> = Vec::new();
                get_all_sub_paths(config_dir.as_path(), &mut config_files);


                // This string holds the complete concatination of the 
                // files
                let mut all_content = String::new();


                // Concat everything into all_content
                for config_module in config_files {
                    if ! config_module.ends_with(".kdl") {
                        continue;
                    }
                    match fs::read_to_string(config_module) {
                        Ok(cont) => {
                            all_content.push_str(&cont.as_str());
                        },
                        Err(_) => continue,
                    }
                    all_content.push('\n'); // For readablity
                }

                // Open config file for writing
                let mut new_config_file = match OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(config_file) {
                        Ok(f) => f,
                        Err(e) => {
                            println!(
                                "{}Unable to open file for writing {e:?}{}",
                                FgColor!(Red),
                                FgColor!());
                            return;
                        }
                    };


                // Write to newly created file
                if let Err(e) = new_config_file.write(all_content.as_bytes()) {
                    println!(
                        "{}Unable to open read to file {e:?}{}",
                        FgColor!(Red),
                        FgColor!());
                    return;
                }

            },
        }
    }
}

/// This enum reprresents a Xorg window manager
/// this compositor has an array of packages that are associated
/// with it.
pub enum XorgWM {
    Awesome(&'static[&'static str]),
    Bspwm(&'static[&'static str]),
    I3(&'static[&'static str]),
}
impl XorgWM {
    /// This function returns A [XorgWM] which contains an array
    /// of packages as [&str]. A distro has to be provided.
    pub fn get(distro: &Distro<'_>) -> io::Result<Self> {
        let chares = print_menu(
            "Please choose a Windowmanager",
            &[
                "[A]wesome",
                "[B]spwm",
                "[I]3 (Default)",
            ],
        );
        match chares {
            Ok('a') | Ok('A') =>
                match distro {
                    Distro::Debian(_distro) => Ok(Self::Awesome(pacs::DEB_AWE)),
                    Distro::Fedora(_distro) => Ok(Self::Awesome(pacs::FED_AWE)),
                    Distro::Arch(_distro) => Ok(Self::Awesome(pacs::ARCH_AWE)),
                    Distro::Unknown => Ok(Self::Awesome(&[""])),
                }
            Ok('b') | Ok('B') =>
                match distro {
                    Distro::Debian(_distro) => Ok(Self::Bspwm(pacs::DEB_BSP)),
                    Distro::Fedora(_distro) => Ok(Self::Bspwm(pacs::FED_BSP)),
                    Distro::Arch(_distro) => Ok(Self::Bspwm(pacs::ARCH_BSP)),
                    Distro::Unknown => Ok(Self::Bspwm(&[""])),
                },
            Ok(_) => 
                match distro {
                    Distro::Debian(_distro) => Ok(Self::I3(pacs::DEB_I3)),
                    Distro::Fedora(_distro) => Ok(Self::I3(pacs::FED_I3)),
                    Distro::Arch(_distro) => Ok(Self::I3(pacs::ARCH_I3)),
                    Distro::Unknown => Ok(Self::I3(&[""])),
                },
            Err(e) => Err(e),
        }
    }

    fn init(&self) {
        match &self {
            XorgWM::Awesome(_items) => {}
            XorgWM::Bspwm(_items) => {}
            XorgWM::I3(_items) => {}
        }
    }
}






pub enum DspServer {
    Xorg (XorgWM, &'static[&'static str]),
    Wayland (WlComp, &'static[&'static str]),
    Desktop,
}
impl DspServer {
    /// This function gets the Displayserver from the user
    /// and needs a reference to a [Distro] 
    /// To select the proper packages
    pub fn get(distro: &Distro) -> io::Result<Self> {
        let charres = print_menu(
            "Please choose a displayserver",
            &[
                "[W]ayland",
                "[T]ty",
                "[X]org (Default)",
            ],
        );
        match charres {
            Ok('w') | Ok('W') => {
                let comp = {
                    match  WlComp::get(distro) {
                        Ok(compy) => compy,
                        Err(e) => { 
                            return Err(e);
                        }
                    }
                };
                match distro {
                    Distro::Debian(_distro) => Ok(Self::Wayland(comp, pacs::DEB_WAY)),
                    Distro::Fedora(_distro) => Ok(Self::Wayland(comp, pacs::FED_WAY)),
                    Distro::Arch(_distro) => Ok(Self::Wayland(comp, pacs::ARCH_WAY)),
                    Distro::Unknown => todo!(),
                }


            }
            Ok('t') | Ok('T') => Ok(Self::Desktop),
            Ok(_) => {
                let wm = {
                    match  XorgWM::get(distro) {
                        Ok(wmy) => wmy,
                        Err(e) => { 
                            return Err(e);
                        }
                    }
                };
                match distro {
                    Distro::Debian(_distro) => Ok(Self::Xorg(wm, pacs::DEB_XORG)),
                    Distro::Fedora(_distro) => Ok(Self::Xorg(wm, pacs::FED_XORG)),
                    Distro::Arch(_distro) => Ok(Self::Xorg(wm, pacs::ARCH_XORG)),
                    Distro::Unknown => todo!(),
                }
            }

            Err(_) => todo!(),
        }
    }

    pub fn init(&self) {
        match &self {
            DspServer::Xorg(xorg_wm, _items) => {
                XorgWM::init(xorg_wm);
            }
            DspServer::Wayland(wl_comp, _items) => {
                WlComp::init(wl_comp);
            }
            DspServer::Desktop => {
            }
        }
    }
}







