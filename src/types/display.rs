// Used modules
use crate::utils::menu::print_menu;
use super::distro::Distro;
use crate::packages as pacs;
use std::io;

/// This enum reprresents a Wayland compositor
/// this compositor has an array of packages that are associated
/// with it.
pub enum WlComp {
    Hyprland(&'static[&'static str]),
    River(&'static[&'static str]),
    Sway(&'static[&'static str]),
}
impl WlComp {
    /// This function returns A [WlComp] which contains an array
    /// of packages as [&str]. A distro has to be provided.
    pub fn get(distro: &Distro<'_>) -> io::Result<Self> {
        let chares = print_menu(
            "Please choose a compositor",
            &[
                "[H]yprland",
                "[R]iver",
                "[S]way (Default)",
            ],
        );
        match chares {
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
                    Distro::Debian(_distro) => Ok(Self::Awesome(pacs::DEB_I3)),
                    Distro::Fedora(_distro) => Ok(Self::Awesome(pacs::FED_I3)),
                    Distro::Arch(_distro) => Ok(Self::Awesome(pacs::ARCH_I3)),
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
}






pub enum DspServer {
    Xorg (XorgWM, &'static[&'static str]),
    Wayland (WlComp, &'static[&'static str]),
    Tty,
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
            Ok('t') | Ok('T') => Ok(Self::Tty),
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
}







