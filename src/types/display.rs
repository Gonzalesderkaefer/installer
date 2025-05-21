
// Used modules
use crate::utils::menu::print_menu;
use super::distro::Distro;

pub enum WlComp {
    Hyprland(&'static[&'static str]),
    River(&'static[&'static str]),
    Sway(&'static[&'static str]),
}

pub enum XorgWM {
    Awesome(&'static[&'static str]),
    Bspwm(&'static[&'static str]),
    I3(&'static[&'static str]),
}

pub enum DspServer {
    Xorg (XorgWM, &'static[&'static str]),
    Wayland (WlComp, &'static[&'static str]),
    Tty,
}


impl DspServer {
    pub fn get_dsp(distro: Distro) -> Self {
        let char = print_menu("Choose a display server",
            &["[X]org", "[W]ayland", "[T]ty"]);

        match char {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }
}
