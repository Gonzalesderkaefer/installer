/// This file defines all constants that are useful for main.rs
/// If file paths are used make then relative to $HOME

/// The source of all files that will land in ~/.config or similar
pub static CFGSRC: &'static str = "Jazzian/dotfiles/config/";
pub static CFGDEST: &'static str = ".config/";


/// Custom scripts
pub static BINSRC: &'static str = "Jazzian/dotfiles/bin/";
pub static BINDEST: &'static str = ".local/bin/";




/// Predifined file contents
pub static CUSTOMIZED_SH: &'static str = "killshells() { pkill -KILL -u $USER -t tty1 }\n\
    export BROWSER_PREFIX=\"firefox\"\n\
    [ \"$(tty)\" = \"/dev/tty1\" ] && (startx; killshells)\n";

pub static _DEBMDMENU: &'static str = "#!/usr/bin/sh\n\
                  if [ $XDG_SESSION_TYPE = \"wayland\" ]; then\n\
                      exec wofi_dmenu;\n\
                  else\n\
                      exec rofi_dmenu;\n\
                  fi\n";

pub static _DEBMDRUN: &'static str = "#!/usr/bin/sh\n\
                 if [ $XDG_SESSION_TYPE = \"wayland\" ]; then\n\
                     exec wofi_app;\n\
                 else\n\
                     exec rofi_app;\n\
                 fi\n";

pub static MDMENU_CONTENT: &'static str = "#!/usr/bin/sh\n\
                       exec rofi_dmenu\n";

pub static MDRUN_CONTENT: &'static str = "#!/usr/bin/sh\n\
                      exec rofi_app\n";

pub static MYTERM_CONTENT: &'static str = "#!/bin/sh\n\
                       case $XDG_SESSION_TYPE in\n\
                           \"wayland\")\n\
                               exec alacritty -o font.size=12 $@\n\
                               ;;\n\
                           *)\n\
                               exec alacritty -o font.size=12 $@\n\
                               ;;\n\
                       esac\n";

pub static STARTX_CONTENT: &'static str = "x11startup &\nexec i3\n";

pub static X11STARTUP: &'static str = "#!/usr/bin/bash\n";


pub static GTK3_CONFIG: &'static str = "[Settings]\n\
                    gtk-theme-name=Adwaita-dark\n\
                    gtk-icon-theme-name=Papirus-Dark\n\
                    gtk-font-name=JetBrains Mono Light 12\n\
                    gtk-cursor-theme-size=0";



/// Customized files
pub static CUSTOMIZED: &'static [(&str, &str, u32)] =
&[
    // Customized files for window managers/compositors
    (".config/i3/customzied/customzied", "", 0o644),
    (".config/bspwm/customzied/customzied", "", 0o644),
    (".config/awesome/customzied/customzied.lua", "", 0o644),
    (".config/sway/customzied/customzied", "", 0o644),
    (".config/hypr/customzied/customzied", "", 0o644),
    (".config/river/customzied/customzied", "", 0o644),


    // Customized shell files
    (".customized.sh", CUSTOMIZED_SH, 0o644),
    (".customrc", "", 0o644),
    (".customenv",  "export BROWSER_PREFIX=\"firefox\"", 0o644),


    // X11 Startup
    (".local/bin/x11startup", X11STARTUP, 0o755),


    // Xinitrc
    (".xinitrc", STARTX_CONTENT, 0o644),


    // Myterm
    (".local/bin/myterm", MYTERM_CONTENT, 0o755),


    // GTK3
    (".config/gtk-3.0/settings.ini", GTK3_CONFIG, 0o644),


    // Menus
    (".local/bin/mdmenu", MDMENU_CONTENT, 0o755),
    (".local/bin/mdrun", MDRUN_CONTENT, 0o755),
];
