use crate::FgColor;

// Used modules
use super::{display::DspServer, display::WlComp, display::XorgWM, distro::Distro, transfer::Transfer, };
use std::{io, process::Command, };

pub struct System<'a> {
    distro: Distro<'a>,
    display: DspServer,
    transfer: Transfer
}

impl<'a> System <'a> {
    pub fn get() -> io::Result<Self> {
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
            display: dsp,
            transfer: tr,
        });
    }

    pub fn install(&self) {
        // This also stores the install cmd
        let mut pacs: Vec<&str> = Vec::new();

        // Get reference to suffix
        let mut suffix: Option<&'a [&'a str]> = None;

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
            Distro::Unknown => todo!(), // TODO: Figure out what to do here.
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
                    }
                };
                // Append Compositor packages
                for var in *wmpacs {
                    pacs.push(var);
                }
            }
            DspServer::Desktop => todo!(), // TODO: Figure out what to do here.
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
}
