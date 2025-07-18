use crate::FgColor;

// Used modules
use super::{display::DspServer, display::WlComp, display::XorgWM, distro::Distro, transfer::Transfer, };
use std::{io::Error, io, process::Command };

pub struct System<'a> {
    distro: Distro<'a>,
    pub home: String,
    pub display: DspServer,
    pub transfer: Transfer
}

impl<'a> System <'a> {
    pub fn get() -> Result<Self, Error> {
        // Store distro
        let dist = Distro::get_distro();

        // Get the dsp server return an error if it
        // returns an error
        let dsp = {
            match DspServer::get(&dist) {
                Ok(dsp) => dsp,
                Err(e) => {
                    return Err(e);
                }
            }
        };

        // Get the home dir
        let hme = {
            match std::env::var("HOME") {
                Ok(h) => h,
                Err(e) => {
                    return match e {
                        std::env::VarError::NotPresent => 
                            Err(std::io::Error::new(io::ErrorKind::NotFound, e)),
                        std::env::VarError::NotUnicode(_) => 
                            Err(std::io::Error::new(io::ErrorKind::InvalidData, e))
                    }
                }
            }
        };



        // Get The transfermethod
        let tr = {
            match Transfer::get_transfer() {
                Ok(value) => value,
                Err(e) => {
                    return Err(e);
                }
            }
        };

        return Ok(System {
            distro: dist,
            home: hme,
            display: dsp,
            transfer: tr,
        });
    }

    pub fn install(&self) {
        // This also stores the install cmd
        let mut pacs: Vec<&str> = Vec::new();

        // Get reference to suffix
        let mut suffix: Option<&'a [&'a str]> = None;


        if let DspServer::Desktop = &self.display {
             let dist = {
                 match &self.distro {
                    Distro::Debian(_distro) => _distro,
                    Distro::Fedora(_distro) => _distro,
                    Distro::Arch(_distro) => _distro,
                    Distro::Unknown => {
                        return;
                    }
                }
             };

             for var in dist.desktop {
                 pacs.push(var);
             }



        } else {
            // Add install cmd and base packs
            match &self.distro {
                Distro::Debian(_distro) => {
                    // Add install cmd
                    for var in _distro.install {
                        pacs.push(var);
                    }
                    // Add packs
                    for var in _distro.basepkg {
                        pacs.push(var);
                    }
                    suffix = _distro.suffix;
                }
                Distro::Fedora(_distro) => {
                    // Add install cmd
                    for var in _distro.install {
                        pacs.push(var);
                    }
                    // Add packs
                    for var in _distro.basepkg {
                        pacs.push(var);
                    }
                    suffix = _distro.suffix;
                }
                Distro::Arch(_distro) => {
                    // Add install cmd
                    for var in _distro.install {
                        pacs.push(var);
                    }
                    // Add packs
                    for var in _distro.basepkg {
                        pacs.push(var);
                    }
                    suffix = _distro.suffix;
                }
                Distro::Unknown => {
                    println!(
                        "{}Distro is unknown not installing{}",
                        FgColor!(Red),
                        FgColor!());
                    return;
                }
            }
        }


        match &self.display {
            DspServer::Xorg(xorg_wm, items) => {
                // Append Xorg packages
                for var in *items {
                    pacs.push(var);
                }
                // Get WM packages
                let wmpacs = {
                    match xorg_wm {
                        XorgWM::Awesome(items) => items,
                        XorgWM::Bspwm(items) => items,
                        XorgWM::I3(items) => items,
                    }
                };
                // Append WM packages
                for var in *wmpacs {
                    pacs.push(var);
                }
            }
            DspServer::Wayland(wl_comp, items) => {
                // Append Wayland packages
                for var in *items {
                    pacs.push(var);
                }
                // Get Compositor packages
                let wmpacs = {
                    match wl_comp {
                        WlComp::Hyprland(items) => items,
                        WlComp::River(items) => items,
                        WlComp::Sway(items) => items,
                        WlComp::Niri(items) => items,
                    }
                };
                // Append Compositor packages
                for var in *wmpacs {
                    pacs.push(var);
                }
            }
            DspServer::Desktop => {}
        }

        // Append suffix
        if let Some(val) = suffix {
            for var in val {
                pacs.push(var);
            }
        }

        // Perform installaion
        let installres = Command::new("sudo")
            .args(pacs)
            .spawn();

        let mut install = {
            match installres {
                Ok(x) => x,
                Err(e) => {
                    println!(
                        "{}Subproc creation failed: {e:?}{}", 
                        FgColor!(Red),
                        FgColor!(),
                    );
                    return ;
                },
            }
        };


        // Wait for subproc to finish
        match install.wait() {
            Ok(_) => {}
            Err(e) => {
                println!(
                    "{}Command was not running {e:?}{}",
                    FgColor!(Red),
                    FgColor!(),
                );
            }
        }
    }

    pub fn update(&self) {

        // Get distro
        let dist = {
            match &self.distro {
                Distro::Debian(_distro) => _distro,
                Distro::Fedora(_distro) => _distro,
                Distro::Arch(_distro) => _distro,
                Distro::Unknown => {
                    return;
                }
            }
        };

        // Perform update
        let updateres = Command::new("sudo")
            .args(dist.update)
            .spawn();

        let mut update = {
            match updateres {
                Ok(x) => x,
                Err(e) => {
                    println!(
                        "{}Subproc creation failed: {e:?}{}", 
                        FgColor!(Red),
                        FgColor!(),
                    );
                    return ;
                },
            }
        };


        // Wait for subproc to finish
        match update.wait() {
            Ok(_) => {}
            Err(e) => {
                println!(
                    "{}Command was not running {e:?}{}",
                    FgColor!(Red),
                    FgColor!(),
                );
            }
        }


        // Check if upgrade is neccesary
        match dist.upgrade {
            Some(x) => {
                let updateres = Command::new("sudo")
                    .args(x)
                    .spawn();

                let mut update = {
                    match updateres {
                        Ok(x) => x,
                        Err(e) => {
                            println!(
                                "{}Subproc creation failed: {e:?}{}", 
                                FgColor!(Red),
                                FgColor!(),
                            );
                            return ;
                        },
                    }
                };

                // Wait for subproc to finish
                match update.wait() {
                    Ok(_) => {}
                    Err(e) => {
                        println!(
                            "{}Command was not running {e:?}{}",
                            FgColor!(Red),
                            FgColor!(),
                        );
                    }
                }
            }
            None => {}
        }
    }

    pub fn init(&self) {
        // Initialize for display server
        DspServer::init(&self.display);


        // NOTE: might put this in System as a new parameter

        // switch shell
        println!("{}Going to switch shell to zsh{}", FgColor!(Green), FgColor!());
        let switchres = Command::new("chsh")
            .args(["-s", "/usr/bin/zsh"])
            .spawn();

        let mut switch = {
            match switchres {
                Ok(x) => x,
                Err(e) => {
                    println!(
                        "{}Subproc creation failed: {e:?}{}", 
                        FgColor!(Red),
                        FgColor!(),
                    );
                    return ;
                },
            }
        };

        // Wait for subproc to finish
        match switch.wait() {
            Ok(_) => {}
            Err(e) => {
                println!(
                    "{}Command was not running {e:?}{}",
                    FgColor!(Red),
                    FgColor!(),
                );
            }
        }
    }
}
