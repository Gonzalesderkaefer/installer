/// This enum represents a Foreground color Text printed
/// to the Terminal can have.
#[macro_export]
macro_rules! FgColor {
    (Fg::Black) => { "\x1b[30m" };
    (Fg::Red) => { "\x1b[31m" };
    (Fg::Green) => { "\x1b[32m" };
    (Fg::Yellow) => { "\x1b[33m" };
    (Fg::Blue) => { "\x1b[34m" };
    (Fg::Purple) => { "\x1b[35m" };
    (Fg::Cyan) => { "\x1b[36m" };
    (Fg::White) => { "\x1b[37m" };
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
    (Bg::Black) => { "\x1b[40m" };
    (Bg::Red) => { "\x1b[41m" };
    (Bg::Green) => { "\x1b[42m" };
    (Bg::Yellow) => { "\x1b[43m" };
    (Bg::Blue) => { "\x1b[44m" };
    (Bg::Purple) => { "\x1b[45m" };
    (Bg::Cyan) => { "\x1b[46m" };
    (Bg::White) => { "\x1b[47m" };
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
    (Bold) => {"\x1b[1m"};
    (Faint) => {"\x1b[2m"};
    (Italic) => {"\x1b[3m"};
    (Underline) => {"\x1b[4m"};
    (BlinkS) => {"\x1b[5m"};
    (BlinkF) => {"\x1b[6m"};
    (Conceal) => {"\x1b[8m"};
    (Crossed) => {"\x1b[9m"};
    (BoldOff) => {"\x1b[22m"};
    (ItalicOff) => {"\x1b[23m"};
    (ULineOff) => {"\x1b[24m"};
    (BlinkOff) => {"\x1b[25m"};
    (ConcealOff) => {"\x1b[28m"};
    (CrossedOff) => {"\x1b[29m"};
    () => {"\x1b[0m"};
}
