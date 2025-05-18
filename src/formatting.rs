/// This enum represents a Foreground color Text printed
/// to the Terminal can have.
pub enum FgColors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    Default,
}
#[macro_export]
macro_rules! FgColor {
    (FgColors::Black) => { "\x1b[30m" };
    (FgColors::Red) => { "\x1b[31m" };
    (FgColors::Green) => { "\x1b[32m" };
    (FgColors::Yellow) => { "\x1b[33m" };
    (FgColors::Blue) => { "\x1b[34m" };
    (FgColors::Purple) => { "\x1b[35m" };
    (FgColors::Cyan) => { "\x1b[36m" };
    (FgColors::White) => { "\x1b[37m" };
    () => { "\x1b[39m" };
}


/// This enum represents a Foreground color Text printed
/// to the Terminal can have.
pub enum BgColors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
}

#[macro_export]
macro_rules! BgColor {
    (BgColors::Black) => { "\x1b[40m" };
    (BgColors::Red) => { "\x1b[41m" };
    (BgColors::Green) => { "\x1b[42m" };
    (BgColors::Yellow) => { "\x1b[43m" };
    (BgColors::Blue) => { "\x1b[44m" };
    (BgColors::Purple) => { "\x1b[45m" };
    (BgColors::Cyan) => { "\x1b[46m" };
    (BgColors::White) => { "\x1b[47m" };
    () => { "\x1b[49m" };
}

/// This enum rersesents a Formatting option for text
/// printed to the terminal
pub enum Formatting {
    ResetAll,
    Bold,
    Faint,
    Italic,
    Underline,
    BlinkS,
    BlinkF,
    Conceal,
    Crossed,
    BoldOff,
    ItalicOff,
    ULineOff,
    BlinkOff,
    ConcealOff,
    CrossedOff,
}

#[macro_export]
macro_rules! AnsiFormat {
    (Formatting::Bold) => {"\x1b[1m"};
    (Formatting::Faint) => {"\x1b[2m"};
    (Formatting::Italic) => {"\x1b[3m"};
    (Formatting::Underline) => {"\x1b[4m"};
    (Formatting::BlinkS) => {"\x1b[5m"};
    (Formatting::BlinkF) => {"\x1b[6m"};
    (Formatting::Conceal) => {"\x1b[8m"};
    (Formatting::Crossed) => {"\x1b[9m"};
    (Formatting::BoldOff) => {"\x1b[22m"};
    (Formatting::ItalicOff) => {"\x1b[23m"};
    (Formatting::ULineOff) => {"\x1b[24m"};
    (Formatting::BlinkOff) => {"\x1b[25m"};
    (Formatting::ConcealOff) => {"\x1b[28m"};
    (Formatting::CrossedOff) => {"\x1b[29m"};
    () => {"\x1b[0m"};
}
