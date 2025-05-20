



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
    Wayland (XorgWM, &'static[&'static str]),
    Tty (&'static[&'static str]),
}
