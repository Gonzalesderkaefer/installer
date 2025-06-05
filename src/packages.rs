pub const DEB_DESKTOP: &'static[&'static str] = &[
    "tmux",
    "zsh",
    "flatpak",
    "git",
    "lf",
    "nala",
    "nnn",
    "fzf",
    "jq",
];



pub const DEB_BASE: &'static[&'static str] = &[
    "tmux",
    "vifm",
    "zsh",
    "evince",
    "flatpak",
    "network-manager",
    "network-manager-gnome",
    "thunar",
    "file-roller",
    "network-manager-openconnect-gnome",
    "eom",
    "network-manager-openconnect",
    "git",
    "lf",
    "fonts-jetbrains-mono",
    "firefox-esr",
    "tlp",
    "alacritty",
    "pipewire",
    "pipewire-alsa",
    "pipewire-pulse",
    "libglib2.0-bin",
    "nala",
    "mpv",
    "papirus-icon-theme",
    "gnome-themes-extra",
    "arc-theme",
    "libnotify-bin",
    "acpi-support",
    "acpid",
    "acpi",
    "linux-cpupower",
    //"cpufrequtils",
    "openssh-server",
    "nnn",
    "fzf",
    "zathura",
    "jq",
];


pub const DEB_XORG: &'static[&'static str] = &[
    "lxappearance",
    "maim",
    "rofi",
    "arandr",
    "rofi",
    "xclip",
    "i3lock",
    "picom",
    "dunst",
    "xinput",
    "xorg",
    "xwallpaper",
    "rxvt-unicode",
    "nitrogen",
];


pub const DEB_WAY: &'static[&'static str] = &[
    "grim",
    "swaylock",
    "wofi",
    "xwayland",
    "wlr-randr",
    "wl-clipboard",
    "swayidle",
    "mako-notifier",
    "slurp",
];


pub const DEB_I3: &'static[&'static str] = &[
  "i3",
  "i3blocks",
];

pub const DEB_BSP: &'static[&'static str] = &[
  "bspwm",
  "sxhkd",
  "polybar",
];

/* Debian awesome packages */
pub const DEB_AWE: &'static[&'static str] = &[
  "awesome",
];


pub const DEB_SWAY: &'static[&'static str] = &[
    "sway",
    "i3blocks",
];



pub const FED_DESKTOP: &'static[&'static str] = &[
  "flatpak",
  "rofi-wayland",
  "tmux",
  "zsh",
  "git",
  "nnn",
  "fzf",
  "jq",
  "neovim",
];



pub const FED_BASE: &'static[&'static str] = &[
  "flatpak",
  "rofi-wayland",
  "vifm",
  "tmux",
  "zsh",
  "pinentry",
  "thunar",
  "mate-polkit",
  "nnn",
  "neovim",
  "alacritty",
  "mpv",
  "firefox",
  "zathura",
  "zathura-pdf-poppler",
  "evince",
  "git",
  "pipewire",
  "pipewire-utils",
  "file-roller",
  "pipewire-pulseaudio",
  "NetworkManager-openconnect-gnome",
  "gsettings-desktop-schemas",
  "papirus-icon-theme",
  "NetworkManager-tui",
  "eom",
  "tlp",
  "libnotify",
  "pipewire-alsa",
  "qalculate-gtk",
  "fzf",
  "jetbrains-mono-fonts",
  "papirus-icon-theme-dark",
  "network-manager-applet",
  "arc-theme",
  "jq",
];



pub const FED_XORG: &'static[&'static str] = &[
  "xclip",
  "@base-x",
  "maim",
  "lxappearance",
  "xinput",
  "arandr",
  "nitrogen",
  "picom",
  "dunst",
  "xclip",
  "i3lock",
  "rxvt-unicode",
  "nitrogen",
];



pub const FED_WAY: &'static[&'static str] = &[
    "grim",
    "swaybg",
    "swayidle",
    "waybar",
    "wofi",
    "wl-clipboard",
    "swaylock",
    "mako",
    "slurp",
];


pub const FED_AWE: &'static[&'static str] = &[
  "awesome",
];

pub const FED_BSP: &'static[&'static str] = &[
  "bspwm",
  "polybar",
  "sxhkd",
];

pub const FED_I3: &'static[&'static str] = &[
  "i3",
  "i3blocks",
];


pub const FED_HYPR: &'static[&'static str] = &[
  "hyprland",
  "waybar",
];

pub const FED_RIV: &'static[&'static str] = &[
  "river",
  "waybar",
];

pub const FED_SWAY: &'static[&'static str] = &[
  "sway",
  "i3blocks",
];


pub const ARCH_DESKTOP: &'static[&'static str] = &[
    "flatpak",
    "fzf",
    "tmux",
    "zsh",
    "nnn",
    "rofi-wayland",
    "neovim",
    "zsh-completions",
    "gcc",
    "less",
    "wget",
    "git",
    "jq",
];


pub const ARCH_BASE: &'static[&'static str] = &[
    "flatpak",
    "fzf",
    "tmux",
    "zsh",
    "nnn",
    "rofi-wayland",
    "networkmanager",
    "thunar",
    "nm-connection-editor",
    "neovim",
    "zathura-pdf-poppler",
    "zathura",
    "evince",
    "webkit2gtk-4.1",
    "networkmanager-openconnect",
    "firefox",
    "lf",
    "tlp",
    "alacritty",
    "pipewire",
    "mpv",
    "gsettings-desktop-schemas",
    "eom",
    "network-manager-applet",
    "openconnect",
    "lxappearance",
    "file-roller",
    "papirus-icon-theme",
    "gnome-themes-extra",
    "ttf-jetbrains-mono-nerd",
    "ttf-jetbrains-mono",
    "gcr",
    "bash-completion",
    "zsh-completions",
    "gcc",
    "less",
    "wget",
    "pipewire-pulse",
    "pipewire-alsa",
    "wireplumber",
    "cpupower",
    "git",
    "jq",
];



pub const ARCH_XORG: &'static[&'static str] = &[
    "xorg",
    "lxappearance",
    "xwallpaper",
    "maim",
    "picom",
    "xorg-xinput",
    "xorg-xinit",
    "xclip",
    "i3lock",
    "rxvt-unicode",
    "nitrogen",
];


pub const ARCH_WAY: &'static[&'static str] = &[
    "grim",
    "wofi",
    "swaybg",
    "waybar",
    "swayidle",
    "swaylock",
    "wl-clipboard",
    "mako",
    "slurp",
];

pub const ARCH_AWE: &'static[&'static str] = &[
  "awesome",
];

pub const ARCH_BSP: &'static[&'static str] = &[
  "bspwm",
  "polybar",
  "sxhkd",
];

pub const ARCH_I3: &'static[&'static str] = &[
  "i3",
  "i3blocks",
];


pub const ARCH_HYPR: &'static[&'static str] = &[
  "hyprland",
  "waybar",
];

pub const ARCH_RIV: &'static[&'static str] = &[
  "river",
  "waybar",
];

pub const ARCH_SWAY: &'static[&'static str] = &[
  "sway",
  "i3blocks",
];
